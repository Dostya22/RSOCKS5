# RSOCKS5

A SOCKS5 proxy server implementation in Rust using Tokio for asynchronous I/O.

## Overview

RSOCKS5 is a lightweight, high-performance SOCKS5 proxy server that implements the SOCKS5 protocol as defined in [RFC 1928](https://datatracker.ietf.org/doc/html/rfc1928). It's built with Rust and Tokio to provide efficient, non-blocking I/O operations.

## Features

- **SOCKS5 Protocol Implementation**: Fully implements the core SOCKS5 protocol
- **CONNECT Command Support**: Allows clients to establish TCP connections through the proxy
- **Address Type Support**: Handles IPv4 addresses and domain names
- **Authentication Support**: Supports both no authentication and username/password authentication methods
- **Asynchronous I/O**: Built with Tokio for high-performance, non-blocking operations
- **Configurable**: Customizable bind address, port, log level, and authentication credentials
- **Logging**: Comprehensive logging with configurable log levels

## Installation

### Prerequisites

- Rust and Cargo (install from [rust-lang.org](https://www.rust-lang.org/tools/install))

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/Dostya22/RSOCKS5.git
   cd RSOCKS5
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/rsocks5`

## Usage

### Basic Usage

Run the proxy server with default settings (binds to 0.0.0.0:1080):

```
./rsocks5
```

### Command-line Options

```
USAGE:
    rsocks5 [OPTIONS]

OPTIONS:
    -i, --ip <IP>                IP address to bind to [default: 0.0.0.0]
    -p, --port <PORT>            Port to listen on [default: 1080]
    -l, --log-level <LOG_LEVEL>  Log level (trace, debug, info, warn, error) [default: info]
    -U, --username <USERNAME>    Username for SOCKS5 authentication (requires password to be set as well)
    -P, --password <PASSWORD>    Password for SOCKS5 authentication (requires username to be set as well)
    -h, --help                   Print help information
    -V, --version                Print version information
```

### Examples

Run the server on a specific IP and port:
```
./rsocks5 --ip 127.0.0.1 --port 8080
```

Run with debug logging:
```
./rsocks5 --log-level debug
```

Run with username/password authentication:
```
./rsocks5 --username myuser --password mypassword
```

Run with all options combined:
```
./rsocks5 --ip 127.0.0.1 --port 8080 --log-level debug --username myuser --password mypassword
```

## Using with Clients

Any SOCKS5-compatible client can connect to the proxy server. Here are some examples:

### cURL

Without authentication:
```
curl --socks5 127.0.0.1:1080 https://example.com
```

With authentication:
```
curl --socks5-hostname 127.0.0.1:1080 --socks5 myuser:mypassword https://example.com
```

### Web Browser

Most web browsers support SOCKS5 proxies. Configure your browser's network settings to use the proxy server.

For authentication, most browsers allow you to specify a username and password in the proxy settings.

### SSH

Without authentication:
```
ssh -o ProxyCommand="nc -X 5 -x 127.0.0.1:1080 %h %p" user@remote-host
```

With authentication:
```
ssh -o ProxyCommand="nc -X 5 -x myuser:mypassword@127.0.0.1:1080 %h %p" user@remote-host
```

## Architecture

RSOCKS5 is built with a modular architecture:

- **Server**: Handles client connections and orchestrates the SOCKS5 protocol flow
- **Protocol**: Implements the SOCKS5 protocol handshake and command processing
- **Connection**: Manages connections to target servers
- **Relay**: Efficiently transfers data between client and target connections
- **Error Handling**: Comprehensive error types and handling

## Limitations

- Currently only supports the CONNECT command (no BIND or UDP ASSOCIATE)
- Supports NO_AUTH and USERNAME/PASSWORD authentication methods (no GSSAPI)
- IPv6 address support is limited

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tokio](https://tokio.rs/) - The asynchronous runtime for Rust
- [RFC 1928](https://datatracker.ietf.org/doc/html/rfc1928) - SOCKS Protocol Version 5
- [RFC 1929](https://datatracker.ietf.org/doc/html/rfc1929) - Username/Password Authentication for SOCKS V5
