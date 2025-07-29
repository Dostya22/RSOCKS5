use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use rsocks5::constants::{auth, atyp, cmd, SOCKS_VERSION};

/// A simple SOCKS5 client for testing
pub struct Socks5Client {
    stream: TcpStream,
}

impl Socks5Client {
    /// Connect to a SOCKS5 proxy server
    pub async fn connect(proxy_addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(proxy_addr).await?;
        Ok(Self { stream })
    }

    /// Perform the SOCKS5 handshake with the proxy server
    pub async fn handshake(&mut self) -> io::Result<()> {
        // Send handshake request: VER, NMETHODS, METHODS
        let request = [
            SOCKS_VERSION, // VER
            1,             // NMETHODS
            auth::NO_AUTH, // METHOD (no authentication)
        ];
        self.stream.write_all(&request).await?;

        // Read handshake response: VER, METHOD
        let mut response = [0u8; 2];
        self.stream.read_exact(&mut response).await?;

        // Check if the server accepted our authentication method
        if response[0] != SOCKS_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unexpected SOCKS version: {}", response[0]),
            ));
        }

        if response[1] == auth::NO_ACCEPTABLE_METHODS {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "No acceptable authentication methods",
            ));
        }

        Ok(())
    }

    /// Send a CONNECT command to the proxy server
    pub async fn connect_to(&mut self, target_addr: &str, target_port: u16) -> io::Result<()> {
        // Parse the target address
        let addr_parts: Vec<&str> = target_addr.split('.').collect();
        
        // Determine if the target is an IPv4 address or a domain name
        let is_ipv4 = addr_parts.len() == 4 && addr_parts.iter().all(|part| {
            part.parse::<u8>().is_ok()
        });

        // Prepare the CONNECT request
        let mut request = vec![
            SOCKS_VERSION, // VER
            cmd::CONNECT,  // CMD
            0,             // RSV
        ];

        if is_ipv4 {
            // IPv4 address
            request.push(atyp::IPV4);
            
            // Add the IPv4 address bytes
            for part in addr_parts {
                request.push(part.parse::<u8>().unwrap());
            }
        } else {
            // Domain name
            request.push(atyp::DOMAIN);
            
            // Add the domain name length and bytes
            request.push(target_addr.len() as u8);
            request.extend_from_slice(target_addr.as_bytes());
        }

        // Add the port (big-endian)
        request.push((target_port >> 8) as u8);
        request.push((target_port & 0xFF) as u8);

        // Send the CONNECT request
        self.stream.write_all(&request).await?;

        // Read the response: VER, REP, RSV, ATYP, BND.ADDR, BND.PORT
        let mut response_header = [0u8; 4];
        self.stream.read_exact(&mut response_header).await?;

        // Check the response
        if response_header[0] != SOCKS_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unexpected SOCKS version in response: {}", response_header[0]),
            ));
        }

        if response_header[1] != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("SOCKS5 server returned error code: {}", response_header[1]),
            ));
        }

        // Read the bound address and port based on the address type
        match response_header[3] {
            atyp::IPV4 => {
                // Read 4 bytes for IPv4 address and 2 bytes for port
                let mut addr_port = [0u8; 6];
                self.stream.read_exact(&mut addr_port).await?;
            },
            atyp::DOMAIN => {
                // Read domain name length
                let mut len_buf = [0u8; 1];
                self.stream.read_exact(&mut len_buf).await?;
                let domain_len = len_buf[0] as usize;
                
                // Read domain name and port
                let mut domain_port = vec![0u8; domain_len + 2];
                self.stream.read_exact(&mut domain_port).await?;
            },
            atyp::IPV6 => {
                // Read 16 bytes for IPv6 address and 2 bytes for port
                let mut addr_port = [0u8; 18];
                self.stream.read_exact(&mut addr_port).await?;
            },
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unsupported address type in response: {}", response_header[3]),
                ));
            }
        }

        Ok(())
    }

    /// Send data to the target server through the proxy
    pub async fn send(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data).await
    }

    /// Receive data from the target server through the proxy
    pub async fn receive(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf).await
    }

    /// Close the connection
    pub async fn close(self) -> io::Result<()> {
        drop(self.stream);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::time::Duration;

    // A simple mock SOCKS5 server for testing the client
    async fn run_mock_server(listener: TcpListener) {
        let (mut socket, _) = listener.accept().await.unwrap();
        
        // Handle handshake
        let mut buf = [0u8; 3];
        socket.read_exact(&mut buf).await.unwrap();
        
        // Respond with success
        socket.write_all(&[SOCKS_VERSION, auth::NO_AUTH]).await.unwrap();
        
        // Handle CONNECT command
        let mut header = [0u8; 4];
        socket.read_exact(&mut header).await.unwrap();
        
        // Read the rest of the CONNECT request based on address type
        match header[3] {
            atyp::IPV4 => {
                let mut addr_port = [0u8; 6];
                socket.read_exact(&mut addr_port).await.unwrap();
            },
            atyp::DOMAIN => {
                let mut len_buf = [0u8; 1];
                socket.read_exact(&mut len_buf).await.unwrap();
                let domain_len = len_buf[0] as usize;
                
                let mut domain_port = vec![0u8; domain_len + 2];
                socket.read_exact(&mut domain_port).await.unwrap();
            },
            _ => {
                // Unsupported address type
                return;
            }
        }
        
        // Respond with success
        let response = [
            SOCKS_VERSION, // VER
            0,             // REP (success)
            0,             // RSV
            atyp::IPV4,    // ATYP
            127, 0, 0, 1,  // BND.ADDR (127.0.0.1)
            0x1F, 0x90     // BND.PORT (8080)
        ];
        socket.write_all(&response).await.unwrap();
        
        // Echo any data received
        let mut buf = [0u8; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    socket.write_all(&buf[0..n]).await.unwrap();
                },
                Err(_) => break,
            }
        }
    }

    #[tokio::test]
    async fn test_socks5_client() {
        // Start a mock SOCKS5 server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        
        let server_handle = tokio::spawn(async move {
            run_mock_server(listener).await;
        });
        
        // Give the server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Connect to the mock server
        let mut client = Socks5Client::connect(&server_addr.to_string()).await.unwrap();
        
        // Perform handshake
        client.handshake().await.unwrap();
        
        // Connect to a target
        client.connect_to("example.com", 80).await.unwrap();
        
        // Send and receive data
        let test_data = b"Hello, SOCKS5!";
        client.send(test_data).await.unwrap();
        
        let mut response = [0u8; 1024];
        let n = client.receive(&mut response).await.unwrap();
        
        assert_eq!(&response[0..n], test_data);
        
        // Close the connection
        client.close().await.unwrap();
        
        // Clean up
        server_handle.abort();
    }
}