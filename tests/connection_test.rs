use rsocks5::protocol::TargetAddr;
use std::net::Ipv4Addr;

#[test]
fn test_target_addr_ipv4_to_string() {
    // Create a target address
    let addr = TargetAddr::Ipv4(Ipv4Addr::new(192, 168, 1, 1), 8080);
    
    // Verify the to_string method returns the expected string
    assert_eq!(addr.to_string(), "192.168.1.1:8080");
}

#[test]
fn test_target_addr_domain_to_string() {
    // Create a domain target address
    let addr = TargetAddr::Domain("example.com".to_string(), 443);
    
    // Verify the to_string method returns the expected string
    assert_eq!(addr.to_string(), "example.com:443");
}

// Note: Testing the connect_to_target function is challenging because it directly
// calls TcpStream::connect, which is difficult to mock without refactoring the code.
// In a real-world scenario, we would refactor the code to use dependency injection
// or a trait-based approach to make it more testable.
//
// Initially, we tried to test the TargetConnection struct's addr_string method by creating
// a TargetConnection instance with a placeholder TcpStream using std::mem::zeroed().
// However, this approach caused a panic because TcpStream cannot be safely zero-initialized.
//
// Instead, we've refactored the tests to test the TargetAddr::to_string method directly,
// since that's what TargetConnection::addr_string calls. This approach avoids the need
// to create a mock TcpStream altogether and focuses the tests on what they were actually
// testing: the string representation of the target address.