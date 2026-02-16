# TuxBox - Quick Start Guide

Guida rapida per iniziare a usare TuxBox con un registry SSH privato.

---

## 📦 Installazione

### Metodo 1: Download Binary Pre-compilato (RACCOMANDATO)

#### Linux (x86_64)

```bash
# Download latest release
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-linux-amd64.tar.gz -o /tmp/tbox.tar.gz

# Extract
cd /tmp && tar xzf tbox.tar.gz

# Install to system directory (richiede sudo)
sudo mv tbox /usr/local/bin/

# Verify installation
tbox --version
```

#### macOS (Apple Silicon & Intel)

```bash
# Download latest release (ARM64 binary runs on Intel via Rosetta 2)
curl -L https://github.com/disoardi/tuxbox/releases/latest/download/tbox-macos-arm64.tar.gz -o /tmp/tbox.tar.gz

# Extract
cd /tmp && tar xzf tbox.tar.gz

# Install to system directory (richiede sudo)
sudo mv tbox /usr/local/bin/

# Verify installation
tbox --version
```

### Metodo 2: Install senza sudo (user directory)

Se non hai permessi sudo o preferisci installare solo per il tuo utente:

```bash
# Create user bin directory
mkdir -p ~/.local/bin

# Move binary (after extraction)
mv /tmp/tbox ~/.local/bin/

# Add to PATH (aggiungi a ~/.bashrc o ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc  # Reload shell config

# Verify installation
tbox --version
```

### Metodo 3: Compile from Source

Se hai Rust installato (1.80+) e vuoi compilare tu stesso:

```bash
# Clone repository
git clone https://github.com/disoardi/tuxbox.git
cd tuxbox

# Build and install
cargo install --path .

# Binary installed in ~/.cargo/bin/tbox
# Make sure ~/.cargo/bin is in your PATH

# Verify installation
tbox --version
```

### Verifica Installazione

```bash
# Check version
tbox --version
# Expected output: tbox 0.2.0

# Check help
tbox --help
# Should show all available commands
```

---

## 📋 Prerequisiti

### 1. SSH Passwordless Setup (OBBLIGATORIO per registry privati)

Il tuo sistema deve essere configurato per accedere a GitHub/GitLab via SSH senza password.

#### Verifica SSH Setup

```bash
# Test connessione SSH (deve funzionare senza chiedere password)
ssh -T git@github.dxc.com
# O per GitHub pubblico:
ssh -T git@github.com

# Output atteso: "Hi username! ..."
```

#### Se SSH non funziona, configura le chiavi:

```bash
# 1. Genera chiave SSH (se non l'hai già)
ssh-keygen -t ed25519 -C "tua-email@example.com"
# Premi Enter per accettare il percorso default (~/.ssh/id_ed25519)
# IMPORTANTE: Non impostare password sulla chiave per uso automatico

# 2. Aggiungi la chiave all'SSH agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# 3. Copia la chiave pubblica
cat ~/.ssh/id_ed25519.pub
# Copia l'output e aggiungilo a GitHub/GitLab:
# - GitHub: Settings → SSH and GPG keys → New SSH key
# - GitLab: Preferences → SSH Keys → Add key

# 4. (Opzionale) Configura SSH config per GitHub Enterprise
cat >> ~/.ssh/config << 'EOF'
Host github.dxc.com
    IdentityFile ~/.ssh/id_ed25519
    User git
EOF

# 5. Testa di nuovo
ssh -T git@github.dxc.com
```

**⚠️ IMPORTANTE:** La chiave SSH **NON deve avere password** per funzionare automaticamente con TuxBox.

---

## 🚀 Uso di TuxBox (Prima Volta)

### Scenario: Usare un registry esistente

```bash
# 1. Inizializza TuxBox con il registry SSH
tbox init git@github.dxc.com:disoardi/tuxbox-registry-private.git

# Output:
# → Initializing TuxBox...
# ✓ TuxBox initialized successfully!

# 2. Verifica configurazione
tbox status

# Output:
# TuxBox Status
# =============
# Base directory: /Users/username/.tuxbox
# Tools directory: /Users/username/.tuxbox/tools
#
# Registries: (1 configured):
#   🔐 tuxbox-registry-private (priority: 100) - git@github.dxc.com:...

# 3. (Opzionale) Sincronizza registry manualmente
tbox registry sync

# Output:
# → Registry: tuxbox-registry-private
#   → Cloning registry...
#   ✓ Registry cloned successfully
#   ✓ 1 tools available

# 4. Lista tools disponibili
tbox list

# Output:
# Configured registries:
#   🔐 tuxbox-registry-private (priority: 100)
#
# Available tools from registries:
#   • sshmenuc - SSH connection manager (from tuxbox-registry-private)

# 5. Esegui tool (tutto automatico!)
tbox run sshmenuc

# TuxBox farà automaticamente:
# - Clone del registry (se non già fatto)
# - Clone del tool
# - Setup dipendenze (Docker/venv)
# - Esecuzione
```

---

## 📦 Creare il Proprio Registry

### Step 1: Crea repository Git

```bash
# Su GitHub/GitLab, crea un nuovo repository:
# - Nome: tuxbox-registry-private (o altro nome)
# - Visibilità: Private (per repository privato)
# - NON inizializzare con README

# Poi localmente:
mkdir -p ~/Progetti/tuxbox-registry-private
cd ~/Progetti/tuxbox-registry-private
git init
git branch -M main
```

### Step 2: Crea tools.toml

```bash
cat > tools.toml << 'EOF'
# TuxBox Registry - Personal Tools

[tools.sshmenuc]
name = "sshmenuc"
repo = "https://github.com/disoardi/sshmenuc"
branch = "main"
version = "1.1.0"
type = "python"
description = "SSH connection manager with interactive TUI"

[tools.sshmenuc.commands]
run = "python3 -m sshmenuc"

[tools.sshmenuc.dependencies]
python = ">=3.8"
requirements = "requirements.txt"

# Aggiungi altri tool qui...
# [tools.altro-tool]
# name = "altro-tool"
# ...
EOF
```

### Step 3: Crea README

```bash
cat > README.md << 'EOF'
# TuxBox Registry - Personal Tools

Registry privato per tool personali gestiti con TuxBox.

## Setup

```bash
# Inizializza TuxBox con questo registry
tbox init git@github.com:user/tuxbox-registry-private.git

# Lista tools
tbox list

# Esegui tool
tbox run sshmenuc
```

## Aggiungere Tool

Edita `tools.toml` e aggiungi la definizione del tool, poi:

```bash
git add tools.toml
git commit -m "Add nuovo-tool"
git push
```
EOF
```

### Step 4: Push su Git

```bash
# Commit iniziale
git add .
git commit -m "Initial commit: personal tools registry"

# Collega al remote (usa URL SSH!)
git remote add origin git@github.com:user/tuxbox-registry-private.git

# Push
git push -u origin main
```

### Step 5: Usa il Registry

```bash
# Inizializza TuxBox
tbox init git@github.com:user/tuxbox-registry-private.git

# Esegui tool
tbox run sshmenuc
```

---

## 📝 Formato tools.toml

### Struttura Base

```toml
[tools.<tool-name>]
name = "<tool-name>"                    # REQUIRED: Nome tool
repo = "<git-url>"                      # REQUIRED: URL repository Git
branch = "main"                         # OPTIONAL: Branch (default: main)
version = "1.0.0"                       # OPTIONAL: Versione tool
type = "python"                         # OPTIONAL: python, bash, node, etc.
description = "Tool description"        # OPTIONAL: Descrizione

[tools.<tool-name>.commands]
run = "python3 -m tool"                 # REQUIRED: Comando esecuzione
setup = "pip install -e ."              # OPTIONAL: Comando setup

[tools.<tool-name>.dependencies]
python = ">=3.8"                        # OPTIONAL: Versione Python richiesta
requirements = "requirements.txt"       # OPTIONAL: File requirements
```

### Esempio Completo: Tool Python con pyproject.toml

```toml
[tools.mytool]
name = "mytool"
repo = "git@github.com:user/mytool.git"
branch = "main"
version = "2.0.0"
type = "python"
description = "My awesome Python tool"

[tools.mytool.commands]
run = "python3 -m mytool"
setup = "pip install -e ."

[tools.mytool.dependencies]
python = ">=3.9"
```

### Esempio: Tool Bash

```toml
[tools.deploy-script]
name = "deploy-script"
repo = "git@github.com:user/deploy-script.git"
type = "bash"
description = "Deployment automation script"

[tools.deploy-script.commands]
run = "./deploy.sh"
setup = "chmod +x deploy.sh"
```

---

## 🔧 Comandi TuxBox

```bash
# Setup e configurazione
tbox init <registry-url>              # Inizializza con registry
tbox status                           # Mostra stato TuxBox

# Gestione registry
tbox registry list                    # Lista registry configurati
tbox registry add <name> <url> -p N   # Aggiungi registry con priorità
tbox registry remove <name>           # Rimuovi registry
tbox registry sync                    # Sincronizza tutti i registry

# Gestione tools
tbox list                             # Lista tools disponibili
tbox run <tool> [args...]             # Esegui tool
tbox update [tool]                    # Aggiorna tool installato

# Self-update TuxBox
tbox self-update                      # Check e mostra update disponibile
tbox self-update --install            # Scarica e installa update
```

---

## 🎯 Multi-Registry Setup

TuxBox supporta registri multipli con risoluzione basata su priorità.

```bash
# Registry privato aziendale (priority 200 - checked first)
tbox registry add work git@github.dxc.com:company/tools.git --priority 200

# Registry personale (priority 150)
tbox registry add personal git@github.com:user/my-tools.git --priority 150

# Registry pubblico community (priority 100 - fallback)
tbox registry add community https://github.com/tuxbox/registry-public.git --priority 100

# Lista registry
tbox registry list

# Quando esegui un tool, TuxBox cerca nei registry in ordine di priorità
tbox run some-tool
```

---

## 🐛 Troubleshooting

### Errore: "Failed to clone registry via SSH"

**Causa:** SSH non configurato correttamente o chiave con password.

**Soluzione:**
```bash
# 1. Verifica SSH
ssh -T git@github.com
# Deve funzionare senza chiedere password!

# 2. Se chiede password, rimuovi password dalla chiave:
ssh-keygen -p -f ~/.ssh/id_ed25519
# Premi Enter quando chiede "Enter new passphrase" (lascia vuoto)

# 3. Aggiungi chiave all'agent
ssh-add ~/.ssh/id_ed25519

# 4. Riprova
tbox registry sync
```

### Errore: "Tool not found in registry"

**Causa:** Registry non sincronizzato o tool non definito in tools.toml.

**Soluzione:**
```bash
# Sincronizza registry
tbox registry sync

# Verifica tools disponibili
tbox list

# Se tool manca, aggiungilo al tools.toml nel repository registry
```

### Warning: "git2 failed, trying system git command"

**Non è un errore!** TuxBox usa automaticamente fallback al comando `git` di sistema quando git2 (libreria Rust) non funziona. È normale con GitHub Enterprise o configurazioni SSH custom.

---

## 📚 Documentazione Completa

- [Registry Format Reference](REGISTRY_FORMAT.md) - Formato dettagliato tools.toml
- [Distribution Strategy](DISTRIBUTION_STRATEGY.md) - Strategie distribuzione
- [Architecture Notes](../.claude/architecture-notes.md) - Deep dive architettura

---

## ✅ Checklist Setup Completo

- [ ] SSH configurato e funzionante (`ssh -T git@github.com` senza password)
- [ ] TuxBox installato (`cargo install tuxbox` o `cargo build --release`)
- [ ] Registry creato su GitHub/GitLab (privato con SSH)
- [ ] tools.toml creato con almeno un tool
- [ ] Registry pushato su remote
- [ ] TuxBox inizializzato (`tbox init <registry-url>`)
- [ ] Registry sincronizzato (`tbox registry sync`)
- [ ] Tool eseguito con successo (`tbox run <tool>`)

---

**Setup completato!** 🚀 Ora puoi gestire tutti i tuoi tool da un'unica interfaccia.

Per domande o problemi: https://github.com/disoardi/tuxbox/issues
