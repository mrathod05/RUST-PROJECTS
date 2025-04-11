use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(ADDRESS).await?;
    println!("Connected to the server! Type 'exit' to quit.");

    loop {
        let mut input = String::new();
        print!("You: ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut input);

        let message = input.trim().to_string();

        if message.to_lowercase() == "exit" {
            println!("🚪 Exiting chat...");
            break;
        }

        stream.write_all(message.as_bytes()).await?;

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;

        println!("Server replied:{}\n", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}
