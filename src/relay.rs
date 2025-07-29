//! Data relay functionality for SOCKS5 proxy.
//!
//! This module handles bidirectional data transfer between client and target server
//! connections, implementing the core proxy functionality.

use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpStream;
use log;

use crate::error::{Socks5Error, Socks5Result};

/// Represents a data relay between client and target server
pub struct Relay {
    /// Client peer address for logging
    client_addr: SocketAddr,
    /// Target server address string for logging
    target_addr: String,
}

impl Relay {
    /// Creates a new relay instance
    ///
    /// # Arguments
    /// * `client_addr` - The client's socket address
    /// * `target_addr` - The target server's address as a string
    ///
    /// # Returns
    /// * A new Relay instance
    pub fn new(client_addr: SocketAddr, target_addr: String) -> Self {
        Self {
            client_addr,
            target_addr,
        }
    }
    
    /// Returns the client address
    pub fn client_addr(&self) -> SocketAddr {
        self.client_addr
    }
    
    /// Returns the target address
    pub fn target_addr(&self) -> &str {
        &self.target_addr
    }

    /// Starts bidirectional data relay between client and target
    ///
    /// This function splits both streams into read and write halves,
    /// then copies data in both directions concurrently.
    ///
    /// # Arguments
    /// * `client_stream` - The TCP stream connected to the client
    /// * `target_stream` - The TCP stream connected to the target server
    ///
    /// # Returns
    /// * `Ok(())` - If relay completes successfully
    /// * `Err(Socks5Error)` - If an error occurs during relay
    pub async fn start_relay(
        &self,
        client_stream: TcpStream,
        target_stream: TcpStream,
    ) -> Socks5Result<()> {
        log::info!("Starting data relay for client: {:?} to target: {}", 
                 self.client_addr, self.target_addr);
        
        // Split the client and target streams into read and write halves.
        // This allows concurrent reading from one and writing to the other.
        let (mut client_reader, mut client_writer) = client_stream.into_split();
        let (mut target_reader, mut target_writer) = target_stream.into_split();
        
        // Copy data from client to target
        let client_to_target = async {
            match io::copy(&mut client_reader, &mut target_writer).await {
                Ok(n) => {
                    log::info!("Client to target: {} bytes transferred", n);
                    Ok(n)
                }
                Err(e) => Err(Socks5Error::RelayError(format!(
                    "Error copying data from client to target: {}", e
                ))),
            }
        };
        
        // Copy data from target to client
        let target_to_client = async {
            match io::copy(&mut target_reader, &mut client_writer).await {
                Ok(n) => {
                    log::info!("Target to client: {} bytes transferred", n);
                    Ok(n)
                }
                Err(e) => Err(Socks5Error::RelayError(format!(
                    "Error copying data from target to client: {}", e
                ))),
            }
        };
        
        // Run both copy operations concurrently
        match tokio::try_join!(client_to_target, target_to_client) {
            Ok((from_client, from_target)) => {
                log::info!("Data transfer complete: {} bytes from client, {} bytes from target", 
                         from_client, from_target);
                Ok(())
            }
            Err(e) => {
                log::error!("Error during data transfer: {}", e);
                Err(e)
            }
        }
    }
}

/// A simplified function to relay data between client and target streams
///
/// This is a convenience function that creates a Relay instance and starts the relay.
///
/// # Arguments
/// * `client_stream` - The TCP stream connected to the client
/// * `client_addr` - The client's socket address
/// * `target_stream` - The TCP stream connected to the target server
/// * `target_addr` - The target server's address as a string
///
/// # Returns
/// * `Ok(())` - If relay completes successfully
/// * `Err(Socks5Error)` - If an error occurs during relay
pub async fn relay_data(
    client_stream: TcpStream,
    client_addr: SocketAddr,
    target_stream: TcpStream,
    target_addr: String,
) -> Socks5Result<()> {
    let relay = Relay::new(client_addr, target_addr);
    relay.start_relay(client_stream, target_stream).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_relay_new() {
        // Create test data
        let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let client_port = 54321;
        let client_addr = SocketAddr::new(client_ip, client_port);
        let target_addr = "example.com:443".to_string();

        // Create a new Relay instance
        let relay = Relay::new(client_addr, target_addr.clone());

        // Verify the fields are set correctly
        assert_eq!(relay.client_addr, client_addr);
        assert_eq!(relay.target_addr, target_addr);
    }
}