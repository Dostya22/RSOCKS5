//! SOCKS5 server implementation.
//!
//! This module provides the main server functionality for the SOCKS5 proxy,
//! including server initialization and client connection handling.

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use log;

use crate::constants::DEFAULT_PORT;
use crate::error::{Socks5Error, Socks5Result};
use crate::protocol::{handshake, process_command};
use crate::connection::connect_to_target;
use crate::relay::relay_data;

/// SOCKS5 proxy server
pub struct Server {
    /// The address the server is bound to
    bind_addr: String,
    /// The port the server is listening on
    port: u16,
    /// Optional username for authentication
    username: Option<String>,
    /// Optional password for authentication
    password: Option<String>,
}

impl Server {
    /// Creates a new SOCKS5 server instance
    ///
    /// # Arguments
    /// * `bind_addr` - The address to bind the server to (e.g., "0.0.0.0")
    /// * `port` - The port to listen on (default: 1080)
    /// * `username` - Optional username for authentication
    /// * `password` - Optional password for authentication
    ///
    /// # Returns
    /// * A new Server instance
    pub fn new(bind_addr: String, port: Option<u16>, username: Option<String>, password: Option<String>) -> Self {
        Self {
            bind_addr,
            port: port.unwrap_or(DEFAULT_PORT),
            username,
            password,
        }
    }

    /// Returns the server's bind address
    pub fn bind_addr(&self) -> &str {
        &self.bind_addr
    }

    /// Returns the server's port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the server's bind address as a string
    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind_addr, self.port)
    }

    /// Starts the SOCKS5 server
    ///
    /// This method binds to the specified address and port, then enters a loop
    /// to accept and handle client connections.
    ///
    /// # Returns
    /// * `Ok(())` - If the server starts and runs successfully
    /// * `Err(Socks5Error)` - If an error occurs during server operation
    pub async fn run(&self) -> Socks5Result<()> {
        // Bind the TCP listener to the specified address and port
        let listener = TcpListener::bind(self.addr()).await
            .map_err(|e| Socks5Error::IoError(e))?;
        
        log::info!("SOCKS5 proxy listening on {}", self.addr());
        
        // Loop indefinitely to accept incoming client connections
        loop {
            // Accept a new client connection
            let (client_stream, peer_addr) = match listener.accept().await {
                Ok((stream, addr)) => (stream, addr),
                Err(e) => {
                    log::error!("Error accepting connection: {}", e);
                    continue;
                }
            };
            
            log::info!("New client connected from: {:?}", peer_addr);
            
            // Clone username and password to avoid lifetime issues
            let username_clone = self.username.clone();
            let password_clone = self.password.clone();
            
            // Spawn a new task to handle the client
            tokio::spawn(async move {
                // Convert Option<String> to Option<&str>
                let username_ref = username_clone.as_deref();
                let password_ref = password_clone.as_deref();
                
                if let Err(e) = handle_client(client_stream, peer_addr, username_ref, password_ref).await {
                    log::error!("Error handling client {}: {}", peer_addr, e);
                }
            });
        }
    }
}

/// Handles a single client connection
///
/// This function implements the SOCKS5 protocol flow:
/// 1. Perform handshake
/// 2. Process command request
/// 3. Connect to target
/// 4. Relay data between client and target
///
/// # Arguments
/// * `client_stream` - The TCP stream connected to the client
/// * `peer_addr` - The client's socket address
/// * `username` - Optional username for authentication
/// * `password` - Optional password for authentication
///
/// # Returns
/// * `Ok(())` - If client handling completes successfully
/// * `Err(Socks5Error)` - If an error occurs during client handling
async fn handle_client(
    mut client_stream: TcpStream, 
    peer_addr: SocketAddr,
    username: Option<&str>,
    password: Option<&str>
) -> Socks5Result<()> {
    // Step 1: Perform SOCKS5 handshake
    handshake(&mut client_stream, username, password).await?;
    
    if username.is_some() {
        log::info!("SOCKS5 handshake with authentication successful with {:?}", peer_addr);
    } else {
        log::info!("SOCKS5 handshake successful with {:?}", peer_addr);
    }
    
    // Step 2: Process command request
    let target_addr = process_command(&mut client_stream).await?;
    log::info!("Received request to connect to: {}", target_addr.to_string());
    
    // Step 3: Connect to target server
    let target_stream = connect_to_target(&mut client_stream, &target_addr).await?;
    
    // Step 4: Relay data between client and target
    relay_data(
        client_stream,
        peer_addr,
        target_stream,
        target_addr.to_string(),
    ).await?;
    
    log::info!("Connection closed for client: {:?}", peer_addr);
    Ok(())
}