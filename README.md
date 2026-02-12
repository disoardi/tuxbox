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

```bash
# From source (requires Rust 1.80+)
cargo install --path .

# Or download binary from releases
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-linux -o tbox
chmod +x tbox
sudo mv tbox /usr/local/bin/
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

Full documentation available at: [https://disoardi.github.io/tuxbox](https://disoardi.github.io/tuxbox)

## 🛠️ Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- run <tool-name>

# Build release
cargo build --release
```

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

## 🙏 Credits

Built with Rust 🦀 and inspired by tools like `mise`, `rustup`, and Homebrew.

---

Made with ❤️ by [Davide Isoardi](https://github.com/disoardi)
