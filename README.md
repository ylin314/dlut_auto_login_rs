# DLUT Auto Login - Rust Version

Rust implementation of the DLUT (Dalian University of Technology) campus network auto-login tool.

## Features

- DES/3DES encryption support for login credentials
- Automatic network status detection
- Lightweight CLI for embedded systems (OpenWrt)
- Synchronous HTTP requests (minimal binary size)
- Auto-retry mechanism for unattended operation

## Build

```bash
cargo build --release
# Optional: Strip symbols for smaller binary
strip target/release/dlut_auto_login_rs
```

## Usage

### Basic usage (one-time login):
```bash
./dlut_auto_login_rs -u <username> -p <password>
```

### Get current network status:
```bash
./dlut_auto_login_rs --info
```

### Force login (skip status check):
```bash
./dlut_auto_login_rs -u <username> -p <password> --force
```

### Keep-alive mode (retry loop):
```bash
./dlut_auto_login_rs -u <username> -p <password> --daemon
```

### Specify IP address:
```bash
./dlut_auto_login_rs -u <username> -p <password> -i <ip_address>
```

### Get help:
```bash
./dlut_auto_login_rs --help
```

## Command-line Options

- `-u, --username <USERNAME>`: Username
- `-p, --password <PASSWORD>`: Password
- `-i, --ip <IP>`: IPV4 Address (optional, will auto-detect if not provided)
- `--info`: Get drcom info and exit
- `--force`: Skip status check before login
- `--daemon`: Keep-alive mode (retry loop until success)

## Project Structure

- `src/main.rs` - Entry point and CLI handling
- `src/des_crypto.rs` - DES encryption/decryption implementation
- `src/drcom.rs` - DrCOM protocol handling (network status checking)
- `src/login.rs` - Login logic and form handling

## Dependencies

- `ureq` - Minimal synchronous HTTP client
- `des` - DES encryption
- `hex` - Hex encoding/decoding
- `serde` - Serialization framework
- `serde_json` - JSON parsing
- `argh` - Minimal command-line parsing

## Notes

- This tool is optimized for embedded systems (e.g., OpenWrt)
- Interactive mode has been removed for daemon/script usage
- The tool checks if you're already online before attempting login
- If login fails, it will retry indefinitely until success (useful for startup scripts)

## License

Same as the original Python project
