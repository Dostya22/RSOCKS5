use rsocks5::Server;
use rsocks5::constants::DEFAULT_PORT;

#[test]
fn test_server_new_with_default_port() {
    // Test creating a server with default port
    let server = Server::new("127.0.0.1".to_string(), None, None, None);
    assert_eq!(server.bind_addr(), "127.0.0.1");
    assert_eq!(server.port(), DEFAULT_PORT);
}

#[test]
fn test_server_new_with_custom_port() {
    // Test creating a server with custom port
    let custom_port = 8080;
    let server = Server::new("0.0.0.0".to_string(), Some(custom_port), None, None);
    assert_eq!(server.bind_addr(), "0.0.0.0");
    assert_eq!(server.port(), custom_port);
}

#[test]
fn test_server_addr() {
    // Test the addr method
    let server = Server::new("localhost".to_string(), Some(9999), None, None);
    assert_eq!(server.addr(), "localhost:9999");
}

#[test]
fn test_server_new_with_auth() {
    // Test creating a server with username/password authentication
    let username = "testuser".to_string();
    let password = "testpass".to_string();
    let server = Server::new(
        "127.0.0.1".to_string(), 
        Some(8888), 
        Some(username.clone()), 
        Some(password.clone())
    );
    
    assert_eq!(server.bind_addr(), "127.0.0.1");
    assert_eq!(server.port(), 8888);
    assert_eq!(server.addr(), "127.0.0.1:8888");
}
