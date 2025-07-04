use thiserror::Error;

#[derive(Error, Debug)]
pub enum SshError {}

pub type Result<T> = std::result::Result<T, SshError>;
