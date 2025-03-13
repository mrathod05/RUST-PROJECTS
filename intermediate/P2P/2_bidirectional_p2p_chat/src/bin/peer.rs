use std::{error::Error, net::SocketAddr, process, str::FromStr, sync::Arc};

use clap::{Arg, Command};
use tokio::{
    io::{self, AsyncReadExt},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Sender},
};

#[derive(Debug, Clone)]
struct ChatMessage {
    from: SocketAddr,
    message: String,
}

fn get_args() -> (std::net::SocketAddr, std::net::SocketAddr) {
    let matches = Command::new("Bidireactional p2p")
        .about("To ge the peer address")
        .arg(Arg::new("LISTEN").short('l').long("listing").required(true))
        .arg(
            Arg::new("CONNECT")
                .short('c')
                .long("connecting")
                .required(true),
        )
        .get_matches();

    let listing_address = match matches.get_one::<String>("LISTEN") {
        Some(add) => match SocketAddr::from_str(add) {
            Ok(address) => address,
            Err(_) => {
                eprintln!("Error: Invalid address {} ", add);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Please define listening address");
            process::exit(1);
        }
    };

    let connection_address = match matches.get_one::<String>("CONNECT") {
        Some(add) => match SocketAddr::from_str(add) {
            Ok(address) => address,
            Err(_) => {
                eprintln!("Error: Invalid address {} ", add);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Please define connection address");
            process::exit(1);
        }
    };

    return (listing_address, connection_address);
}

async fn listing_for_client(
    listing_add: SocketAddr,
    tx: Sender<ChatMessage>,
) -> Result<(), Box<dyn Error>> {
    let listen = TcpListener::bind(listing_add).await?;

    match listen.accept().await {
        Ok((stream, address)) => {
            println!("New connection {}", address);
            print!(">");

            tokio::io::AsyncWriteExt::flush(&mut tokio::io::stdout()).await?;

            let welcome_message = format!("System: {address} had joined the chat");
            let _ = tx.send(ChatMessage {
                from: listing_add,
                message: welcome_message,
            })?;

            let tx = &tx.clone();

            let _ = tokio::spawn(handle_connection(stream, tx));
        }
        Err(e) => {
            eprintln!("Error: Accepting connection {e}")
        }
    };

    Ok(())
}

fn handle_connection(stream: TcpStream, tx: Arc<Sender<ChatMessage>>) -> Result<(), ()> {
    let test = io::split(stream);

    Ok(())
}

#[tokio::main]
async fn main() {
    //Result<(), Box<dyn Error>>
    let (listing_add, connecting_add) = get_args();

    let (tx, rx) = broadcast::channel::<ChatMessage>(20);

    let tx_clone = tx.clone();

    let listing = tokio::spawn(async move {
        listing_for_client(listing_add, tx);
    });
}
