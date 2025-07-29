use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// A simple SOCKS5 client example that connects to a SOCKS5 proxy and
/// makes a request to a target server through the proxy.
///
/// This example can be used to test the SOCKS5 proxy server manually.
///
/// Usage:
/// ```
/// cargo run --example socks5_client
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configuration
    let proxy_addr = "127.0.0.1:1080"; // Address of the SOCKS5 proxy server
    let target_host = "example.com"; // Target host to connect to
    let target_port: u16 = 80; // Target port to connect to

    println!("Connecting to SOCKS5 proxy at {}", proxy_addr);
    
    // Connect to the SOCKS5 proxy server
    let mut stream = TcpStream::connect(proxy_addr).await?;
    
    println!("Connected to proxy, performing handshake");
    
    // Step 1: Perform SOCKS5 handshake
    // Send: SOCKS version 5, 1 auth method, NO_AUTH (0x00)
    stream.write_all(&[0x05, 0x01, 0x00]).await?;
    
    // Receive: SOCKS version 5, selected auth method (should be NO_AUTH)
    let mut response = [0u8; 2];
    stream.read_exact(&mut response).await?;
    
    if response[0] != 0x05 {
        return Err(format!("Unexpected SOCKS version: {}", response[0]).into());
    }
    
    if response[1] != 0x00 {
        return Err(format!("Unexpected auth method: {}", response[1]).into());
    }
    
    println!("Handshake successful, sending connection request");
    
    // Step 2: Send connection request
    // Prepare the target host as bytes
    let host_bytes = target_host.as_bytes();
    let host_len = host_bytes.len() as u8;
    
    // Create the connection request
    // Format: SOCKS version, command (CONNECT), reserved, address type, address, port
    let mut request = vec![
        0x05, // SOCKS version
        0x01, // CONNECT command
        0x00, // Reserved
        0x03, // Domain name address type
        host_len, // Length of the domain name
    ];
    
    // Add the domain name
    request.extend_from_slice(host_bytes);
    
    // Add the port in network byte order (big-endian)
    request.extend_from_slice(&target_port.to_be_bytes());
    
    // Send the connection request
    stream.write_all(&request).await?;
    
    // Receive the reply
    let mut reply = [0u8; 4];
    stream.read_exact(&mut reply).await?;
    
    if reply[0] != 0x05 {
        return Err(format!("Unexpected SOCKS version in reply: {}", reply[0]).into());
    }
    
    if reply[1] != 0x00 {
        let error = match reply[1] {
            0x01 => "General failure",
            0x02 => "Connection not allowed by ruleset",
            0x03 => "Network unreachable",
            0x04 => "Host unreachable",
            0x05 => "Connection refused",
            0x06 => "TTL expired",
            0x07 => "Command not supported",
            0x08 => "Address type not supported",
            _ => "Unknown error",
        };
        return Err(format!("Connection failed: {}", error).into());
    }
    
    // Read the rest of the reply (bound address and port)
    // We don't need this information for this example, so we'll just read and discard it
    let atyp = reply[3];
    match atyp {
        0x01 => {
            // IPv4 address (4 bytes) + port (2 bytes)
            let mut addr_port = [0u8; 6];
            stream.read_exact(&mut addr_port).await?;
        },
        0x03 => {
            // Domain name length (1 byte)
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).await?;
            // Domain name (len bytes) + port (2 bytes)
            let mut addr_port = vec![0u8; len[0] as usize + 2];
            stream.read_exact(&mut addr_port).await?;
        },
        0x04 => {
            // IPv6 address (16 bytes) + port (2 bytes)
            let mut addr_port = [0u8; 18];
            stream.read_exact(&mut addr_port).await?;
        },
        _ => {
            return Err(format!("Unexpected address type in reply: {}", atyp).into());
        }
    }
    
    println!("Connection to target established, sending HTTP request");
    
    // Step 3: Send HTTP request to the target server through the proxy
    let http_request = format!(
        "GET / HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         \r\n",
        target_host
    );
    
    stream.write_all(http_request.as_bytes()).await?;
    
    println!("HTTP request sent, reading response");
    
    // Step 4: Read the HTTP response
    let mut buffer = [0u8; 4096];
    let mut response = Vec::new();
    
    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..n]);
    }
    
    // Convert the response to a string and print it
    let response_str = String::from_utf8_lossy(&response);
    println!("Received HTTP response:");
    println!("{}", response_str);
    
    Ok(())
}