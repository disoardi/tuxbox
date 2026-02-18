# 🐧 TuxBox

> A meta-tool to manage and run personal tools from Git repositories with lazy loading

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

## 🎯 What is TuxBox?

TuxBox (`tbox`) is a lightweight CLI tool that helps you manage personal tools scattered across multiple Git repositories. It works like a package manager with lazy loading: clone, configure, and run tools on-demand without manual setup.

### Problem Solved

Instead of manually cloning and configuring tools every time you need them (Ansible playbooks, custom scripts, dotfiles), TuxBox automates the entire workflow:

```bash
tbox run sshmenuc
# → Clones repository if needed
# → Sets up environment (venv for Python, etc.)
# → Runs the tool
```

## 🚀 Quick Start

### Installation

#### Option 1: One-liner Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/disoardi/tuxbox/main/install.sh | sh
```

Installs `tbox` to `~/.local/bin` (no sudo needed). To install system-wide:

```bash
TBOX_INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/disoardi/tuxbox/main/install.sh | sh
```

#### Option 2: Download Pre-built Binary (Manual)

**Linux (x86_64)**
```bash
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-linux-amd64.tar.gz -o /tmp/tbox.tar.gz
cd /tmp && tar xzf tbox.tar.gz

# Install to system (requires sudo)
sudo mv tbox /usr/local/bin/

# Or install to user directory (no sudo)
mkdir -p ~/.local/bin
mv tbox ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

**macOS (Apple Silicon & Intel)**
```bash
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-macos-arm64.tar.gz -o /tmp/tbox.tar.gz
cd /tmp && tar xzf tbox.tar.gz

# Install to system (requires sudo)
sudo mv tbox /usr/local/bin/

# Or install to user directory (no sudo)
mkdir -p ~/.local/bin
mv tbox ~/.local/bin/
# Add to ~/.zshrc: export PATH="$HOME/.local/bin:$PATH"
```

**Note**: The ARM64 binary runs on Intel Macs via Rosetta 2.

#### Option 2: Compile from Source

```bash
# Requires Rust 1.80+
cargo install --path .
```

#### Verify Installation

```bash
tbox --version
# Output: tbox 0.2.0
```

### First Run

```bash
# Initialize TuxBox with a registry
tbox init https://github.com/disoardi/tuxbox-registry

# List available tools
tbox list

# Run a tool (clones automatically if needed)
tbox run <tool-name>
```

### Updating TuxBox

TuxBox can update itself to the latest version:

```bash
# Check for updates
tbox self-update

# Install update automatically (no prompt)
tbox self-update --install
```

## 📖 Features

- ✅ **Lazy Loading**: Download and setup tools only when you need them
- ✅ **Git-based**: Tools live in their own repositories
- ✅ **Isolation Support**: Python venv, Docker containers, or no isolation
- ✅ **Registry System**: Centralized catalog of available tools
- ✅ **Cross-platform**: Linux and macOS support

## 🏗️ Architecture

TuxBox consists of:
1. **Binary** (`tbox`) - The CLI tool
2. **Registry** - TOML files describing available tools
3. **Local Storage** (`~/.tuxbox/`) - Cloned tools and config

## 📚 Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Complete installation instructions
- **[Quick Start Guide](docs/QUICK_START.md)** - Setup and first steps
- **[Registry Format](docs/REGISTRY_FORMAT.md)** - Creating tool registries

Full documentation: [https://disoardi.github.io/tuxbox](https://disoardi.github.io/tuxbox)

## 🛠️ Development

```bash
# Setup pre-commit hook (recommended)
cp .githooks/pre-commit .git/hooks/ && chmod +x .git/hooks/pre-commit

# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- run <tool-name>

# Build release
cargo build --release
```

**Pre-commit Hook**: Automatically formats code with `cargo fmt` before each commit, preventing CI formatting failures. See [.githooks/README.md](.githooks/README.md) for details.

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

## 🙏 Credits

Built with Rust 🦀 and inspired by tools like `mise`, `rustup`, and Homebrew.

---

Made with ❤️ by [Davide Isoardi](https://github.com/disoardi)
