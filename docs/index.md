# TuxBox Documentation

> A meta-tool CLI to manage and run personal tools from Git repositories with lazy loading

[![GitHub release](https://img.shields.io/github/v/release/disoardi/tuxbox)](https://github.com/disoardi/tuxbox/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI Status](https://github.com/disoardi/tuxbox/workflows/CI/badge.svg)](https://github.com/disoardi/tuxbox/actions)

## 🚀 Quick Start

### Installation

**Linux (x86_64)**
```bash
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-linux-amd64.tar.gz -o /tmp/tbox.tar.gz
cd /tmp && tar xzf tbox.tar.gz
sudo mv tbox /usr/local/bin/
```

**macOS (Apple Silicon & Intel)**
```bash
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-macos-arm64.tar.gz -o /tmp/tbox.tar.gz
cd /tmp && tar xzf tbox.tar.gz
sudo mv tbox /usr/local/bin/
```

**Alternative: Install without sudo**
```bash
# After extraction, install to user directory
mkdir -p ~/.local/bin
mv tbox ~/.local/bin/

# Add to PATH (add this line to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

**Verify Installation**
```bash
tbox --version  # Should show: tbox 0.2.0
```

### First Run

```bash
# Initialize with a registry
tbox init git@github.com:your-user/your-registry.git

# List available tools
tbox list

# Run a tool
tbox run <tool-name>
```

## 📋 Features

- **Multi-Registry Support**: Manage tools from multiple Git registries with priority-based resolution
- **SSH & HTTPS Auth**: Support for private registries via SSH keys or HTTPS tokens
- **Docker-First Execution**: Automatic containerized execution with Python venv fallback
- **Zero-Config Experience**: Lazy loading, auto-setup, auto-sync - no manual configuration needed
- **Self-Update**: Built-in update mechanism via `tbox self-update`
- **Smart Tool Management**: Automatic dependency detection and installation

## 📚 Documentation

### Getting Started
- [Installation Guide](INSTALLATION.md) - Complete installation instructions for all platforms
- [Quick Start Guide](QUICK_START.md) - Complete setup and first steps

### Core Concepts
- [Registry Format Reference](REGISTRY_FORMAT.md) - How to create and manage tool registries
- [Distribution Strategy](DISTRIBUTION_STRATEGY.md) - Architecture and distribution approaches

### Advanced
- [Release Instructions](../RELEASE_INSTRUCTIONS.md) - How to create releases (for maintainers)

## 🎯 Use Cases

### Personal Tool Management
Centralize all your personal CLI tools in a private registry:
```bash
tbox init git@github.com:you/my-tools-registry.git
tbox run deploy-script
tbox run backup-tool
tbox run custom-linter
```

### Team Tool Distribution
Share team tools without complex installation procedures:
```bash
# Team lead sets up registry once
tbox init git@github.company.com:team/tools.git

# Team members just run tools
tbox run deploy --env production
tbox run test-runner --parallel
```

### Multi-Source Tools
Combine tools from multiple registries with priority control:
```bash
# Company tools (high priority)
tbox registry add company git@github.company.com:tools.git --priority 200

# Personal tools (medium priority)
tbox registry add personal git@github.com:me/tools.git --priority 150

# Community tools (fallback)
tbox registry add community https://github.com/tuxbox/registry-public.git --priority 100
```

## 🔧 Commands

| Command | Description |
|---------|-------------|
| `tbox init <url>` | Initialize TuxBox with a registry |
| `tbox list` | List available tools |
| `tbox run <tool> [args]` | Run a tool |
| `tbox status` | Show TuxBox status |
| `tbox update [tool]` | Update tool(s) |
| `tbox registry list` | List configured registries |
| `tbox registry add <name> <url>` | Add a new registry |
| `tbox registry sync` | Sync all registries |
| `tbox self-update` | Update TuxBox itself |
| `tbox version` | Show TuxBox version |

## 🏗️ Architecture

TuxBox follows a **registry-based architecture**:

```
┌─────────────────────────────────────┐
│          TuxBox CLI                 │
│  (Rust binary: tbox)                │
└────────────┬────────────────────────┘
             │
             ├─→ Multi-Registry Manager
             │   ├─→ SSH Authentication
             │   ├─→ HTTPS Authentication
             │   └─→ Priority Resolution
             │
             ├─→ Execution Engine
             │   ├─→ Docker (primary)
             │   └─→ Python venv (fallback)
             │
             └─→ Self-Update System
                 └─→ GitHub Releases API
```

### Execution Flow

1. **Tool Request**: User runs `tbox run tool-name`
2. **Registry Resolution**: TuxBox searches registries by priority
3. **Auto-Clone**: Tool repository cloned if not present
4. **Auto-Setup**: Dependencies installed automatically
5. **Execution**: Tool runs in isolated environment (Docker or venv)
6. **Caching**: Tool cached for instant subsequent runs

## 🔐 Security

- **SSH Key Authentication**: Use your existing SSH keys for private registries
- **HTTPS Token Support**: Optional token-based authentication
- **Isolated Execution**: Tools run in containers or virtual environments
- **No Credential Storage**: TuxBox uses system Git credentials

## 📦 Platform Support

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64 | ✅ Supported |
| macOS | ARM64 (Apple Silicon) | ✅ Supported |
| macOS | x86_64 (Intel via Rosetta) | ✅ Supported |
| Windows | x86_64 | 🚧 Planned |

## 🤝 Contributing

Contributions are welcome! Please see:
- [Issues](https://github.com/disoardi/tuxbox/issues) for bugs and feature requests
- [Pull Requests](https://github.com/disoardi/tuxbox/pulls) for contributions

## 📜 License

TuxBox is released under the [MIT License](https://github.com/disoardi/tuxbox/blob/main/LICENSE).

## 🔗 Links

- [GitHub Repository](https://github.com/disoardi/tuxbox)
- [Latest Release](https://github.com/disoardi/tuxbox/releases/latest)
- [Issue Tracker](https://github.com/disoardi/tuxbox/issues)
- [Changelog](https://github.com/disoardi/tuxbox/releases)

---

**Made with ❤️ and Rust 🦀**
