use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, sync::broadcast::{self, Sender, Receiver}};
use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // Setup game state
    let mut game_state = HashMap::new();

    game_state.insert("phase".to_string(), 0);

    // Only one game at a time for now
    let game_state = Arc::new(RwLock::new(game_state));

    // Start listening to port 3000
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    // Broadcast
    let (tx, _rx) = broadcast::channel::<String>(10);

    info!("Listening");

    loop {
        // Get copy of sender
        let tx = tx.clone();
        // Get unique receiver for each client
        let rx = tx.subscribe();

        // Get game state
        let game_state = Arc::clone(&game_state);

        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket, &tx, rx, &game_state).await;
        });
    }

    //print!("{:?}", buffer);
}

async fn handle_client(mut socket: TcpStream, tx: &Sender<String>, mut rx: Receiver<String>, game_state: &Arc<RwLock<HashMap<String, i32>>>) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    info!("Received a connection from {}", remote_ip);

    let game_state_inner = game_state.clone();

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
                info!("They sent: {}", String::from_utf8(received.clone()).unwrap());
                
                handle_message(received, tx, &game_state_inner)?;
                
                // Send it to other clients
                //tx.send(received.clone())?;

                // Reset buffer
                buffer = [0u8; 1024];
            }

            result = rx.recv() => {
                let sendback = result?;
                socket.write_all(sendback.as_bytes().clone()).await?;
                info!("We sent back: {:?} or {}", sendback, String::from_utf8(sendback.as_bytes().to_vec()).unwrap());
            }
        }
    }

    Ok(())
}

fn handle_message(message: Vec<u8>, tx: &Sender<String>, game_state: &Arc<RwLock<HashMap<String, i32>>>) -> Result<(), Box<dyn Error>> {

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
        let mut to_send_back = vec![109];

        // Modify game state
        let mut inner_game_state = game_state.write().unwrap();
        *inner_game_state.entry("phase".to_string()).or_insert(1) = 1;

        // TODO: Send back game state
        to_send_back.extend(vec![message[0]]);
        tx.send(String::from_utf8(to_send_back)?)?;
    }

    Ok(())
}