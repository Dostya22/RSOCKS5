use rsocks5::{Server, constants::DEFAULT_PORT};
use env_logger::{self, Env};
use log;
use clap::Parser;
use std::net::IpAddr;
use std::str::FromStr;

/// Command line arguments for the SOCKS5 proxy server
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
    
    /// Username for SOCKS5 authentication (requires password to be set as well)
    #[arg(short = 'U', long)]
    username: Option<String>,
    
    /// Password for SOCKS5 authentication (requires username to be set as well)
    #[arg(short = 'P', long)]
    password: Option<String>,
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

/// Main function where the SOCKS5 proxy server starts
///
/// This function parses command-line arguments, initializes the logger,
/// creates a new Server instance with the specified parameters,
/// and starts the server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Validate that both username and password are provided if either is provided
    if args.username.is_some() != args.password.is_some() {
        return Err("Both username and password must be provided if either is provided".into());
    }
    
    // Initialize the logger with the specified log level
    env_logger::Builder::from_env(Env::default().default_filter_or(&args.log_level)).init();
    
    // Log server start
    log::info!("Starting SOCKS5 proxy server on {}:{}", args.ip, args.port);
    
    // Log authentication status
    if args.username.is_some() {
        log::info!("Authentication required with username: {}", args.username.as_ref().unwrap());
    } else {
        log::info!("No authentication required");
    }
    
    // Create a new server instance with the specified IP, port, and authentication credentials
    let server = Server::new(
        args.ip.clone(), 
        Some(args.port),
        args.username.clone(),
        args.password.clone()
    );
    
    // Run the server
    server.run().await?;
    
    Ok(())
}