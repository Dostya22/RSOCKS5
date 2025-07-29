//! Error types for the SOCKS5 protocol implementation.
//!
//! This module defines custom error types that provide more context and
//! better error handling for SOCKS5 protocol operations.

use std::fmt;
use std::io;

/// Custom error type for SOCKS5 protocol operations
#[derive(Debug)]
pub enum Socks5Error {
    /// Error during protocol handshake
    HandshakeError(String),
    
    /// Error during command processing
    CommandError(String),
    
    /// Error related to address handling
    AddressError(String),
    
    /// Error connecting to target server
    ConnectionError(String),
    
    /// Error during data relay
    RelayError(String),
    
    /// Underlying IO error
    IoError(io::Error),
}

impl fmt::Display for Socks5Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Socks5Error::HandshakeError(msg) => write!(f, "SOCKS5 handshake error: {}", msg),
            Socks5Error::CommandError(msg) => write!(f, "SOCKS5 command error: {}", msg),
            Socks5Error::AddressError(msg) => write!(f, "SOCKS5 address error: {}", msg),
            Socks5Error::ConnectionError(msg) => write!(f, "SOCKS5 connection error: {}", msg),
            Socks5Error::RelayError(msg) => write!(f, "SOCKS5 relay error: {}", msg),
            Socks5Error::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Socks5Error {}

impl From<io::Error> for Socks5Error {
    fn from(error: io::Error) -> Self {
        Socks5Error::IoError(error)
    }
}

/// Result type for SOCKS5 operations
pub type Socks5Result<T> = Result<T, Socks5Error>;
