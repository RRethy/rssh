use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(name = "rssh-server")]
#[command(author, version, about = "Rust SSH Server", long_about = None)]
struct Args {
    /// Address to bind the SSH server to
    #[arg(short, long, default_value = "127.0.0.1:2222")]
    bind: SocketAddr,
    
    /// Verbosity level (can be repeated: -v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    
    /// Path to host key file
    #[arg(short = 'k', long)]
    host_key: Option<String>,
    
    /// Maximum number of concurrent connections
    #[arg(short = 'm', long, default_value = "100")]
    max_connections: usize,
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
    
    info!("Starting SSH server");
    info!("Binding to: {}", args.bind);
    info!("Max connections: {}", args.max_connections);
    
    if let Some(key_path) = &args.host_key {
        info!("Using host key: {}", key_path);
    }
    
    let listener = TcpListener::bind(args.bind).await?;
    info!("SSH server listening on {}", args.bind);
    
    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                error!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(_socket: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement SSH protocol handling
    Ok(())
}