//! SOCKS5 protocol implementation.
//!
//! This module handles the SOCKS5 protocol operations as defined in RFC 1928,
//! including handshake, authentication, and command processing.

use std::net::Ipv4Addr;
use std::string::FromUtf8Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::constants::{auth, atyp, cmd, reply, RESERVED, SOCKS_VERSION};
use crate::error::{Socks5Error, Socks5Result};

/// Represents a target address in SOCKS5 protocol
#[derive(Debug, Clone)]
pub enum TargetAddr {
    /// IPv4 address and port
    Ipv4(Ipv4Addr, u16),
    /// Domain name and port
    Domain(String, u16),
}

impl TargetAddr {
    /// Converts the target address to a string representation
    pub fn to_string(&self) -> String {
        match self {
            TargetAddr::Ipv4(addr, port) => format!("{}:{}", addr, port),
            TargetAddr::Domain(domain, port) => format!("{}:{}", domain, port),
        }
    }
}

/// Handles the SOCKS5 handshake process
///
/// The handshake consists of:
/// 1. Client sends version and authentication methods
/// 2. Server selects an authentication method
/// 3. Authentication takes place if required
///
/// # Returns
/// - Ok(()) if handshake is successful
/// - Err(Socks5Error) if handshake fails
pub async fn handshake(stream: &mut TcpStream) -> Socks5Result<()> {
    // Read the first two bytes: SOCKS version (VER) and number of authentication methods (NMETHODS)
    let mut buf = [0; 2];
    stream.read_exact(&mut buf).await?;
    
    let ver = buf[0];
    let nmethods = buf[1];
    
    // Check if the SOCKS version is 5
    if ver != SOCKS_VERSION {
        return Err(Socks5Error::HandshakeError(format!(
            "Unsupported SOCKS version: {}", ver
        )));
    }
    
    // Read the authentication methods
    let mut methods = vec![0; nmethods as usize];
    stream.read_exact(&mut methods).await?;
    
    // Check if the client supports no authentication method
    if methods.contains(&auth::NO_AUTH) {
        // Respond with "no authentication required"
        stream.write_all(&[SOCKS_VERSION, auth::NO_AUTH]).await?;
        Ok(())
    } else {
        // No acceptable authentication methods
        stream.write_all(&[SOCKS_VERSION, auth::NO_ACCEPTABLE_METHODS]).await?;
        Err(Socks5Error::HandshakeError(
            "No acceptable authentication methods".to_string()
        ))
    }
}

/// Processes the SOCKS5 command request
///
/// # Returns
/// - Ok(TargetAddr) with the target address if command is supported
/// - Err(Socks5Error) if command is not supported or other error occurs
pub async fn process_command(stream: &mut TcpStream) -> Socks5Result<TargetAddr> {
    // Read the SOCKS5 request: VER, CMD, RSV, ATYP
    let mut request_header = [0; 4];
    stream.read_exact(&mut request_header).await?;
    
    let ver = request_header[0];
    let command = request_header[1];
    // let rsv = request_header[2]; // Reserved, should be 0x00
    let address_type = request_header[3];
    
    // Verify SOCKS version
    if ver != SOCKS_VERSION {
        send_reply(stream, reply::GENERAL_FAILURE).await?;
        return Err(Socks5Error::CommandError(format!(
            "Unsupported SOCKS version in request: {}", ver
        )));
    }
    
    // Check if command is supported (currently only CONNECT)
    if command != cmd::CONNECT {
        send_reply(stream, reply::COMMAND_NOT_SUPPORTED).await?;
        return Err(Socks5Error::CommandError(format!(
            "Unsupported command: {}", command
        )));
    }
    
    // Parse the target address based on address type
    let target_addr = match address_type {
        atyp::IPV4 => {
            // Read 4 bytes for IPv4 address
            let mut ipv4_bytes = [0; 4];
            stream.read_exact(&mut ipv4_bytes).await?;
            let ipv4_addr = Ipv4Addr::new(
                ipv4_bytes[0], ipv4_bytes[1], ipv4_bytes[2], ipv4_bytes[3]
            );
            
            // Read 2 bytes for port number
            let mut port_bytes = [0; 2];
            stream.read_exact(&mut port_bytes).await?;
            let port = u16::from_be_bytes(port_bytes);
            
            TargetAddr::Ipv4(ipv4_addr, port)
        },
        atyp::DOMAIN => {
            // Read domain name length
            let mut len_buf = [0; 1];
            stream.read_exact(&mut len_buf).await?;
            let domain_len = len_buf[0] as usize;
            
            // Read domain name
            let mut domain_bytes = vec![0; domain_len];
            stream.read_exact(&mut domain_bytes).await?;
            
            // Convert bytes to string
            let domain = String::from_utf8(domain_bytes)
                .map_err(|e: FromUtf8Error| {
                    Socks5Error::AddressError(format!("Invalid domain name: {}", e))
                })?;
            
            // Read port number
            let mut port_bytes = [0; 2];
            stream.read_exact(&mut port_bytes).await?;
            let port = u16::from_be_bytes(port_bytes);
            
            TargetAddr::Domain(domain, port)
        },
        atyp::IPV6 => {
            // IPv6 not implemented in this example
            send_reply(stream, reply::ADDRESS_TYPE_NOT_SUPPORTED).await?;
            return Err(Socks5Error::AddressError(
                "IPv6 address type not supported".to_string()
            ));
        },
        _ => {
            // Unknown address type
            send_reply(stream, reply::ADDRESS_TYPE_NOT_SUPPORTED).await?;
            return Err(Socks5Error::AddressError(format!(
                "Unknown address type: {}", address_type
            )));
        }
    };
    
    Ok(target_addr)
}

/// Sends a SOCKS5 reply to the client
///
/// # Arguments
/// * `stream` - The TCP stream to write to
/// * `reply_code` - The reply code to send
///
/// # Returns
/// - Ok(()) if reply is sent successfully
/// - Err(Socks5Error) if an error occurs
pub async fn send_reply(stream: &mut TcpStream, reply_code: u8) -> Socks5Result<()> {
    // Format: VER, REP, RSV, ATYP, BND.ADDR, BND.PORT
    // Using 0.0.0.0:0 as bind address and port
    let reply = [
        SOCKS_VERSION,
        reply_code,
        RESERVED,
        atyp::IPV4,
        0, 0, 0, 0,  // IP address (0.0.0.0)
        0, 0         // Port (0)
    ];
    
    stream.write_all(&reply).await?;
    Ok(())
}

/// Sends a success reply to the client
///
/// # Arguments
/// * `stream` - The TCP stream to write to
///
/// # Returns
/// - Ok(()) if reply is sent successfully
/// - Err(Socks5Error) if an error occurs
pub async fn send_success_reply(stream: &mut TcpStream) -> Socks5Result<()> {
    send_reply(stream, reply::SUCCEEDED).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_target_addr_to_string() {
        // Test IPv4 address
        let ipv4_addr = TargetAddr::Ipv4(Ipv4Addr::new(192, 168, 1, 1), 8080);
        assert_eq!(ipv4_addr.to_string(), "192.168.1.1:8080");

        // Test domain name
        let domain_addr = TargetAddr::Domain("example.com".to_string(), 443);
        assert_eq!(domain_addr.to_string(), "example.com:443");
    }
}