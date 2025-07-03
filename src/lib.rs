pub mod protocol {
    // SSH protocol constants and types
    pub const SSH_VERSION: &str = "SSH-2.0-rssh_0.1.0";
}

pub mod error {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum SshError {
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        
        #[error("Protocol error: {0}")]
        Protocol(String),
        
        #[error("Authentication failed")]
        AuthFailed,
    }
    
    pub type Result<T> = std::result::Result<T, SshError>;
}