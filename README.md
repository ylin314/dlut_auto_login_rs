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

### Daemon mode (periodically check and login):

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
- `--pid <PATH>`: Path to PID file (to ensure single instance)
- `--info`: Get drcom info and exit
- `--force`: Skip status check before login
- `--daemon`: Daemon mode (periodically check status and login if offline)

## Process Management

### Using PID file

To ensure only one instance is running, use the `--pid` option:

```bash
./dlut_auto_login_rs -u <username> -p <password> --daemon --pid /tmp/dlut_login.pid
```

### Using OpenWrt (procd)

Create a file at `/etc/init.d/dlut_login`:

```bash
#!/bin/sh /etc/rc.common

START=99
USE_PROCD=1

start_service() {
    procd_open_instance
    procd_set_param command /usr/bin/dlut_auto_login_rs -u "YOUR_USER" -p "YOUR_PASS" --daemon
    procd_set_param respawn
    procd_close_instance
}
```

Then enable it:

```bash
chmod +x /etc/init.d/dlut_login
/etc/init.d/dlut_login enable
/etc/init.d/dlut_login start
```

### Using Windows

1.  **Task Scheduler (Recommended)**:
    - Open "Task Scheduler".
    - Create a new task, set trigger to "At log on".
    - Set action to "Start a program", select `dlut_auto_login_rs.exe`.
    - Add arguments: `-u USER -p PASS --daemon --pid C:\path\to\login.pid`.
2.  **Startup Folder**:
    - Press `Win + R`, type `shell:startup`.
    - Create a shortcut to `dlut_auto_login_rs.exe` in this folder.
    - Right-click shortcut -> Properties -> Target, append arguments after the path.

### Using macOS (launchd)

Create `~/Library/LaunchAgents/com.user.dlut_login.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.dlut_login</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/dlut_auto_login_rs</string>
        <string>-u</string>
        <string>YOUR_USER</string>
        <string>-p</string>
        <string>YOUR_PASS</string>
        <string>--daemon</string>
        <string>--pid</string>
        <string>/tmp/dlut_login.pid</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

Then load the task:

```bash
launchctl load ~/Library/LaunchAgents/com.user.dlut_login.plist
```

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
- In daemon mode, the tool periodically checks the network status and logs in if disconnected.

## License

Same as the original Python project
