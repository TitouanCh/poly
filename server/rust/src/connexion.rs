use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, sync::broadcast::{self, Sender, Receiver}};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start listening to port 3000
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    // Broadcast
    let (tx, _rx) = broadcast::channel::<String>(10);

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
    print!("Received a connection from {}", remote_ip);

    let mut buffer = [0u8; 1024];

    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                let bytes_read = result?;
                if bytes_read == 0 {
                    break;
                }
                let received = String::from_utf8(buffer[0..bytes_read].to_vec())?;
                print!("They sent: {}", received);
                tx.send(received.clone())?;
            }

            result = rx.recv() => {
                socket.write_all(result?.as_bytes()).await?;
            }
        }
    }
    
    /*
    

    
    */
    //socket.write_all(&buffer[..bytes_read]).await?;

    Ok(())
}