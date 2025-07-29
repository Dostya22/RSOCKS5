//! Target connection handling for SOCKS5 proxy.
//!
//! This module is responsible for establishing connections to target servers
//! as requested by SOCKS5 clients.

use tokio::net::TcpStream;

use crate::error::{Socks5Error, Socks5Result};
use crate::protocol::{TargetAddr, send_reply, send_success_reply};
use crate::constants::reply;

/// Establishes a connection to the target server.
///
/// # Arguments
/// * `client_stream` - The client TCP stream for sending replies
/// * `target_addr` - The target address to connect to
///
/// # Returns
/// * `Ok(TcpStream)` - The established connection to the target server
/// * `Err(Socks5Error)` - If connection fails
pub async fn connect_to_target(
    client_stream: &mut TcpStream,
    target_addr: &TargetAddr,
) -> Socks5Result<TcpStream> {
    // Convert target address to string format for connection
    let addr_string = target_addr.to_string();
    
    // Log connection attempt
    log::info!("Connecting to target: {}", addr_string);
    
    // Attempt to connect to the target server
    match TcpStream::connect(&addr_string).await {
        Ok(stream) => {
            // Connection successful, send success reply to client
            send_success_reply(client_stream).await?;
            log::info!("Successfully connected to target: {}", addr_string);
            Ok(stream)
        }
        Err(e) => {
            // Connection failed, determine appropriate error code
            let reply_code = match e.kind() {
                std::io::ErrorKind::ConnectionRefused => reply::CONNECTION_REFUSED,
                std::io::ErrorKind::TimedOut => reply::HOST_UNREACHABLE,
                std::io::ErrorKind::AddrNotAvailable => reply::NETWORK_UNREACHABLE,
                _ => reply::HOST_UNREACHABLE, // Default to host unreachable
            };
            
            // Send error reply to client
            send_reply(client_stream, reply_code).await?;
            
            // Return error
            Err(Socks5Error::ConnectionError(format!(
                "Failed to connect to target {}: {}", addr_string, e
            )))
        }
    }
}

/// A struct representing a connection to a target server
pub struct TargetConnection {
    /// The TCP stream connected to the target server
    pub stream: TcpStream,
    /// The address of the target server
    pub addr: TargetAddr,
}

impl TargetConnection {
    /// Creates a new target connection
    ///
    /// # Arguments
    /// * `stream` - The TCP stream connected to the target server
    /// * `addr` - The address of the target server
    ///
    /// # Returns
    /// * A new TargetConnection instance
    pub fn new(stream: TcpStream, addr: TargetAddr) -> Self {
        Self { stream, addr }
    }
    
    /// Gets the address as a string
    pub fn addr_string(&self) -> String {
        self.addr.to_string()
    }
}