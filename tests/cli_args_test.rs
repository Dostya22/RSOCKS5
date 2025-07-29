use clap::Parser;
use rsocks5::constants::DEFAULT_PORT;
use std::net::IpAddr;
use std::str::FromStr;

// Import the Args struct from the main binary
// Note: We need to redefine it here because it's not exported from the main binary
#[derive(Parser, Debug)]
#[command(author, version, about = "A SOCKS5 proxy server implemented in Rust", long_about = None)]
struct Args {
    /// IP address to bind to
    #[arg(short, long, default_value = "0.0.0.0", value_parser = validate_ip_addr)]
    ip: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info", value_parser = validate_log_level)]
    log_level: String,
}

/// Validates that the provided string is a valid IP address
fn validate_ip_addr(s: &str) -> Result<String, String> {
    match IpAddr::from_str(s) {
        Ok(_) => Ok(s.to_string()),
        Err(_) => Err(format!("Invalid IP address: {}", s)),
    }
}

/// Validates that the provided string is a valid log level
fn validate_log_level(s: &str) -> Result<String, String> {
    match s {
        "trace" | "debug" | "info" | "warn" | "error" => Ok(s.to_string()),
        _ => Err(format!("Invalid log level: {}. Valid values are: trace, debug, info, warn, error", s)),
    }
}

#[test]
fn test_default_args() {
    // Test parsing with no arguments (should use defaults)
    let args = Args::parse_from(["rsocks5"]);
    
    assert_eq!(args.ip, "0.0.0.0");
    assert_eq!(args.port, DEFAULT_PORT);
    assert_eq!(args.log_level, "info");
}

#[test]
fn test_custom_ip() {
    // Test parsing with custom IP
    let args = Args::parse_from(["rsocks5", "--ip", "127.0.0.1"]);
    
    assert_eq!(args.ip, "127.0.0.1");
    assert_eq!(args.port, DEFAULT_PORT); // Should still use default port
    assert_eq!(args.log_level, "info"); // Should still use default log level
}

#[test]
fn test_custom_port() {
    // Test parsing with custom port
    let args = Args::parse_from(["rsocks5", "--port", "8080"]);
    
    assert_eq!(args.ip, "0.0.0.0"); // Should still use default IP
    assert_eq!(args.port, 8080);
    assert_eq!(args.log_level, "info"); // Should still use default log level
}

#[test]
fn test_custom_log_level() {
    // Test parsing with custom log level
    let args = Args::parse_from(["rsocks5", "--log-level", "debug"]);
    
    assert_eq!(args.ip, "0.0.0.0"); // Should still use default IP
    assert_eq!(args.port, DEFAULT_PORT); // Should still use default port
    assert_eq!(args.log_level, "debug");
}

#[test]
fn test_short_args() {
    // Test parsing with short argument forms
    let args = Args::parse_from(["rsocks5", "-i", "192.168.1.1", "-p", "9999", "-l", "trace"]);
    
    assert_eq!(args.ip, "192.168.1.1");
    assert_eq!(args.port, 9999);
    assert_eq!(args.log_level, "trace");
}

#[test]
fn test_all_args() {
    // Test parsing with all arguments specified
    let args = Args::parse_from([
        "rsocks5", 
        "--ip", "10.0.0.1", 
        "--port", "1234", 
        "--log-level", "warn"
    ]);
    
    assert_eq!(args.ip, "10.0.0.1");
    assert_eq!(args.port, 1234);
    assert_eq!(args.log_level, "warn");
}

#[test]
fn test_validate_ip_addr_valid() {
    // Test validation of valid IP addresses
    assert!(validate_ip_addr("127.0.0.1").is_ok());
    assert!(validate_ip_addr("0.0.0.0").is_ok());
    assert!(validate_ip_addr("192.168.1.1").is_ok());
    assert!(validate_ip_addr("255.255.255.255").is_ok());
    assert!(validate_ip_addr("::1").is_ok()); // IPv6 localhost
    assert!(validate_ip_addr("2001:db8::1").is_ok()); // IPv6 example
}

#[test]
fn test_validate_ip_addr_invalid() {
    // Test validation of invalid IP addresses
    assert!(validate_ip_addr("invalid").is_err());
    assert!(validate_ip_addr("256.0.0.1").is_err()); // Invalid IPv4 octet
    assert!(validate_ip_addr("127.0.0").is_err()); // Incomplete IPv4
    assert!(validate_ip_addr("127.0.0.1.1").is_err()); // Too many octets
    assert!(validate_ip_addr("").is_err()); // Empty string
}

#[test]
fn test_validate_log_level_valid() {
    // Test validation of valid log levels
    assert!(validate_log_level("trace").is_ok());
    assert!(validate_log_level("debug").is_ok());
    assert!(validate_log_level("info").is_ok());
    assert!(validate_log_level("warn").is_ok());
    assert!(validate_log_level("error").is_ok());
}

#[test]
fn test_validate_log_level_invalid() {
    // Test validation of invalid log levels
    assert!(validate_log_level("invalid").is_err());
    assert!(validate_log_level("warning").is_err()); // Similar but not exact
    assert!(validate_log_level("INFO").is_err()); // Case sensitive
    assert!(validate_log_level("").is_err()); // Empty string
}