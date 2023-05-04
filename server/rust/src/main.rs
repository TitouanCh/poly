use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, sync::broadcast::{self, Sender, Receiver}};
use std::error::Error;
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

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

        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket, &tx, rx).await;
        });
    }

    //print!("{:?}", buffer);
}

async fn handle_client(mut socket: TcpStream, tx: &Sender<String>, mut rx: Receiver<String>) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    info!("Received a connection from {}", remote_ip);

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
                
                handle_message(received, tx)?;
                
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

fn handle_message(message: Vec<u8>, tx: &Sender<String>) -> Result<(), Box<dyn Error>> {
    if message[0..3] == [109, 115, 103] {
        info!("Detected as chat message");
        let mut to_send_back = vec![109];
        to_send_back.extend(message[3..].to_vec());
        tx.send(String::from_utf8(to_send_back)?)?;
    }

    Ok(())
}