#!/usr/bin/env bash
# TuxBox installer
# Usage: curl -fsSL https://raw.githubusercontent.com/disoardi/tuxbox/main/install.sh | sh

set -e

REPO="disoardi/tuxbox"
BINARY="tbox"
DEFAULT_INSTALL_DIR="$HOME/.local/bin"

# Colors (only when stdout is a terminal).
# Variables hold the literal escape string (e.g. '\033[1m').
# They MUST be used inside the printf FORMAT string (not as %s arguments)
# so that printf processes the \033 octal sequence. See helper functions below.
if [ -t 1 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    RED='' GREEN='' YELLOW='' CYAN='' BOLD='' NC=''
fi

# Color vars are intentionally used in format strings (printf processes \033 escapes there)
# shellcheck disable=SC2059
status()    { printf "${CYAN}→${NC} %s\n" "$*"; }
# shellcheck disable=SC2059
success()   { printf "${GREEN}✓${NC} %s\n" "$*"; }
# shellcheck disable=SC2059
warn()      { printf "${YELLOW}⚠${NC} %s\n" "$*"; }
# shellcheck disable=SC2059
error()     { printf "${RED}✗${NC} %s\n" "$*" >&2; exit 1; }
# shellcheck disable=SC2059
bold_line() { printf "  ${BOLD}%s${NC}\n" "$*"; }

# shellcheck disable=SC2059
printf "\n${BOLD}TuxBox Installer${NC}\n"
printf '━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n'

# ── Detect OS ────────────────────────────────────────────────────────────────

OS=$(uname -s)
case "$OS" in
    Linux*)  PLATFORM="linux" ;;
    Darwin*) PLATFORM="macos" ;;
    *)       error "Unsupported OS: $OS (only Linux and macOS are supported)" ;;
esac

# ── Detect architecture ───────────────────────────────────────────────────────

ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64)
        if [ "$PLATFORM" = "macos" ]; then
            # macOS Intel: no native build, ARM64 binary runs via Rosetta 2
            ARCH_TAG="arm64"
            status "macOS Intel detected — using ARM64 binary (runs via Rosetta 2)"
        else
            ARCH_TAG="amd64"
        fi
        ;;
    aarch64|arm64) ARCH_TAG="arm64" ;;
    *) error "Unsupported architecture: $ARCH" ;;
esac

ASSET_NAME="tbox-${PLATFORM}-${ARCH_TAG}"
status "Platform: ${PLATFORM}/${ARCH_TAG}"

# ── Check dependencies ────────────────────────────────────────────────────────

if command -v curl >/dev/null 2>&1; then
    fetch() { curl -fsSL "$1"; }
    download() { curl -fsSL --progress-bar "$1" -o "$2"; }
elif command -v wget >/dev/null 2>&1; then
    fetch() { wget -qO- "$1"; }
    download() { wget -q --show-progress "$1" -O "$2"; }
else
    error "curl or wget is required"
fi

# ── Fetch latest version ───────────────────────────────────────────────────────

status "Fetching latest version..."
VERSION=$(fetch "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' \
    | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
[ -z "$VERSION" ] && error "Could not determine latest version (GitHub API rate limit?)"
success "Latest version: ${VERSION}"

# ── Determine install directory ────────────────────────────────────────────────
# Override with: TBOX_INSTALL_DIR=/usr/local/bin curl ... | sh

INSTALL_DIR="${TBOX_INSTALL_DIR:-}"
if [ -z "$INSTALL_DIR" ]; then
    if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="$DEFAULT_INSTALL_DIR"
    fi
fi

# ── Download ───────────────────────────────────────────────────────────────────

BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
TARBALL_URL="${BASE_URL}/${ASSET_NAME}.tar.gz"
CHECKSUM_URL="${BASE_URL}/${ASSET_NAME}.tar.gz.sha256"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

status "Downloading ${ASSET_NAME} ${VERSION}..."
download "$TARBALL_URL"  "$TMP_DIR/tbox.tar.gz"
download "$CHECKSUM_URL" "$TMP_DIR/tbox.tar.gz.sha256"

# ── Verify checksum ────────────────────────────────────────────────────────────

status "Verifying checksum..."
EXPECTED=$(awk '{print $1}' "$TMP_DIR/tbox.tar.gz.sha256")
if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$TMP_DIR/tbox.tar.gz" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
    ACTUAL=$(shasum -a 256 "$TMP_DIR/tbox.tar.gz" | awk '{print $1}')
else
    warn "sha256sum/shasum not found — skipping checksum verification"
    ACTUAL="$EXPECTED"
fi
[ "$ACTUAL" != "$EXPECTED" ] && error "Checksum mismatch! Expected: $EXPECTED  Got: $ACTUAL"
success "Checksum OK"

# ── Install ────────────────────────────────────────────────────────────────────

tar xzf "$TMP_DIR/tbox.tar.gz" -C "$TMP_DIR"

mkdir -p "$INSTALL_DIR"
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
elif command -v sudo >/dev/null 2>&1; then
    status "Writing to ${INSTALL_DIR} requires sudo..."
    sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
else
    error "Cannot write to ${INSTALL_DIR} (no write permission and sudo not available)"
fi
chmod +x "$INSTALL_DIR/$BINARY"

# ── Verify ─────────────────────────────────────────────────────────────────────

INSTALLED_VERSION=$("$INSTALL_DIR/$BINARY" --version 2>/dev/null || echo "unknown")
success "Installed: ${INSTALLED_VERSION}  →  ${INSTALL_DIR}/${BINARY}"

# ── PATH setup ─────────────────────────────────────────────────────────────────
# Auto-add to shell RC file if installing to ~/.local/bin and not already in PATH

PATH_ADDED=0
if [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
    # Detect user's default shell and RC file
    SHELL_NAME=$(basename "${SHELL:-bash}")
    case "$SHELL_NAME" in
        zsh)  RC_FILE="$HOME/.zshrc" ;;
        bash) RC_FILE="$HOME/.bashrc" ;;
        fish) RC_FILE="$HOME/.config/fish/config.fish" ;;
        ksh)  RC_FILE="$HOME/.kshrc" ;;
        *)    RC_FILE="$HOME/.profile" ;;
    esac

    # Check if ~/.local/bin is already in PATH or already in the RC file
    case ":$PATH:" in
        *":$HOME/.local/bin:"*) : ;;  # already in current PATH, nothing to do
        *)
            if [ "$SHELL_NAME" = "fish" ]; then
                FISH_LINE="fish_add_path $HOME/.local/bin"
                if ! grep -qF "fish_add_path" "$RC_FILE" 2>/dev/null; then
                    mkdir -p "$(dirname "$RC_FILE")"
                    printf '\n# Added by TuxBox installer\n' >> "$RC_FILE"
                    printf '%s\n' "$FISH_LINE" >> "$RC_FILE"
                    success "Added ~/.local/bin to PATH in $RC_FILE"
                    PATH_ADDED=1
                fi
            else
                if ! grep -qF '.local/bin' "$RC_FILE" 2>/dev/null; then
                    printf '\n# Added by TuxBox installer\n' >> "$RC_FILE"
                    # Intentional: write literal $HOME/$PATH to RC file (expands when user sources it)
                    # shellcheck disable=SC2016
                    printf '%s\n' 'export PATH="$HOME/.local/bin:$PATH"' >> "$RC_FILE"
                    success "Added ~/.local/bin to PATH in $RC_FILE"
                    PATH_ADDED=1
                fi
            fi
            ;;
    esac
fi

if [ "$PATH_ADDED" -eq 1 ]; then
    printf '\n'
    warn "Reload your shell to activate PATH:"
    # shellcheck disable=SC2059
    printf "  ${BOLD}source %s${NC}\n" "$RC_FILE"
elif ! command -v "$BINARY" >/dev/null 2>&1; then
    printf '\n'
    warn "${INSTALL_DIR} is not in your PATH."
    # shellcheck disable=SC2016,SC2059
    printf "  Add manually: ${BOLD}%s${NC}\n" 'export PATH="$HOME/.local/bin:$PATH"'
fi

# ── Done ───────────────────────────────────────────────────────────────────────

# shellcheck disable=SC2059
printf "\n${GREEN}${BOLD}TuxBox is ready!${NC}\n\n"
bold_line "tbox init https://github.com/disoardi/tuxbox-registry"
bold_line "tbox list"
bold_line "tbox run <tool-name>"
printf '\n'
