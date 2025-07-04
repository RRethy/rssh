use crate::error::{Result, SshError};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub const SSH_VERSION: &str = "SSH-2.0-rssh_0.1.0";

pub async fn send_version(stream: &mut TcpStream) -> Result<()> {
    let version_string = format!("{SSH_VERSION}\r\n");
    stream.write_all(version_string.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn read_version(stream: &mut TcpStream) -> Result<String> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).await?;

    if line.ends_with("\r\n") {
        line.truncate(line.len() - 2);
    } else if line.ends_with('\n') {
        line.truncate(line.len() - 1);
    }

    if !line.starts_with("SSH-") {
        return Err(SshError::Protocol(format!(
            "Invalid SSH version string: {}",
            line
        )));
    }

    Ok(line)
}

pub fn parse_version(version: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = version.split('-').collect();
    if parts.len() < 3 {
        return Err(SshError::Protocol(format!(
            "Invalid version format: {}",
            version
        )));
    }

    let proto_version = format!("{}-{}", parts[0], parts[1]);
    let software_version = parts[2..].join("-");

    if proto_version != "SSH-2.0" {
        return Err(SshError::Protocol(format!(
            "Unsupported protocol version: {}",
            proto_version
        )));
    }

    Ok((proto_version, software_version))
}

