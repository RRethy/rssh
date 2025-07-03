use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let addr = "127.0.0.1:2222";
    let listener = TcpListener::bind(addr).await?;
    info!("SSH server listening on {}", addr);
    
    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New connection from {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                error!("Error handling client: {}", e);
            }
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement SSH protocol handling
    Ok(())
}