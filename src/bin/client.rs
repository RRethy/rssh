use clap::Parser;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
#[command(name = "rssh-client")]
#[command(author, version, about = "Rust SSH Client", long_about = None)]
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

    // TODO: Implement SSH protocol client
    stream.write_all(b"Hello from client\n").await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
