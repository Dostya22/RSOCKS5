//! Constants used in the SOCKS5 protocol implementation.
//!
//! This module defines constants for SOCKS5 protocol as specified in RFC 1928.
//! Centralizing these values makes the code more maintainable and easier to understand.

/// SOCKS protocol version
pub const SOCKS_VERSION: u8 = 0x05;

/// Authentication methods
pub mod auth {
    /// No authentication required
    pub const NO_AUTH: u8 = 0x00;
    /// GSSAPI authentication (not implemented)
    pub const GSSAPI: u8 = 0x01;
    /// Username/Password authentication (not implemented)
    pub const USER_PASS: u8 = 0x02;
    /// No acceptable methods
    pub const NO_ACCEPTABLE_METHODS: u8 = 0xFF;
}

/// Command codes
pub mod cmd {
    /// CONNECT command
    pub const CONNECT: u8 = 0x01;
    /// BIND command (not implemented)
    pub const BIND: u8 = 0x02;
    /// UDP ASSOCIATE command (not implemented)
    pub const UDP_ASSOCIATE: u8 = 0x03;
}

/// Address types
pub mod atyp {
    /// IPv4 address
    pub const IPV4: u8 = 0x01;
    /// Domain name
    pub const DOMAIN: u8 = 0x03;
    /// IPv6 address
    pub const IPV6: u8 = 0x04;
}

/// Reply codes
pub mod reply {
    /// Succeeded
    pub const SUCCEEDED: u8 = 0x00;
    /// General SOCKS server failure
    pub const GENERAL_FAILURE: u8 = 0x01;
    /// Connection not allowed by ruleset
    pub const NOT_ALLOWED: u8 = 0x02;
    /// Network unreachable
    pub const NETWORK_UNREACHABLE: u8 = 0x03;
    /// Host unreachable
    pub const HOST_UNREACHABLE: u8 = 0x04;
    /// Connection refused
    pub const CONNECTION_REFUSED: u8 = 0x05;
    /// TTL expired
    pub const TTL_EXPIRED: u8 = 0x06;
    /// Command not supported
    pub const COMMAND_NOT_SUPPORTED: u8 = 0x07;
    /// Address type not supported
    pub const ADDRESS_TYPE_NOT_SUPPORTED: u8 = 0x08;
}

/// Reserved byte value
pub const RESERVED: u8 = 0x00;

/// Default SOCKS5 port
pub const DEFAULT_PORT: u16 = 1080;