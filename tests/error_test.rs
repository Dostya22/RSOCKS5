use rsocks5::error::Socks5Error;
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
