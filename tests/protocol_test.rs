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