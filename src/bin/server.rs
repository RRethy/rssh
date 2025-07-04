use clap::Parser;
use rssh::error::Result;
use rssh::protocol::{parse_version, read_version, send_version};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(name = "rssh-server")]
#[command(author, version, about = "Rust SSH Server")]
struct Args {
    /// Address to bind the SSH server to
    #[arg(short, long, default_value = "127.0.0.1:2222")]
    bind: SocketAddr,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting SSH server");
    println!("Binding to: {}", args.bind);

    let listener = TcpListener::bind(args.bind).await?;
    let bind = args.bind;
    println!("SSH server listening on {bind}");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {addr}: {e}");
            }
        });
    }
}

async fn handle_client(mut socket: tokio::net::TcpStream) -> Result<()> {
    send_version(&mut socket).await?;
    println!("Sent SSH version");

    let client_version = read_version(&mut socket).await?;
    println!("Received client version: {client_version}");

    let (proto, software) = parse_version(&client_version)?;
    println!("Client protocol: {proto}, software: {software}");

    Ok(())
}
