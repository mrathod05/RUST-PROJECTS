use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, stdout, Write},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

const LISTENER_ADD: &str = "127.0.0.1:8080";

struct Peer {
    connections: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
}

impl Peer {
    fn new() -> Self {
        Peer {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn listen(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(LISTENER_ADD).await?;
        println!("Listening for peers on {}", LISTENER_ADD);

        loop {
            let (socket, socket_address) = listener.accept().await?;
            println!("New peer connected {}", { socket_address });

            let socket = Arc::new(Mutex::new(socket));
            self.connections
                .lock()
                .await
                .insert(socket_address, socket.clone());

            let connection = self.connections.clone();
            let socket_clone = socket.clone();

            tokio::spawn(async move {
                let mut buffer = [0; 1024];

                loop {
                    let mut socket = socket_clone.lock().await;
                    match socket.read(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            let message = String::from_utf8_lossy(&buffer[..n]);
                            println!("{} {}", socket_address, message);
                        }
                        Ok(_) => {
                            println!("Peer {} disconnected", socket_address);
                            connection.lock().await.remove(&socket_address);
                            break;
                        }
                        Err(e) => {
                            eprintln!("Error reading from {} {}", socket_address, e);
                            connection.lock().await.remove(&socket_address);
                            break;
                        }
                    }
                }
            });
        }
    }

    async fn connect(&self, address: &str) -> Result<(), Box<dyn Error>> {
        let socket = TcpStream::connect(LISTENER_ADD).await?;
        println!("Connected to peer at{address}");

        let socket_guard = Arc::new(Mutex::new(socket));
        self.connections
            .lock()
            .await
            .insert(socket_guard.lock().await.peer_addr()?, socket_guard.clone());

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                let mut socket = socket_guard.lock().await;
                match socket.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        println!("Peer {message}");
                    }
                    Ok(_) => {
                        println!("Peer disconnected.");
                        break;
                    }
                    Err(e) => {
                        println!("Error reading from peer. {e}");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn broadcast(&self, message: String) {
        let peers = self.connections.lock().await;

        for (add, stream) in peers.iter() {
            let stream = stream.clone();

            let _ = stream.lock().await.write_all(message.as_bytes()).await;
            println!("Send to {add} {message}");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let peer = Arc::new(Peer::new());

    let peer_clone = peer.clone();

    tokio::spawn(async move {
        let _ = peer_clone.listen().await;
    });

    loop {
        print!("Enter peer address to connect (or type 'msg' to send message) ");
        stdout().flush()?;

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);

        let input = input.trim();

        if input == "msg" {
            print!("Enter message");
            stdout().flush()?;

            let mut input = String::new();
            let _ = stdin().read_line(&mut input);

            peer.broadcast(input.trim().to_string()).await;
        } else {
            if let Err(e) = peer.connect(input).await {
                eprintln!("Failed to connect {e}");
            }
        }
    }
}
