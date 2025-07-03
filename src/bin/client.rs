use clap::Parser;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(name = "rssh-client")]
#[command(author, version, about = "Rust SSH Client", long_about = None)]
struct Args {
    /// SSH server address to connect to
    #[arg(value_name = "HOST[:PORT]", default_value = "127.0.0.1:2222")]
    server: SocketAddr,
    
    /// Username for authentication
    #[arg(short, long, default_value = "user")]
    user: String,
    
    /// Port to connect to (overrides port in server address)
    #[arg(short, long)]
    port: Option<u16>,
    
    /// Identity file (private key) for authentication
    #[arg(short, long)]
    identity: Option<String>,
    
    /// Verbosity level (can be repeated: -v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    
    /// Command to execute on remote server
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let level = match args.verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
    
    let mut addr = args.server;
    if let Some(port) = args.port {
        addr.set_port(port);
    }
    
    info!("Connecting to SSH server at {}", addr);
    info!("Username: {}", args.user);
    
    if let Some(identity) = &args.identity {
        info!("Using identity file: {}", identity);
    }
    
    if !args.command.is_empty() {
        info!("Remote command: {}", args.command.join(" "));
    }
    
    let mut stream = TcpStream::connect(addr).await?;
    info!("Connected to {}", addr);
    
    // TODO: Implement SSH protocol client
    stream.write_all(b"Hello from client\n").await?;
    
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    info!("Received: {}", String::from_utf8_lossy(&buf[..n]));
    
    Ok(())
}