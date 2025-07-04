use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(name = "rssh-server")]
#[command(author, version, about = "Rust SSH Server", long_about = None)]
struct Args {
    /// Address to bind the SSH server to
    #[arg(short, long, default_value = "127.0.0.1:2222")]
    bind: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

async fn handle_client(_socket: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
