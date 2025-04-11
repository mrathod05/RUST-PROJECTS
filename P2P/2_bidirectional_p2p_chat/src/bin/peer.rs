use std::{collections::HashMap, error::Error, net::SocketAddr, sync::Arc};

use clap::{arg, Parser};
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, required = true)]
    name: String,
    #[arg(long, required = true)]
    listening_port: u16,
}

const LOCAL_ADDR: &str = "127.0.0.1";

async fn read_message(reader: &mut BufReader<tokio::io::Stdin>) -> Option<String> {
    let mut buffer = String::new();

    match reader.read_line(&mut buffer).await {
        Ok(_) => {
            let message = buffer.trim().to_string();

            if !message.is_empty() {
                Some(message)
            } else {
                None
            }
        }
        Err(_) => {
            eprintln!("Failed to read the message");
            None
        }
    }
}

async fn connect_to_peer(
    reader: &mut BufReader<tokio::io::Stdin>,
    peers: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
) -> Result<(), Box<dyn Error>> {
    println!("Please enter the peer address");

    let addr = match read_message(reader).await {
        Some(a) => a,
        None => {
            eprintln!("No input received");
            return Err("No address provided".into());
        }
    };

    let socket_addr = format!("{LOCAL_ADDR}:{}", addr);
    let address = match socket_addr.parse::<SocketAddr>() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Invalid peer address {e}");
            return Err(Box::new(e));
        }
    };

    let stream = TcpStream::connect(address).await?;
    let stream_arc = Arc::new(Mutex::new(stream));
    {
        let mut peers_guard = peers.lock().await;
        peers_guard.insert(address, stream_arc.clone());
    }
    // Store the peer stream

    // Spawn a new async task to handle communication with the peer
    // tokio::spawn(handle_peer(stream_arc, address, peers.clone()));

    Ok(())
}

async fn run_server(args: Cli) -> Result<(), Box<dyn Error>> {
    let list_addr = format!("{LOCAL_ADDR}:{}", args.listening_port);
    let listen = TcpListener::bind(list_addr).await?;
    let mut reader = BufReader::new(stdin());
    let name = args.name;
    let peers = Arc::new(Mutex::new(
        HashMap::<SocketAddr, Arc<Mutex<TcpStream>>>::new(),
    ));

    println!("Server listening on {}", args.listening_port);

    loop {
        println!("Type msg to send a message, con to connect to a peer, or exit to quit");

        tokio::select! {
            res = listen.accept()=> {
                match  res {
                    Ok((stream,address))=>{

                        println!("New peer Connected {address}");

                        let stream_guard = Arc::new(Mutex::new(stream));
                            // {
                            //     let mut  peers_guard = peers.lock().await;
                            //     peers_guard.insert(address,stream_guard.clone());
                            //     drop(peers_guard);
                            // }
                        tokio::spawn(handle_peer(name.clone(),stream_guard.clone(),address,peers.clone()));
                    },
                    Err(e)=> eprintln!("Failed to accept connection {e}")


                }

            }

            Some(choice)= read_message(&mut reader)=>{
                match choice.as_str() {
                    "msg" => {
                        println!("Enter message: ");
                       if let Some(message) = read_message(&mut reader).await{
                            let _ = send_messages_to_peers(peers.clone(), message).await;
                       }

                    }
                    "con" => {
                        let _ = connect_to_peer(&mut reader,peers.clone()).await;
                    }
                    "exit" => {
                        println!("🚪 Exiting chat...");
                        break;
                    }
                    _ => {
                        println!("⚠️ Invalid choice, try again.");
                    }
                }
            }
        }
    }

    Ok(())
}

async fn send_messages_to_peers(
    peers: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
    message: String,
) {
    let peers = peers.lock().await;
    let mut peers_clone = peers.clone();
    drop(peers);

    for (address, stream) in peers_clone.iter_mut() {
        let mut stream_guard = stream.lock().await;

        let msg = format!("{}\n", message);

        match stream_guard.write_all(msg.as_bytes()).await {
            Ok(_) => {
                if let Err(e) = stream_guard.flush().await {
                    eprintln!("⚠️ Failed to flush message to {}: {}", address, e);
                } else {
                    println!("✅ You: {}", address);
                }
            }
            Err(e) => {
                eprintln!("⚠️ Failed to send message to {}: {}", address, e);
            }
        }
    }
}

async fn handle_peer(
    name: String,
    stream: Arc<Mutex<TcpStream>>,
    address: SocketAddr,
    peers: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
) {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let mut locked_stream = stream.lock().await;
        let mut reader = BufReader::new(&mut *locked_stream); // Lock & use stream

        match reader.read_line(&mut buffer).await {
            Ok(0) => {
                println!("🔌 Peer disconnected: {}", address);
                peers.lock().await.remove(&address);
                break;
            }
            Ok(_) => {
                let msg = buffer.trim().to_string();

                println!("📨 {}: {}", name, msg);
            }
            Err(e) => {
                eprintln!("⚠️ Failed to read from {}: {}", address, e);
                break;
            }
        }

        drop(locked_stream);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let _ = run_server(args).await?;

    Ok(())
}
