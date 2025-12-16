# DLUT Auto Login - Rust Version

Rust implementation of the DLUT (Dalian University of Technology) campus network auto-login tool.

## Features

- DES/3DES encryption support for login credentials
- Automatic network status detection
- Command-line interface with argument support
- Secure password input handling
- Async HTTP requests with timeout support

## Build

```bash
cargo build --release
```

## Usage

### Basic usage with automatic IP detection:
```bash
cargo run -- -u <username> -p <password>
```

### Specify IP address:
```bash
cargo run -- -u <username> -p <password> -i <ip_address>
```

### Interactive mode (prompts for username and password):
```bash
cargo run
```

### Get help:
```bash
cargo run -- --help
```

## Command-line Options

- `-u, --username <USERNAME>`: Username (optional, will prompt if not provided)
- `-p, --password <PASSWORD>`: Password (optional, will prompt if not provided)
- `-i, --ip <IP>`: IPV4 Address (optional, will auto-detect if not provided)

## Project Structure

- `src/main.rs` - Entry point and CLI handling
- `src/des_crypto.rs` - DES encryption/decryption implementation
- `src/drcom.rs` - DrCOM protocol handling (network status checking)
- `src/login.rs` - Login logic and form handling

## Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime
- `des` - DES encryption
- `hex` - Hex encoding/decoding
- `serde` - Serialization framework
- `serde_json` - JSON parsing
- `scraper` - HTML parsing
- `clap` - Command-line parsing
- `rpassword` - Secure password input

## Notes

- This tool is designed for DLUT campus network login only
- The tool checks if you're already online before attempting login
- If login fails, it will retry up to 3 times with 3-second intervals
- Requires network connectivity to the campus network

## License

Same as the original Python project
