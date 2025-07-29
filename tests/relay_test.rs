use rsocks5::relay::Relay;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[test]
fn test_relay_new() {
    // Create test data
    let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    let client_port = 54321;
    let client_addr = SocketAddr::new(client_ip, client_port);
    let target_addr = "example.com:443".to_string();

    // Create a new Relay instance
    let relay = Relay::new(client_addr, target_addr.clone());

    // Verify the fields are set correctly using the getter methods
    assert_eq!(relay.client_addr(), client_addr);
    assert_eq!(relay.target_addr(), &target_addr);
}

// Note: Testing the relay functionality is challenging because it involves
// bidirectional data transfer using tokio's async I/O. The tokio_test::io
// module doesn't fully support this scenario. In a real-world scenario,
// we would need to refactor the code to make it more testable or use a
// more sophisticated mocking approach.
//
// For example, we could:
// 1. Extract the data copying logic into a separate function that takes
//    generic AsyncRead and AsyncWrite traits
// 2. Use dependency injection to allow for easier mocking
// 3. Create a custom mock implementation for testing
//
// For now, we've focused on testing the Relay struct's constructor and
// providing comments explaining the limitations of the tests.