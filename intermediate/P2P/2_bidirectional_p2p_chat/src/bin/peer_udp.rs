use std::{collections::HashMap, error::Error, net::SocketAddr, sync::Arc, time::Duration};

use clap::{arg, Parser};
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, UdpSocket},
    sync::Mutex,
    time::interval,
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, required = true)]
    name: String,
    #[arg(long, required = true)]
    listening_port: u16,
}

const BROADCAST_PORT: u16 = 9999;

type PeerMap = Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>;

async fn start_peer_discovery(
    listening_port: u16,
    peers: PeerMap,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let upd_socket = UdpSocket::bind(("0.0.0.0", BROADCAST_PORT)).await?;
    upd_socket.set_broadcast(true)?;

    let format_address = format!("127.0.0.1:{}", listening_port);
    let local_address: SocketAddr = format_address.parse()?;

    let upd_socket_arc = Arc::new(upd_socket);
    let upd_socket_clone = upd_socket_arc.clone();

    let peers_clone = peers.clone();

    tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            let (size, address) = match upd_socket_clone.recv_from(&mut buf).await {
                Ok(details) => details,
                Err(_) => continue,
            };

            if address == local_address {
                continue;
            }

            let peer_info = String::from_utf8_lossy(&buf[..size]);

            println!("Discover Peer: {} {}", peer_info, address);

            if let Some(port_str) = peer_info.strip_prefix("HELLO: ") {
                if let Ok(port) = port_str.trim().parse::<u16>() {
                    // Create proper address with the advertised port, not the UDP source port
                    let peer_addr = SocketAddr::new(address.ip(), port);

                    // Don't connect to yourself
                    if port == listening_port {
                        continue;
                    }

                    let peers_guard = peers_clone.lock().await;
                    if !peers_guard.contains_key(&peer_addr) {
                        drop(peers_guard);
                        if let Err(e) = connect_to_peer(peer_addr, peers.clone()).await {
                            eprintln!("Failed to connect to peer {}: {}", peer_addr, e);
                        }
                    }
                }
            }
        }
    });

    let mut interval = interval(Duration::from_secs(3));

    loop {
        interval.tick().await;
        let msg = format!("HELLO: {}", listening_port);
        upd_socket_arc
            .send_to(msg.as_bytes(), ("255.255.255.255", BROADCAST_PORT))
            .await?;
    }
}

async fn connect_to_peer(address: SocketAddr, peers: PeerMap) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect(address).await?;
    let stream_arc = Arc::new(Mutex::new(stream));
    let mut peers_guard = peers.lock().await;

    peers_guard.insert(address, stream_arc.clone());
    drop(peers_guard);

    tokio::spawn(handle_peer(stream_arc, address, peers.clone()));

    Ok(())
}

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

async fn run_server(args: Cli) -> Result<(), Box<dyn Error>> {
    let listen_addr = format!("0.0.0.0:{}", args.listening_port).parse::<SocketAddr>()?;
    let listener = TcpListener::bind(listen_addr).await?;

    let peers = Arc::new(Mutex::new(
        HashMap::<SocketAddr, Arc<Mutex<TcpStream>>>::new(),
    ));

    println!("Server listening on {}", args.listening_port);

    let peer_clone = peers.clone();
    tokio::spawn(async move {
        if let Err(e) = start_peer_discovery(args.listening_port, peer_clone).await {
            eprintln!("❌ Peer discovery failed: {}", e);
        }
    });

    let mut reader = BufReader::new(stdin());

    loop {
        println!("Type `msg` to send a message, `con` to connect to a peer, or `exit` to quit");

        tokio::select! {
            res = listener.accept()=> {
                match  res {
                    Ok((stream,address))=>{

                        println!("New peer Connected {address}");

                        let stream_guard = Arc::new(Mutex::new(stream));
                        let mut  peers_guard = peers.lock().await;
                        peers_guard.insert(address,stream_guard.clone());
                        drop(peers_guard);
                        tokio::spawn(handle_peer(stream_guard.clone(),address,peers.clone()));
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

                    },

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

async fn send_messages_to_peers(peers: PeerMap, message: String) {
    println!("Sending.... {message}");
    let peers = peers.lock().await;
    let mut peers_clone = peers.clone();
    drop(peers);

    println!("peers_clone: {:?}", peers_clone);

    for (address, stream) in peers_clone.iter_mut() {
        println!("Address: {} stream {:?}", address, stream);
        let mut stream_guard = stream.lock().await;

        let msg = format!("{}\n", message); // Ensure newline
        println!("msg: {msg}");

        match stream_guard.write_all(msg.as_bytes()).await {
            Ok(_) => {
                if let Err(e) = stream_guard.flush().await {
                    eprintln!("⚠️ Failed to flush message to {}: {}", address, e);
                } else {
                    println!("✅ Message sent to {}", address);
                }
            }
            Err(e) => {
                eprintln!("⚠️ Failed to send message to {}: {}", address, e);
            }
        }
    }
}

async fn handle_peer(stream: Arc<Mutex<TcpStream>>, address: SocketAddr, peers: PeerMap) {
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
                println!("📨 Received from {}: {}", address, msg);
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
