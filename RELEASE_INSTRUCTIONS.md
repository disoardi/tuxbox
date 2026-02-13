# Istruzioni per Creare la Prima Release di TuxBox

## 🎯 Obiettivo
Creare la prima release pubblica di TuxBox (v0.1.0 o v0.2.0) su GitHub con binari per Linux e macOS.

---

## 📋 Pre-requisiti

- [x] Codice compilato e testato
- [x] GitHub Actions configurato
- [ ] Repository GitHub creato
- [ ] Codice pushato su GitHub
- [ ] README.md aggiornato

---

## 🚀 Step 1: Setup GitHub Repository

### 1.1 Crea Repository su GitHub

```bash
# Su GitHub web:
# - Vai a https://github.com/new
# - Nome: tuxbox
# - Descrizione: "A meta-tool CLI to manage and run personal tools from Git repositories"
# - Public (per release pubbliche e self-update)
# - NON inizializzare con README (abbiamo già il nostro)
```

### 1.2 Configura Remote Git

```bash
cd ~/Progetti/tuxbox

# Aggiungi remote GitHub
git remote add origin https://github.com/disoardi/tuxbox.git

# Verifica
git remote -v
```

### 1.3 Push Initial Code

```bash
# Verifica status
git status

# Commit eventuali modifiche pending
git add .
git commit -m "feat: add self-update mechanism and release workflows

- Implement GitHub API integration for version checking
- Add automatic binary download and replacement
- Create CI/CD workflows for automated releases
- Support Linux x86_64, macOS x86_64, macOS ARM64"

# Push su GitHub
git push -u origin main
```

**Verifica:** Vai su https://github.com/disoardi/tuxbox e controlla che il codice sia presente.

---

## 🏷️ Step 2: Crea Tag per Release

### 2.1 Decidi la Versione

Opzioni:
- **v0.1.0** - Prima versione pubblica (conservative choice)
- **v0.2.0** - Versione con registry system completo

**Raccomandazione:** Usa `v0.2.0` dato che abbiamo già:
- Phase 0 ✅ (MVP base)
- Phase 1 ✅ (Venv support)
- Docker support ✅
- Phase 2 ✅ (Multi-registry)
- Self-update ✅

### 2.2 Aggiorna Versione in Cargo.toml

```bash
# Edita Cargo.toml
sed -i '' 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# O manualmente cambia:
# [package]
# version = "0.2.0"
```

### 2.3 Rebuild per Verificare Nuova Versione

```bash
cargo build --release

# Test versione
./target/release/tbox version
# Output atteso: TuxBox version: 0.2.0
```

### 2.4 Commit Versione Update

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 0.2.0 for first release"
git push
```

### 2.5 Crea Tag Git

```bash
# Crea tag annotato
git tag -a v0.2.0 -m "Release v0.2.0 - First Public Release

Features:
- Multi-registry tool management with SSH/HTTPS support
- Docker-first execution with Python venv fallback
- Auto-setup and lazy loading of tools
- Self-update mechanism via GitHub releases
- Support for private and public registries

Platform Support:
- Linux x86_64
- macOS x86_64
- macOS ARM64 (Apple Silicon)"

# Push tag su GitHub (questo triggera il release workflow!)
git push origin v0.2.0
```

**Cosa succede ora:**
- GitHub Actions rileva il push del tag `v0.2.0`
- Parte il workflow `.github/workflows/release.yml`
- Build automatico per tutte le piattaforme
- Creazione release su GitHub con assets

---

## 🔍 Step 3: Monitora Release Build

### 3.1 Verifica GitHub Actions

```bash
# Vai su GitHub:
# https://github.com/disoardi/tuxbox/actions

# Dovresti vedere un workflow "Release" in esecuzione
# Ci vogliono ~5-10 minuti per completare tutti i build
```

### 3.2 Controlla Build Status

Nel tab Actions, verifica che tutti i job siano green:
- ✅ create-release
- ✅ build-release (ubuntu-latest, x86_64-unknown-linux-gnu)
- ✅ build-release (macos-latest, x86_64-apple-darwin)
- ✅ build-release (macos-latest, aarch64-apple-darwin)

---

## 📦 Step 4: Verifica Release su GitHub

### 4.1 Vai alla Release Page

```bash
# Apri:
# https://github.com/disoardi/tuxbox/releases
```

### 4.2 Controlla Assets

Dovresti vedere la release **v0.2.0** con questi assets:
- `tbox-linux-amd64.tar.gz` + `.sha256`
- `tbox-macos-amd64.tar.gz` + `.sha256`
- `tbox-macos-arm64.tar.gz` + `.sha256`

### 4.3 (Opzionale) Migliora Description Release

Edita la release su GitHub per aggiungere:
- Descrizione dettagliata features
- Installation instructions
- Link a documentazione
- Breaking changes (se presenti)

**Template:**
```markdown
# TuxBox v0.2.0 - First Public Release 🎉

## 🚀 Features

- **Multi-Registry Support**: Manage tools from multiple Git registries with priority-based resolution
- **SSH & HTTPS Auth**: Support for private registries via SSH keys or HTTPS
- **Docker-First Execution**: Automatic containerized execution with Python venv fallback
- **Zero-Config Experience**: Lazy loading, auto-setup, auto-sync
- **Self-Update**: Built-in update mechanism via `tbox self-update`

## 📥 Installation

### Linux (x86_64)
```bash
wget https://github.com/disoardi/tuxbox/releases/download/v0.2.0/tbox-linux-amd64.tar.gz
tar xzf tbox-linux-amd64.tar.gz
sudo mv tbox /usr/local/bin/
```

### macOS (Intel)
```bash
wget https://github.com/disoardi/tuxbox/releases/download/v0.2.0/tbox-macos-amd64.tar.gz
tar xzf tbox-macos-amd64.tar.gz
sudo mv tbox /usr/local/bin/
```

### macOS (Apple Silicon)
```bash
wget https://github.com/disoardi/tuxbox/releases/download/v0.2.0/tbox-macos-arm64.tar.gz
tar xzf tbox-macos-arm64.tar.gz
sudo mv tbox /usr/local/bin/
```

## 📚 Quick Start

```bash
# Initialize with a registry
tbox init git@github.com:your-user/your-registry.git

# List available tools
tbox list

# Run a tool
tbox run <tool-name>

# Check for updates
tbox self-update
```

## 📖 Documentation

- [Quick Start Guide](https://github.com/disoardi/tuxbox/blob/main/docs/QUICK_START.md)
- [Registry Format Reference](https://github.com/disoardi/tuxbox/blob/main/docs/REGISTRY_FORMAT.md)
- [Test Instructions](https://github.com/disoardi/tuxbox/blob/main/TEST_INSTRUCTIONS.md)

## 🔐 Platform Support

- ✅ Linux x86_64
- ✅ macOS x86_64 (Intel)
- ✅ macOS ARM64 (Apple Silicon)

## 🐛 Known Issues

None currently. Please report issues at: https://github.com/disoardi/tuxbox/issues

---

**Full Changelog**: Initial release
```

---

## ✅ Step 5: Test Self-Update (Post-Release)

Dopo che la release è live, testa il self-update:

```bash
# Simula vecchia versione (edita Cargo.toml con versione più bassa)
sed -i '' 's/version = "0.2.0"/version = "0.1.0"/' Cargo.toml
cargo build --release

# Test self-update
./target/release/tbox self-update
# Output:
# Checking for updates...
#   → Current version: 0.1.0
#   → Latest version:  0.2.0
#   🎉 New version available: Release v0.2.0
#
# To update, run:
#   tbox self-update --install

# Test auto-install
./target/release/tbox self-update --install
# Output:
#   Installing update...
#   → Detecting platform: tbox-macos-arm64
#   → Downloading: tbox-macos-arm64.tar.gz
#   ✓ Downloaded 2548352 bytes
#   → Extracting binary...
#   → Replacing binary...
#   ✓ Binary replaced successfully
#   ✓ Update installed successfully!

# Verifica versione
./target/release/tbox version
# TuxBox version: 0.2.0
```

---

## 🎉 Success Checklist

Dopo aver completato tutti gli step:

- [ ] Repository GitHub creato e pubblico
- [ ] Codice pushato su origin/main
- [ ] Tag v0.2.0 creato e pushato
- [ ] GitHub Actions workflow completato con successo
- [ ] Release v0.2.0 visibile su GitHub con assets
- [ ] Self-update testato e funzionante
- [ ] README.md aggiornato con installation instructions
- [ ] Documentazione linkata nella release

---

## 🔄 Workflow per Release Future

Per rilasciare versioni successive:

```bash
# 1. Implementa feature/fix
# 2. Commit changes
git add .
git commit -m "feat: new awesome feature"

# 3. Bump version in Cargo.toml
# Segui Semantic Versioning:
# - MAJOR.MINOR.PATCH
# - MAJOR: breaking changes
# - MINOR: new features (backward compatible)
# - PATCH: bug fixes

# 4. Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to X.Y.Z"
git push

# 5. Create and push tag
git tag -a vX.Y.Z -m "Release vX.Y.Z - Description"
git push origin vX.Y.Z

# 6. GitHub Actions fa il resto automaticamente!
```

---

## 📚 Riferimenti

- **GitHub Actions Docs**: https://docs.github.com/en/actions
- **Semantic Versioning**: https://semver.org/
- **Rust Release Best Practices**: https://doc.rust-lang.org/cargo/reference/publishing.html

---

**Data creazione:** 2026-02-13
**Versione target:** v0.2.0
**Status:** Ready to execute
