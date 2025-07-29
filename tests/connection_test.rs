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
