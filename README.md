# Repostistry 🔄

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/Rust-1.65%2B-orange)](https://www.rust-lang.org)
[![Build Status](https://github.com/yourusername/repostistry/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/repostistry/actions)

**Repostistry** is a lightweight port monitoring tool written in Rust that displays active network connections with beautiful terminal formatting.

```plaintext
┌──────────────────────────────────────────────────────────┐
│                  🚦 Network Port Monitor                 │
│                 2023-11-15 14:30:45 UTC                 │
├──────────────────┬──────────┬──────────────────┬─────────┤
│ PORT             │ PID      │ PROCESS          │ STATE   │
├──────────────────┼──────────┼──────────────────┼─────────┤
│ 22 (tcp)         │ 1234     │ sshd             │ LISTEN  │
│ 80 (tcp)         │ 4567     │ nginx            │ LISTEN  │
│ 443 (tcp)        │ 4567     │ nginx            │ LISTEN  │
│ 53 (udp)         │ 8910     │ dnsmasq          │ ACTIVE  │
└──────────────────┴──────────┴──────────────────┴─────────┘
```

## ✨ Features

- Real-time port monitoring
- Color-coded output (customizable)
- Process name resolution
- Lightweight (~2MB binary)
- Cross-platform (Linux/macOS)
- Watch mode for continuous monitoring

## 📦 Installation

### From Source

```bash
cargo install --git https://github.com/yourusername/repostistry.git
```

### Pre-built Binaries

Download the latest binary from the [Releases](https://github.com/yourusername/repostistry/releases) page and add it to your `PATH`.

## 🛠️ Usage

### Basic scan

```bash
repostistry
```

### Watch mode (refresh every 5s)

```bash
repostistry --watch 5
```

### TCP only

```bash
repostistry --tcp
```

### UDP only

```bash
repostistry --udp
```

### No colors

```bash
repostistry --no-color
```

## 🖥️ Examples

### Find specific service

```bash
repostistry | grep ssh
```

### Monitor web ports

```bash
repostistry --tcp | grep -E '80|443'
```

### Save to file

```bash
repostistry --no-color > ports_$(date +%F).log
```

## 🏗️ Building

### Requirements

- Rust 1.65+
- Cargo

### Steps

```bash
git clone https://github.com/yourusername/repostistry.git
cd repostistry
cargo build --release
```

The binary will be located at:

```
target/release/repostistry
```

## ⚙️ Configuration

You can create a configuration file at `~/.config/repostistry/config.toml`:

```toml
[display]
colors = true       # Enable colored output
refresh_rate = 2    # Default watch refresh rate (seconds)
ignore_ports = [53] # Ports to hide from output
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
