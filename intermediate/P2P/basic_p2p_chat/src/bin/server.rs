use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const ADDRESS: &str = "127.0.0.1:8080";

const SERVER_ACKNOWLEDGEMENT: &str = "Message received";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Server is listening on {}", ADDRESS);

    loop {
        let (mut socket, socket_address) = listener.accept().await?;
        println!("New client {} is connected", socket_address);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                match socket.read(&mut buffer).await {
                    Ok(n) if n > 0 => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        println!("From:({}) {}", socket_address, message);

                        // Message back
                        if let Err(e) = socket.write_all(SERVER_ACKNOWLEDGEMENT.as_bytes()).await {
                            eprintln!("Failed to send response {}", e);
                        }
                    }
                    Ok(_) => {
                        println!("Client {} disconnected", socket_address);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Reading from client {} {}", socket_address, e);
                        break;
                    }
                };
            }
        });
    }
}
