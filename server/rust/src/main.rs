use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, sync::broadcast::{self, Sender, Receiver}};
use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // -- Setup game state
    // Only one game at a time for now

    // Game state values: single global values for the game
    let mut game_state_values: HashMap<String, i32> = HashMap::new();
    game_state_values.insert("phase".to_string(), 0);
    let game_state_values: Arc<RwLock<HashMap<String, i32>>> = Arc::new(RwLock::new(game_state_values));

    // Game state hashmap: for entities (cities, units, etc...)
    let mut game_state_cities: HashMap<String, Vec<u32>> = HashMap::new();
    game_state_cities.insert("Test".to_string(), vec![800, 800]);
    let game_state_cities: Arc<RwLock<HashMap<String, Vec<u32>>>> =  Arc::new(RwLock::new(game_state_cities));


    // Start listening to port 3000
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await?;
    // Broadcast
    let (tx, _rx) = broadcast::channel::<String>(10);

    info!("Listening");

    loop {
        // Get copy of sender
        let tx = tx.clone();
        // Get unique receiver for each client
        let rx = tx.subscribe();

        // Get game state
        let game_state_values = Arc::clone(&game_state_values);
        let game_state_cities = Arc::clone(&game_state_cities);

        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket, &tx, rx, &game_state_values, &game_state_cities).await;
        });
    }

    //print!("{:?}", buffer);
}

async fn handle_client(mut socket: TcpStream, tx: &Sender<String>, mut rx: Receiver<String>,
    game_state_values: &Arc<RwLock<HashMap<String, i32>>>,
    game_state_cities: &Arc<RwLock<HashMap<String, Vec<u32>>>>
) -> Result<(), Box<dyn Error>> {
    
    let remote_ip = socket.peer_addr()?.ip();
    info!("Received a connection from {}", remote_ip);

    //let game_state_inner = game_state_values.clone();

    let mut buffer = [0u8; 1024];

    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                let bytes_read = result?;
                if bytes_read == 0 {
                    break;
                }
                // Receive data
                let received = buffer[0..bytes_read].to_vec();

                // Print data
                match String::from_utf8(received[0..3].to_vec()) {
                    Ok(t) => info!("They sent: {} | {:?}", t, received[3..].to_vec()),
                    Err(_e) => info!("They sent: {:?}", received.clone())
                }
                
                handle_message(received, tx, &game_state_values, &game_state_cities)?;
                
                // Send it to other clients
                //tx.send(received.clone())?;

                // Reset buffer
                buffer = [0u8; 1024];
            }

            result = rx.recv() => {
                let sendback = result? + "|end|";
                socket.write_all(sendback.as_bytes().clone()).await?;
                info!("We sent back: {:?} or {}", sendback, String::from_utf8(sendback.as_bytes().to_vec()).unwrap());
            }
        }
    }

    Ok(())
}

fn handle_message(message: Vec<u8>, tx: &Sender<String>,
    game_state_values: &Arc<RwLock<HashMap<String, i32>>>, game_state_cities: &Arc<RwLock<HashMap<String, Vec<u32>>>>) -> Result<(), Box<dyn Error>> {

    // Chat message
    if message[0..3] == [109, 115, 103] {
        info!("Detected as chat message");
        let mut to_send_back = vec![109];
        to_send_back.extend(message[3..].to_vec());
        tx.send(String::from_utf8(to_send_back)?)?;
    }

    // Start game
    else if message[0..3] == [115, 116, 97] {
        info!("Detected as start game message");
        let to_send_back = vec![115];

        // Modify game state
        let mut inner_game_state_values = game_state_values.write().unwrap();
        *inner_game_state_values.entry("phase".to_string()).or_insert(1) = 1;

        // TODO: Send back game state
        tx.send(String::from_utf8(to_send_back)?)?;
    }

    // Place city
    else if message[0..3] == [112, 108, 99] {
        info!("Placing city detected");

        // Modify game state
        {
            let mut inner_game_state_cities = game_state_cities.write().unwrap();
            inner_game_state_cities.insert("Orsay".to_string(), vec![as_u32(message[3..7].to_vec()), as_u32(message[7..11].to_vec())]);
        }

        // Send back game state
        send_back_game_state(tx, game_state_cities)?;
    }

    Ok(())
}

fn send_back_game_state(tx: &Sender<String>, game_state_cities: &Arc<RwLock<HashMap<String, Vec<u32>>>>) -> Result<(), Box<dyn Error>> {
    let inner_game_state_cities = game_state_cities.read().unwrap().clone();

    for (city_id, city_data) in inner_game_state_cities {
        tx.send(city_to_string(city_id, city_data))?;
    }

    Ok(())
}

fn city_to_string(city_id : String, city_data : Vec<u32>) -> String {
    let mut string = "c, ".to_string() + &city_id;

    for data in city_data.iter() {
        string.push_str(&(", ".to_string() + &data.to_string()));
    }

    info!("- City to string: {}", string);

    string
}

fn as_u32(array: Vec<u8>) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}