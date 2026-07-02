# bully-rs

`bully-rs` is a high-performance, asynchronous WPS (Wi-Fi Protected Setup) PIN cracking tool written in Rust. It serves as a modern, safe rewrite of the classic `bully` tool, designed to leverage Rust's memory safety and concurrency primitives for improved efficiency in Wi-Fi security auditing.

## Features

- **High-Performance Asynchronous I/O:** Built with `tokio` to handle packet exchanges efficiently.
- **Multiple Attack Vectors:**
  - **Pixie-Dust Attack:** Rapidly recover WPS PINs from vulnerable devices.
  - **Brute-force/Sequential:** Systematic testing of WPS PINs.
- **Raw Socket Manipulation:** Utilizes direct raw socket access for fine-grained control over WPA/WPS frames.
- **Modern CLI:** Structured with `clap` for a familiar and user-friendly interface.

## Prerequisites

- **OS:** Linux (requires `AF_PACKET` raw socket support).
- **Permissions:** Root/sudo access is required to open raw sockets for packet injection and sniffing.
- **Dependencies:** `rustc` and `cargo` installed.

## Building

To build the tool from source, clone the repository and use `cargo`:

```bash
git clone https://github.com/kimocoder/bully-rs.git
cd bully-rs
cargo build --release
```

The resulting binary will be located at `target/release/bully-rs`.

## Usage

```bash
# Display help
sudo ./target/release/bully-rs --help

# Example: Run a basic attack on an interface
sudo ./target/release/bully-rs --interface wlan0 --bssid AA:BB:CC:DD:EE:FF
```

### CLI Options

- `--interface <IFACE>`: Wireless interface to use (must be in monitor mode).
- `--scan`: Scan for WPS-enabled access points.
- `--bssid <MAC>`: Target BSSID.
- `--pixie`: Enable Pixie-Dust attack mode.
- `--bruteforce`: Enable brute-force mode.
- `--pin <PIN>`: Test a specific PIN.
- `--verbosity <LEVEL>`: Set log level (0-3).

## Disclaimer

This tool is designed for educational purposes and authorized security auditing only. Using this software against targets you do not have explicit permission to test is illegal and unethical. The authors assume no liability for misuse.
