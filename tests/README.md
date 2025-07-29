# RSOCKS5 Testing Documentation

This document describes the testing approach for the RSOCKS5 project, a SOCKS5 proxy server implemented in Rust.

## Testing Approach

The testing strategy for RSOCKS5 includes:

1. **Unit Tests**: Testing individual components in isolation
2. **Integration Tests**: Testing the interaction between components
3. **Manual Testing**: Using example clients to test the full proxy functionality

### Unit Tests

Unit tests are organized by module, with each module having its own test file:

- `protocol_test.rs`: Tests for the SOCKS5 protocol implementation
- `connection_test.rs`: Tests for connection handling
- `relay_test.rs`: Tests for data relay functionality
- `server_test.rs`: Tests for the server implementation
- `cli_args_test.rs`: Tests for command-line argument parsing

### Integration Tests

Due to the nature of network programming, full integration tests are challenging to implement without significant refactoring. Instead, we provide an example SOCKS5 client that can be used to manually test the proxy functionality.

### Test Limitations

Some components are difficult to test due to their direct interaction with network operations:

1. **Server::run method**: This method binds to a network port and accepts connections, which is difficult to mock.
2. **connect_to_target function**: This function directly calls TcpStream::connect, which is difficult to mock.
3. **relay_data function**: This function involves bidirectional data transfer using tokio's async I/O, which is challenging to test with the current mocking tools.

For these components, we've focused on testing the parts that can be tested in isolation and provided comments explaining the limitations and suggestions for future refactoring to improve testability.

## Running Tests

To run all tests:

```bash
cargo test
```

To run tests for a specific module:

```bash
cargo test --test protocol_test
cargo test --test connection_test
cargo test --test relay_test
cargo test --test server_test
cargo test --test cli_args_test
```

## Manual Testing with Example Client

The project includes an example SOCKS5 client that can be used to manually test the proxy functionality:

1. Start the SOCKS5 proxy server:

```bash
cargo run -- --ip 127.0.0.1 --port 1080
```

2. In another terminal, run the example client:

```bash
cargo run --example socks5_client
```

The example client will:
- Connect to the SOCKS5 proxy at 127.0.0.1:1080
- Request a connection to example.com:80
- Send an HTTP GET request
- Display the HTTP response

You can modify the example client to test different scenarios:
- Change the target host and port
- Test error scenarios (e.g., connecting to a non-existent host)
- Test different SOCKS5 commands (though the proxy currently only supports CONNECT)

## Suggestions for Improving Testability

For future development, consider the following refactoring to improve testability:

1. **Dependency Injection**: Inject dependencies like TcpListener and TcpStream to allow for easier mocking.
2. **Trait-Based Approach**: Define traits for network operations to allow for mock implementations.
3. **Extract Pure Logic**: Separate pure logic from I/O operations to make it easier to test in isolation.
4. **Parameterize Network Operations**: Make network operations configurable to allow for testing without actual network connections.

These changes would make it easier to write comprehensive tests for all components of the SOCKS5 proxy server.