use rsocks5::Server;
use rsocks5::constants::DEFAULT_PORT;

#[test]
fn test_server_new_with_default_port() {
    // Test creating a server with default port
    let server = Server::new("127.0.0.1".to_string(), None);
    assert_eq!(server.bind_addr(), "127.0.0.1");
    assert_eq!(server.port(), DEFAULT_PORT);
}

#[test]
fn test_server_new_with_custom_port() {
    // Test creating a server with custom port
    let custom_port = 8080;
    let server = Server::new("0.0.0.0".to_string(), Some(custom_port));
    assert_eq!(server.bind_addr(), "0.0.0.0");
    assert_eq!(server.port(), custom_port);
}

#[test]
fn test_server_addr() {
    // Test the addr method
    let server = Server::new("localhost".to_string(), Some(9999));
    assert_eq!(server.addr(), "localhost:9999");
}

// Note: Testing the Server::run method and handle_client function is challenging
// because they involve network operations and complex async behavior.
// In a real-world scenario, we would refactor the code to make it more testable.
// Here are some suggestions for how the code could be refactored:

// 1. Extract the TcpListener creation into a separate function that can be mocked
// 2. Use dependency injection for the handshake, process_command, connect_to_target,
//    and relay_data functions to allow for easier mocking
// 3. Create interfaces (traits) for the network operations to allow for easier mocking

// For now, we'll focus on testing the Server struct's constructor and addr method,
// which are already covered by the tests above.