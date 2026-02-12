# TuxBox - CLI Reference

## 📖 Comandi Disponibili

### `tbox --help`
Mostra help generale con lista comandi

**Output atteso:**
```
TuxBox - Meta-tool manager for distributed Git tools

Usage: tbox <COMMAND>

Commands:
  init    Initialize TuxBox with a registry URL
  list    List all available tools
  run     Run a specific tool
  update  Update one or all tools
  status  Show TuxBox status
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

### `tbox init <REGISTRY_URL>`
Inizializza TuxBox con URL registry

**Status:** 🚧 NOT IMPLEMENTED in Phase 0 (comando esiste ma noop)

**Usage futuro:**
```bash
tbox init https://github.com/disoardi/tuxbox-registry.git
```

**Comportamento previsto:**
1. Clone registry in ~/.tuxbox/registry/
2. Parse tools.toml o tools.yaml
3. Crea struttura directories
4. Salva config

**File creati:**
- `~/.tuxbox/config.toml` (registry URL, settings)
- `~/.tuxbox/registry/` (clone del registry repo)

**Error handling:**
- Registry URL invalid
- Network failure
- Registry format invalid

---

### `tbox list`
Lista tutti i tool disponibili o installati

**Status Phase 0:** Lista solo tool clonati in ~/.tuxbox/tools/

**Usage:**
```bash
tbox list
```

**Output atteso:**
```
Available tools:
  • sshmenuc (https://github.com/disoardi/sshmenuc)
```

**Output se nessun tool:**
```
No tools installed yet.
Run 'tbox run <tool>' to auto-clone and run a tool.
```

**Status futuro (Phase 2):**
Lista tool da registry anche se non clonati

---

### `tbox run <TOOL> [ARGS...]`
Esegue un tool, clonandolo automaticamente se necessario

**Status:** ✅ IMPLEMENTED (Phase 0 MVP)

**Usage:**
```bash
# Run senza argomenti
tbox run sshmenuc

# Run con argomenti
tbox run sshmenuc --help
tbox run sshmenuc --config ~/.ssh/config

# Argomenti con spazi (quoting bash standard)
tbox run sshmenuc --host "my server"
```

**Comportamento:**
1. Cerca tool in configs hardcoded (Phase 0) o registry (Phase 2+)
2. Se tool non trovato → Errore: "Tool 'xyz' not found"
3. Controlla se tool già clonato in `~/.tuxbox/tools/<tool>/`
4. Se non clonato → Clone automatico da Git
5. Esegue comando definito in tool config
6. Passa tutti `[ARGS...]` al tool

**Flow dettagliato:**
```
tbox run sshmenuc --help
  │
  ├─> get_tool_config("sshmenuc")
  │   └─> ToolConfig { repo: "https://...", run: "python sshmenuc.py" }
  │
  ├─> Check ~/.tuxbox/tools/sshmenuc/
  │   ├─> Not exists: git clone https://github.com/disoardi/sshmenuc
  │   └─> Exists: skip clone
  │
  └─> Execute: python sshmenuc.py --help
      └─> Current dir: ~/.tuxbox/tools/sshmenuc/
```

**Stdout/Stderr:**
- Progress messages da tbox (colored): "Cloning tool 'sshmenuc'..."
- Output del tool stesso: pass-through diretto

**Error handling:**
- Tool non trovato in registry
- Git clone failed (network, auth, repo not found)
- Tool execution failed (command not found, runtime error)

**Example outputs:**

Success (primo run):
```
Cloning tool 'sshmenuc'...
Running tool 'sshmenuc'...
[output di sshmenuc]
```

Success (già clonato):
```
Running tool 'sshmenuc'...
[output di sshmenuc]
```

Error (tool non trovato):
```
Error: Tool 'xyz' not found in registry
```

Error (git clone failed):
```
Error: Failed to clone tool 'sshmenuc'
Git error: Failed to resolve address for github.com
```

---

### `tbox update [TOOL]`
Aggiorna tool (git pull)

**Status:** 🚧 PARTIAL (code exists, needs testing)

**Usage:**
```bash
# Update singolo tool
tbox update sshmenuc

# Update tutti i tool
tbox update
```

**Comportamento:**
1. Se `[TOOL]` specificato → git pull in `~/.tuxbox/tools/<tool>/`
2. Se nessun arg → git pull per tutti i tool in `~/.tuxbox/tools/*/`

**Output atteso:**
```
Updating sshmenuc...
  Already up to date.
```

O se ci sono aggiornamenti:
```
Updating sshmenuc...
  Updated: 3 files changed, 24 insertions(+), 8 deletions(-)
```

**Error handling:**
- Tool non installato: "Tool 'xyz' is not installed"
- Git pull failed: "Failed to update 'xyz': [reason]"
- Uncommitted changes: "Tool 'xyz' has uncommitted changes"

---

### `tbox status`
Mostra status TuxBox e tool installati

**Status:** 🚧 PARTIAL (code exists, needs polish)

**Usage:**
```bash
tbox status
```

**Output atteso:**
```
TuxBox Status
=============

Base directory: /Users/davide/.tuxbox
Tools directory: /Users/davide/.tuxbox/tools
Registry directory: /Users/davide/.tuxbox/registry (not initialized)

Installed tools: 1
  • sshmenuc (branch: main, status: clean)
```

**Info mostrate:**
- Directories TuxBox
- Registry status (initialized or not)
- Lista tool con:
  - Nome tool
  - Branch Git corrente
  - Status working tree (clean, dirty, ahead/behind)

---

## 🎯 Tool Hardcoded (Phase 0)

### sshmenuc
**Repository:** https://github.com/disoardi/sshmenuc
**Type:** Python
**Branch:** main
**Run command:** `python sshmenuc.py`

**Setup manuale necessario:**
```bash
cd ~/.tuxbox/tools/sshmenuc
pip install -r requirements.txt
```

**Note:** Phase 1 implementerà auto-setup con venv

---

### test-tool (esempio)
**Repository:** https://github.com/example/test-tool
**Type:** Generic
**Branch:** main
**Run command:** `./run.sh`

**Note:** Tool di test, può non esistere davvero

---

## 🔧 Opzioni Globali

### `--help` / `-h`
Disponibile per tutti i comandi

```bash
tbox --help           # Help generale
tbox run --help       # Help comando run
tbox update --help    # Help comando update
```

### `--version` / `-V`
Mostra versione TuxBox

```bash
tbox --version
# Output: tuxbox 0.1.0
```

---

## 🌈 Colored Output

TuxBox usa colored output per migliorare UX:

- **Verde:** Success messages ("Tool cloned successfully")
- **Rosso:** Error messages ("Error: Tool not found")
- **Giallo:** Warning messages ("Warning: requirements.txt not installed")
- **Blu/Cyan:** Info messages ("Cloning tool 'sshmenuc'...")

**Disable colors:**
```bash
NO_COLOR=1 tbox run sshmenuc
```

O variabile permanente:
```bash
export NO_COLOR=1
```

---

## 🚨 Exit Codes

```
0   Success
1   General error (tool not found, etc.)
2   Git error (clone failed, update failed)
3   Execution error (tool command failed)
127 Command not found (python, docker, etc.)
```

**Usage in scripts:**
```bash
if tbox run sshmenuc; then
    echo "Success"
else
    echo "Failed with exit code $?"
fi
```

---

## 📂 Directory Structure

```
~/.tuxbox/
├── config.toml          (Phase 2: registry URL, settings)
├── registry/            (Phase 2: clone del registry)
│   └── tools.toml
└── tools/               (Tool installati)
    ├── sshmenuc/
    │   ├── .git/
    │   ├── sshmenuc.py
    │   └── requirements.txt
    └── another-tool/
        └── ...
```

---

## 🔍 Debugging

### Verbose mode (TODO - not implemented yet)
```bash
tbox -v run sshmenuc
tbox --verbose run sshmenuc
```

**Output atteso (futuro):**
```
[DEBUG] Loading tool config for 'sshmenuc'
[DEBUG] Config: ToolConfig { repo: "https://...", ... }
[DEBUG] Checking ~/.tuxbox/tools/sshmenuc/
[DEBUG] Tool already cloned, skipping clone
[DEBUG] Executing: python sshmenuc.py
[DEBUG] Current dir: /Users/davide/.tuxbox/tools/sshmenuc
```

### Environment variables
```bash
# Disable colored output
NO_COLOR=1 tbox run sshmenuc

# Override base directory (TODO - future)
TUXBOX_HOME=/tmp/tuxbox tbox run sshmenuc
```

---

## 💡 Tips & Tricks

### Alias per comandi frequenti
```bash
# .bashrc o .zshrc
alias tb='tbox run'
alias tbl='tbox list'
alias tbu='tbox update'

# Usage:
tb sshmenuc
tbl
tbu sshmenuc
```

### Integration con shell completion (TODO - Phase 2)
```bash
# Bash
source <(tbox completion bash)

# Zsh
source <(tbox completion zsh)

# Fish
tbox completion fish | source
```

### Quick access to tools
```bash
# Vai direttamente nella directory del tool
cd ~/.tuxbox/tools/sshmenuc

# Modifica configurazione tool (future)
vim ~/.tuxbox/registry/tools.toml
```

---

## 📝 Esempi Uso Reale

### Scenario 1: Primo utilizzo sshmenuc
```bash
# Prima volta - clone automatico
$ tbox run sshmenuc
Cloning tool 'sshmenuc'...
Running tool 'sshmenuc'...
Error: No module named 'paramiko'

# Setup manuale requirements (Phase 0)
$ cd ~/.tuxbox/tools/sshmenuc
$ pip install -r requirements.txt

# Secondo run - funziona
$ tbox run sshmenuc
Running tool 'sshmenuc'...
[menu SSH di sshmenuc]
```

### Scenario 2: Update dopo modifiche upstream
```bash
# Controllo stato prima
$ tbox status
TuxBox Status
=============
Installed tools: 1
  • sshmenuc (branch: main, status: clean)

# Update tool
$ tbox update sshmenuc
Updating sshmenuc...
  Updated: 2 files changed, 15 insertions(+), 3 deletions(-)

# Run versione aggiornata
$ tbox run sshmenuc
```

### Scenario 3: Troubleshooting tool
```bash
# Vai nella directory del tool
$ cd ~/.tuxbox/tools/sshmenuc

# Controlla git status
$ git status
On branch main
Your branch is up to date with 'origin/main'.

# Test manuale
$ python sshmenuc.py --help

# Se funziona manualmente ma non con tbox → bug in runner.rs
```

---

**Ultimo aggiornamento:** 2026-02-12
**Versione riferimento:** Phase 0 MVP