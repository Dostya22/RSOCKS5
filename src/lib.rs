//! # rsocks5
//! 
//! A SOCKS5 proxy server implementation in Rust using Tokio for asynchronous I/O.
//! 
//! This library provides a modular implementation of the SOCKS5 protocol as defined in
//! [RFC 1928](https://datatracker.ietf.org/doc/html/rfc1928), with username/password
//! authentication as defined in [RFC 1929](https://datatracker.ietf.org/doc/html/rfc1929).
//! 
//! ## Features
//! 
//! - SOCKS5 protocol implementation
//! - Support for CONNECT command
//! - IPv4 and domain name address types
//! - Authentication methods:
//!   - No authentication
//!   - Username/password authentication
//! - Asynchronous I/O using Tokio

pub mod constants;
pub mod error;
pub mod protocol;
pub mod connection;
pub mod relay;
pub mod server;

// Re-export main components for easier access
pub use server::Server;
pub use error::Socks5Error;