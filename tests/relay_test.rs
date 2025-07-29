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
