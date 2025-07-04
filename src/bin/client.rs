use clap::Parser;
use rssh::protocol;
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
#[command(name = "rssh-client")]
#[command(author, version, about = "Rust SSH Client")]
struct Args {
    /// SSH server address to connect to
    #[arg(value_name = "HOST[:PORT]", default_value = "127.0.0.1:2222")]
    server: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = args.server;
    println!("Connecting to SSH server at {addr}");

    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to SSH server at {addr}");

    let server_version = protocol::read_version(&mut stream).await?;
    println!("Server version: {server_version}");

    let (proto, software) = protocol::parse_version(&server_version)?;
    println!("Server protocol: {proto}, software: {software}");

    protocol::send_version(&mut stream).await?;
    println!("Sent client version");

    Ok(())
}
