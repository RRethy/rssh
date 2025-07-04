use thiserror::Error;

#[derive(Error, Debug)]
pub enum SshError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
}

pub type Result<T> = std::result::Result<T, SshError>;
