# TuxBox - Installation Guide

Complete installation guide for all platforms and methods.

---

## 📋 System Requirements

- **Linux**: x86_64 (amd64) architecture
- **macOS**: Apple Silicon (ARM64) or Intel (x86_64 via Rosetta 2)
- **Windows**: Not yet supported (planned)

### Optional Dependencies (for tool execution)
- **Docker**: For containerized tool execution (recommended)
- **Python 3.8+**: For Python tools (fallback if Docker not available)
- **Git**: For cloning tool repositories (usually pre-installed)

---

## 🚀 Installation Methods

### Method 1: Download Pre-built Binary (Recommended)

Pre-compiled binaries are available for Linux and macOS from [GitHub Releases](https://github.com/disoardi/tuxbox/releases).

#### Linux (x86_64)

```bash
# Download latest release
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-linux-amd64.tar.gz -o /tmp/tbox.tar.gz

# Extract
cd /tmp
tar xzf tbox.tar.gz

# Verify extracted binary
ls -lh tbox
# Should show: -rwxr-xr-x ... tbox

# Install (choose option A or B)
```

**Option A: Install to system directory (requires sudo)**
```bash
sudo mv tbox /usr/local/bin/
```

**Option B: Install to user directory (no sudo required)**
```bash
mkdir -p ~/.local/bin
mv tbox ~/.local/bin/

# Add to PATH (choose your shell)
# For bash:
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh:
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### macOS (Apple Silicon & Intel)

```bash
# Download latest release (ARM64 binary works on both architectures)
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-macos-arm64.tar.gz -o /tmp/tbox.tar.gz

# Extract
cd /tmp
tar xzf tbox.tar.gz

# macOS may quarantine the binary, remove quarantine attribute
xattr -d com.apple.quarantine tbox 2>/dev/null || true

# Verify extracted binary
ls -lh tbox

# Install (choose option A or B)
```

**Option A: Install to system directory (requires sudo)**
```bash
sudo mv tbox /usr/local/bin/
```

**Option B: Install to user directory (no sudo required)**
```bash
mkdir -p ~/.local/bin
mv tbox ~/.local/bin/

# Add to PATH (macOS uses zsh by default)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Note for Intel Mac users**: The ARM64 binary runs seamlessly on Intel Macs via Rosetta 2 with minimal performance overhead.

---

### Method 2: Compile from Source

If you have Rust installed (1.80 or later), you can compile TuxBox yourself.

#### Prerequisites

Install Rust using [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify Rust installation:
```bash
rustc --version  # Should show 1.80 or later
cargo --version
```

#### Build and Install

**Option A: Install via cargo install**
```bash
# Clone repository
git clone https://github.com/disoardi/tuxbox.git
cd tuxbox

# Build and install (automatically installed to ~/.cargo/bin/)
cargo install --path .

# Verify installation
tbox --version
```

**Option B: Manual build and install**
```bash
# Clone repository
git clone https://github.com/disoardi/tuxbox.git
cd tuxbox

# Build release binary
cargo build --release

# Binary located at: target/release/tbox
ls -lh target/release/tbox

# Install to system
sudo cp target/release/tbox /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/tbox ~/.local/bin/
```

---

## ✅ Verify Installation

After installation, verify TuxBox is working:

```bash
# Check version
tbox --version
# Expected output: tbox 0.2.0

# Check help
tbox --help
# Should display usage information and commands

# Check PATH
which tbox
# Should show: /usr/local/bin/tbox or ~/.local/bin/tbox
```

---

## 🔄 Updating TuxBox

TuxBox includes a built-in self-update mechanism:

```bash
# Check for available updates
tbox self-update
# Output:
# Checking for updates...
#   → Current version: 0.2.0
#   → Latest version:  0.2.1
# 🎉 New version available: Release v0.2.1
#
# To update, run:
#   tbox self-update --install

# Install update automatically
tbox self-update --install
# Downloads, extracts, and replaces binary
# Backup saved to tbox.bak
```

**Manual update**: Simply re-download the binary using Method 1 instructions.

---

## 🐛 Troubleshooting

### "tbox: command not found"

**Cause**: TuxBox binary is not in your PATH.

**Solution**:
```bash
# Check if binary exists
ls -l ~/.local/bin/tbox

# If exists, add to PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc

# Verify
which tbox
```

### "Permission denied" when running tbox

**Cause**: Binary is not executable.

**Solution**:
```bash
# Make binary executable
chmod +x ~/.local/bin/tbox
# or
chmod +x /usr/local/bin/tbox
```

### macOS: "tbox cannot be opened because it is from an unidentified developer"

**Cause**: macOS Gatekeeper security feature.

**Solution 1: Remove quarantine attribute**
```bash
xattr -d com.apple.quarantine ~/.local/bin/tbox
```

**Solution 2: Allow via System Preferences**
1. Try to run `tbox`
2. Go to System Preferences → Security & Privacy → General
3. Click "Open Anyway" next to the tbox message

### Linux: "error while loading shared libraries"

**Cause**: Missing system libraries (rare, usually on minimal distributions).

**Solution**:
```bash
# Debian/Ubuntu
sudo apt-get install libssl-dev pkg-config

# RHEL/CentOS/Fedora
sudo dnf install openssl-devel pkgconfig

# Then recompile from source (Method 2)
```

### Self-update fails with "Permission denied"

**Cause**: TuxBox installed in system directory but running as non-root user.

**Solution**:
```bash
# Option A: Run with sudo (if installed in /usr/local/bin/)
sudo tbox self-update --install

# Option B: Reinstall to user directory
# Download binary to /tmp
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-macos-arm64.tar.gz -o /tmp/tbox.tar.gz
cd /tmp && tar xzf tbox.tar.gz
mv tbox ~/.local/bin/
# Now self-update works without sudo
```

---

## 🔐 Verifying Binary Authenticity

Each release includes SHA256 checksums:

```bash
# Download checksum
curl -L https://github.com/disoardi/tuxbox/releases/download/v0.2.0/tbox-macos-arm64.tar.gz.sha256 -o checksum.txt

# Verify (macOS)
shasum -a 256 -c checksum.txt

# Verify (Linux)
sha256sum -c checksum.txt
```

---

## 📚 Next Steps

After installation:

1. **Initialize TuxBox** with a registry:
   ```bash
   tbox init git@github.com:your-user/your-registry.git
   ```

2. **Read the Quick Start Guide**: [QUICK_START.md](QUICK_START.md)

3. **Learn about registries**: [REGISTRY_FORMAT.md](REGISTRY_FORMAT.md)

---

## 🆘 Getting Help

- **Documentation**: https://disoardi.github.io/tuxbox
- **Issues**: https://github.com/disoardi/tuxbox/issues
- **Discussions**: https://github.com/disoardi/tuxbox/discussions

---

**Installation successful? Try running your first tool!** 🚀
