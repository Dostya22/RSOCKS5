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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error_creation() {
        // Test creating each type of error
        let handshake_err = Socks5Error::HandshakeError("handshake failed".to_string());
        let command_err = Socks5Error::CommandError("invalid command".to_string());
        let address_err = Socks5Error::AddressError("invalid address".to_string());
        let connection_err = Socks5Error::ConnectionError("connection failed".to_string());
        let relay_err = Socks5Error::RelayError("relay failed".to_string());
        let io_err = Socks5Error::IoError(IoError::new(ErrorKind::ConnectionRefused, "connection refused"));

        // Verify the debug representation contains the expected information
        assert!(format!("{:?}", handshake_err).contains("HandshakeError"));
        assert!(format!("{:?}", command_err).contains("CommandError"));
        assert!(format!("{:?}", address_err).contains("AddressError"));
        assert!(format!("{:?}", connection_err).contains("ConnectionError"));
        assert!(format!("{:?}", relay_err).contains("RelayError"));
        assert!(format!("{:?}", io_err).contains("IoError"));
    }

    #[test]
    fn test_error_display() {
        // Test the Display implementation for each error type
        let handshake_err = Socks5Error::HandshakeError("handshake failed".to_string());
        assert_eq!(format!("{}", handshake_err), "SOCKS5 handshake error: handshake failed");

        let command_err = Socks5Error::CommandError("invalid command".to_string());
        assert_eq!(format!("{}", command_err), "SOCKS5 command error: invalid command");

        let address_err = Socks5Error::AddressError("invalid address".to_string());
        assert_eq!(format!("{}", address_err), "SOCKS5 address error: invalid address");

        let connection_err = Socks5Error::ConnectionError("connection failed".to_string());
        assert_eq!(format!("{}", connection_err), "SOCKS5 connection error: connection failed");

        let relay_err = Socks5Error::RelayError("relay failed".to_string());
        assert_eq!(format!("{}", relay_err), "SOCKS5 relay error: relay failed");

        let io_err = Socks5Error::IoError(IoError::new(ErrorKind::ConnectionRefused, "connection refused"));
        assert!(format!("{}", io_err).contains("IO error: connection refused"));
    }

    #[test]
    fn test_from_io_error() {
        // Test conversion from io::Error
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let socks_error: Socks5Error = io_error.into();
        
        match socks_error {
            Socks5Error::IoError(e) => {
                assert_eq!(e.kind(), ErrorKind::NotFound);
                assert_eq!(e.to_string(), "file not found");
            },
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_error_trait() {
        // Test that Socks5Error implements std::error::Error
        let error = Socks5Error::HandshakeError("test error".to_string());
        let _: Box<dyn std::error::Error> = Box::new(error);
        
        // This test passes if the code compiles, as it verifies that
        // Socks5Error can be used as a std::error::Error
    }
}