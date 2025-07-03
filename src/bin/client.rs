use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let args: Vec<String> = env::args().collect();
    let addr = if args.len() > 1 {
        &args[1]
    } else {
        "127.0.0.1:2222"
    };
    
    info!("Connecting to SSH server at {}", addr);
    let mut stream = TcpStream::connect(addr).await?;
    info!("Connected to {}", addr);
    
    // TODO: Implement SSH protocol client
    stream.write_all(b"Hello from client\n").await?;
    
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    info!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    
    Ok(())
}