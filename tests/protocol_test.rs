use rsocks5::protocol::TargetAddr;
use std::net::Ipv4Addr;

#[test]
fn test_target_addr_ipv4_to_string() {
    let addr = TargetAddr::Ipv4(Ipv4Addr::new(192, 168, 1, 1), 8080);
    assert_eq!(addr.to_string(), "192.168.1.1:8080");
}

#[test]
fn test_target_addr_domain_to_string() {
    let addr = TargetAddr::Domain("example.com".to_string(), 443);
    assert_eq!(addr.to_string(), "example.com:443");
}

// Note: Testing the protocol functions is challenging because they involve
// network operations and require mocking TcpStream. In a real-world scenario,
// we would refactor the code to make it more testable.
//
// For example, we could:
// 1. Extract the protocol logic into separate functions that take
//    generic AsyncRead and AsyncWrite traits
// 2. Use dependency injection to allow for easier mocking
// 3. Create a custom mock implementation for testing
//
// For now, we've focused on testing the TargetAddr struct's to_string method
// and providing comments explaining the limitations of the tests.