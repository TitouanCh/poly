use async_trait::async_trait;
use tokio::{sync::mpsc, time};
use std::{collections::HashMap, time::Duration};
use bimap::BiMap;
use log::info;

use crate::{link::{
    userinfo::UserInfo,
    user::{User, self},
    link::Link,
    linkable::Linkable,
    message::Message,
    satellite::Satellite
}, 
utilities::as_u32,
battle_engine::{battle_engine::BattleEngine, self}
};

pub struct Entity {
    position: Vec<u32>,
    owner: u32,
    position_matters: bool
}

impl Entity {
    fn to_bytes(&self) {
    
    }
}

pub struct PlayerState {
    ready: bool,
    connected: bool,
    in_game_id: u32,
    spectator: bool,
}

impl PlayerState {
    fn new(in_game_id: u32, spectator: bool) -> Self {
        PlayerState { ready: false, connected: true, in_game_id, spectator}
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.ready as u8, self.connected as u8];
        bytes.extend(self.in_game_id.to_le_bytes().to_vec());
        bytes.push(self.spectator as u8);
        bytes
    }
}

pub struct GameState {
    phase: u32,
    number_of_cities: u32,
    maximum_players: u32,
    number_of_players: u32
}

impl GameState {
    pub fn to_string(&self) -> String {
        let mut a = "".to_string();
        for b in [self.phase, self.number_of_cities, self.maximum_players, self.number_of_players] {
            a += &b.to_string();
            a += " ";
        }
        a
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for b in [self.phase, self.number_of_cities, self.maximum_players, self.number_of_players] {
            bytes.extend(&b.to_le_bytes());
        }
        bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let phase = as_u32(bytes[0..4].to_vec());
        let number_of_cities = as_u32(bytes[4..8].to_vec());
        let maximum_players = as_u32(bytes[8..12].to_vec());
        let number_of_players = as_u32(bytes[12..16].to_vec());
        GameState { phase, number_of_cities, maximum_players, number_of_players }
    }
}

pub struct Game {
    user: User,
    message_history: Vec<Message>,

    connected: HashMap<UserInfo, mpsc::Sender<Message>>,
    connected_link_senders: HashMap<UserInfo, mpsc::Sender<Link>>,
    game_handler_info: Option<UserInfo>,

    player_states: HashMap<UserInfo, PlayerState>,

    game_state: GameState,
    entities: HashMap<String, Vec<Entity>>,
    battle_engine: Option<BattleEngine>,

    username_ids: BiMap<u32, UserInfo>,

    update_interval: time::Interval,
    turn_time: time::Duration,
    turn_time_current: time::Duration
}

#[async_trait]
impl Linkable for Game {
    fn id(&self) -> u32 { self.user.info.id.clone() }
    fn info(&self) -> UserInfo { self.user.info.clone() }
    fn message_sender(&self) -> mpsc::Sender<Message> { self.user.message.sender.clone() }
    fn link_sender(&self) -> mpsc::Sender<Link> { self.user.link.sender.clone() }
    fn mut_message(&mut self) -> &mut Satellite<Message> { &mut self.user.message}
    fn mut_link(&mut self) -> &mut Satellite<Link> { &mut self.user.link }
    fn mut_message_and_link(&mut self) -> (&mut Satellite<Message>, &mut Satellite<Link>) { (&mut self.user.message, &mut self.user.link) }
    fn mut_connected(&mut self) -> &mut HashMap<UserInfo,mpsc::Sender<Message> > { &mut self.connected }
    
    fn add_to_history(&mut self, message:Message) {
        self.message_history.push(message);
    }

    async fn handle(&mut self) -> bool {
        tokio::select! {
            received_message = self.user.message.receiver.recv() => {
                info!("{}: Received message", self.info().to_string());
                let message = received_message.unwrap();
                self.add_to_history(message.clone());

                 // Disconnected
                 if message.bytes[0] == 60 {
                    self.unlink(message.info());

                    // Set player state to disconnected if ingame
                    if message.info().what == "peer" {
                        let player_state = self.player_states.get_mut(&message.info());
                        match player_state {
                            Some(state) => { state.connected = false; }
                            None => {} 
                        }
                    }
                    return  true;
                }

                // Received message
                self.handle_message(message).await;
                
                true
            }

            received_link = self.user.link.receiver.recv() => {
                let link = received_link.unwrap(); 
                info!("{}: Received new link: {}", self.info().to_string(), link.info.to_string());
                self.add_linked(link).await;
                true
            }

            _ = self.update_interval.tick() => {
                self.send_player_states_to_all().await;
                self.send_info_to_gh().await;
                // If game has started, tick game
                if self.battle_engine.is_some() {self.battle_tick(self.update_interval.period()).await}
                true
            }
        }
    }

    async fn add_linked(&mut self, link: Link) {
        let self_link = self.as_link(true);
        
        // Save player links (for some reason)
        self.connected_link_senders.insert(link.info.clone(), link.connexion_sendback.clone());

        self.connected.insert(link.info.clone(), link.message_sendback);
        
        if !link.dont_respond {
            link.connexion_sendback.send(self_link).await.unwrap();
        }

        // Save game_handle info
        if link.info.what == "game_handler" {
            self.game_handler_info = Some(link.info);
        }
    }

    async fn handle_message(&mut self, message: Message) {
        if message.info.what == "peer" {
            // joi: join game
            if message.bytes[0..3] == vec![106, 111, 105] {
                info!("{}: {} is joining", self.info().to_string(), message.info().to_string());
                // Try to add the player
                self.add_player(message.info());
            }

            // rea: ready
            if message.bytes[0..3] == [114, 101, 97] {
                let mut player_state = self.player_states.get_mut(&message.info).unwrap();
                player_state.ready = !player_state.ready;
                info!("{}: {} ready: {}", self.user.info.to_string(), message.info().to_string(), player_state.ready.to_string());
            }

            // lau: try to launch
            if message.bytes[0..3] == [108, 97, 117] {
                let mut all_ready = true;
                for (_info, player_state) in &self.player_states {
                    if player_state.ready == false && player_state.spectator == false {
                        all_ready = false;    
                        break;
                    }
                }
                if all_ready {
                    self.launch().await;
                } else {
                    info!("{}: tried to launch but not enought players are ready", self.info().to_string());
                }
            }

            // lea: try to leave
            if message.bytes[0..3] == [108, 101, 97] {
                self.remove_player(message.info());
            }
        }
    }
}

impl Game {
    pub fn new(id: u32, name: Option<String>, maximum_players: u32) -> (Self, mpsc::Sender<Link>) {
        let (user, link_sender) = User::new(id, name.unwrap_or("Game".to_string()), "game".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        let connected_link_senders = HashMap::new();
        
        let player_states = HashMap::new();

        // Set starting game state
        let game_state = GameState {
            phase: 0,
            number_of_cities: 0,
            maximum_players: maximum_players,
            number_of_players: 0
        };

        let battle_engine = None;
        let entities = HashMap::new();
        let username_ids = BiMap::new();
        
        let update_interval = time::interval(time::Duration::from_secs(1));
        let turn_time = time::Duration::from_secs(10);
        let turn_time_current = time::Duration::from_secs(0);

        (Game {user, message_history, connected, connected_link_senders, game_handler_info: None, player_states, game_state, battle_engine, entities, username_ids, update_interval, turn_time, turn_time_current}, link_sender)
    }

    fn add_player(&mut self, user: UserInfo) {
        // Check if the player was previously in the game
        match self.username_ids.get_by_right(&user) {
            // If he never was in the game
            None => {
                // Check if there are any spots left for the player and the game hasn't started
                if self.game_state.number_of_players < self.game_state.maximum_players && self.game_state.phase == 0 {
                    let new_user_id = self.game_state.number_of_players;
                    
                    self.username_ids.insert(new_user_id, user.clone());
                    self.player_states.insert(user.clone(), PlayerState::new(new_user_id, false));
                    info!("{}: {} was added to the game", self.info().to_string(), user.to_string());

                    self.game_state.number_of_players += 1;
                } else {
                    info!("{}: tried adding {} but there is not enought room left in the game", self.info().to_string(), user.to_string());
                }
            }

            // If he was previously in the game
            Some(_id) => {
                info!("{}: {} rejoined", self.info().to_string(), user.to_string());
            }
        };
    }

    fn remove_player(&mut self, user: UserInfo) {
        // This can only occur if the game hasn't already started ie: phase 0
        if self.game_state.phase == 0 {
            // Check if the user is in the game
            match self.username_ids.get_by_right(&user) {
                Some(_id) => {
                    self.username_ids.remove_by_right(&user);
                    self.player_states.remove(&user);
                    info!("{}: {} was removed from the game", self.info().to_string(), user.to_string());

                    self.game_state.number_of_players -= 1;
                }
                None => {
                    info!("{}: tried to remove {} but he isn't in the game", self.info().to_string(), user.to_string());
                }
            }
        } else {
            info!("{}: tried to remove {} but the game has already started", self.info().to_string(), user.to_string());
        }
    }

    async fn send_entities(&self, user: UserInfo, entity_type: String, identifier: u8) {
        match self.entities.get(&entity_type) {
            Some(list) => {
                let mut bytes = vec![identifier];
                for entity in list.into_iter() {
                    //bytes.push(entity.to_bytes());
                }
            }
            None => {}
        }
    }

    async fn send_player_states_to_all(&self) {
        info!("{}: sending back player states", self.info().to_string());
        for (_user, sender) in &self.connected {
            self.send_player_state(sender.clone()).await;
        }
    }

    async fn send_player_state(&self, tx: mpsc::Sender<Message>) { 
        for (from_user, player_state) in &self.player_states {
            let mut bytes = vec![108];
            bytes.extend(player_state.to_bytes());
            let _ = tx.send(Message { info: from_user.clone(), bytes }).await;
        }
    }

    async fn send_battle_state_to_all(&self) {
        info!("{}: sending back battle state", self.info().to_string());
        for (_user, sender) in &self.connected {
            self.send_battle_state(sender.clone()).await;
        }
    }

    async fn send_battle_state(&self, tx: mpsc::Sender<Message>) {
        match &self.battle_engine {
            Some(battle_engine) => {
                let mut list_of_bytes = battle_engine.units_as_bytes(0);
                for bytes in &mut list_of_bytes {
                    bytes.insert(0, 49);
                    let _ = tx.send(Message { info: self.info(), bytes: bytes.to_vec()}).await; 
                }
            }
            None => {info!("{}: Could not send game state because there is no battle engine", self.info().to_string());}
        }
    }

    async fn send_info_to_gh(&mut self) {
        match &self.game_handler_info {
            Some(info) => {
                let mut bytes = vec![105];
                bytes.extend(self.game_state.to_bytes());
                self.connected.get(&info).unwrap().send(Message { info: self.info(), bytes }).await.unwrap();
            }
            None => {
                info!("{}: Not connected to game handler", self.info().to_string());
            }
        }
    }

    async fn launch(&mut self) {
        info!("{}: launching!!", self.info().to_string());
        self.game_state.phase = 1;
        for (user, tx) in &self.connected {
            if user.what == "peer" {
                let _ = tx.send(Message { info: self.info(), bytes: vec![115] }).await;
            }
        }

        self.setup_battle_engine();
    }

    fn setup_battle_engine(&mut self) {
        info!("{}: Starting battle engine", self.info().to_string());
        self.battle_engine = Some(BattleEngine::new());
        self.battle_engine.as_mut().unwrap().ready();
    }

    async fn battle_tick(&mut self, time: time::Duration) {
        self.turn_time_current += time;

        if self.turn_time_current > self.turn_time {
            self.turn_time_current = Duration::ZERO;
            self.battle_engine.as_mut().unwrap().process_by_intervall(60.0, 0.1);
            self.send_battle_state_to_all().await;
        }
    }

}

