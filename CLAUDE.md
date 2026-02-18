# TuxBox - Claude Code Implementation Guide

## 🎯 Obiettivo Progetto

**TuxBox** è un meta-tool CLI in Rust per gestire tool personali distribuiti su repository Git. Permette di scaricare, configurare e lanciare tool da un'unica interfaccia, con lazy loading automatico.

**Binary name:** `tbox`
**Storage:** `~/.tuxbox/`
**Tool principale:** sshmenuc (Python tool per gestione SSH)

---

## 📊 Stato Corrente (Aggiornato: 2026-02-17)

### Repository Setup
- Git repository: ✅ Inizializzato e pubblico
- Latest commit: **ce9b2dc** - fix: handle custom Dockerfiles correctly in container execution
- Branch: main
- Remote (pubblico): https://github.com/disoardi/tuxbox
- Remote (enterprise): https://github.dxc.com/disoardi/tuxbox
- **Current version:** v0.2.1 (dev)

### Codice Implementato
- ✅ Struttura modulare completa (11 moduli Rust)
  - main, cli, config, error, git, runner
  - environment, docker, python, registry
  - **selfupdate** (NUOVO - implementato 2026-02-13)
- ✅ Dependencies moderne (Clap 4.5, git2, thiserror, colored, reqwest, tar, flate2)
- ✅ Documentazione esaustiva in `.claude/` + GitHub Pages
- ✅ **Compilato e testato con successo!**
- ✅ **Workflow Docker funzionante end-to-end**
- ✅ **CI/CD automation completa**

### Features Implementate
- ✅ **Dual-mode execution** (Docker-first + venv fallback)
- ✅ **Auto-setup completo** (zero-config per utente)
- ✅ **Docker container management** (auto-build, auto-install deps)
- ✅ **Python venv fallback** (auto-create, auto-install requirements)
- ✅ **Multi-registry support** (public + private registries)
- ✅ **Self-update mechanism** (GitHub API integration)
- ✅ **Smart TTY handling** (conditional -it)
- ✅ **Container naming** (<tool>_<version>)
- ✅ **UID/GID mapping** (stesso utente host per auto-generated Dockerfiles)
- ✅ **HOME directory preservation** (per auto-generated Dockerfiles)
- ✅ **SSH repository support** (auto-detect e system git fallback)
- ✅ **Bash script execution** (direct execution per tool bash type)
- ✅ **Custom Dockerfile detection** (adaptive execution strategy)

### Infrastructure
- ✅ **GitHub Actions CI/CD**
  - Automated testing on push/PR
  - Multi-platform builds (Linux x86_64, macOS ARM64)
  - Automated releases on version tags
- ✅ **GitHub Pages** - https://disoardi.github.io/tuxbox
- ✅ **Published releases** - https://github.com/disoardi/tuxbox/releases

---

## 🎯 Progress Update (2026-02-17)

### ✅ **COMPLETATO:**
- **Phase 0 (MVP):** ✅ 100% - Compilazione, clone, run base
- **Phase 1 (Venv):** ✅ 100% - Auto-setup Python con venv + Docker support
- **Phase 2a (Multi-Registry):** ✅ 100% - Registry system con support multi-registry
- **Infrastructure:** ✅ 100% - CI/CD, GitHub Pages, self-update, pre-commit hooks
- **SSH & Bash Support:** ✅ 100% - SSH repos, bash execution, custom Dockerfiles

### 🔄 **IN PROGRESS:**
- Testing: self-update end-to-end con release live
- Expanding: aggiungere più tool ai registry

### 📋 **NEXT STEPS:**
1. ✅ Setup GitHub repository pubblico (DONE)
2. ✅ CI/CD con GitHub Actions (DONE)
3. ✅ First release v0.2.0 (DONE)
4. ✅ Setup pre-commit hooks per formatting (DONE)
5. ✅ SSH repository support (DONE)
6. ✅ Bash script execution (DONE)
7. ✅ Custom Dockerfile handling (DONE)
8. 🔜 Test self-update mechanism
9. 🔜 Espandere registry con tool personali

---

## 🗺️ Roadmap Completa

```
Phase 0 (MVP) ────> Phase 1 (Venv) ────> Phase 2 (Registry) ────> Phase 3 (Future)
     ↓                   ↓                      ↓                        ↓
  Clone + Run      Auto-setup Python      Multi-Registry System    Advanced Features
  ✅ DONE          ✅ DONE + DOCKER       ✅ DONE (Phase 2a)       📅 FUTURE
```

**Target corrente:** Testing e espansione registry
**Progress:** Phase 0 ✅ | Phase 1 ✅ | Phase 2a ✅ | **Infrastructure ✅**

---

## 📋 Scaletta di Implementazione Dettagliata

## PHASE 0 - MVP (Foundational)

### 🔨 Task 1: Prima Compilazione
**Priority:** 🔴 Critico
**Estimated effort:** 30-60 min
**Files coinvolti:** Tutti i .rs in `src/`

**Checklist:**
- [ ] Esegui `cargo build --release`
- [ ] Analizza errori di compilazione (se presenti)
- [ ] Fix errori seguendo Rust 2026 best practices
- [ ] Verifica che non ci siano warning bloccanti
- [ ] Conferma che binary `tbox` sia creato in `target/release/`

**Success criteria:**
```bash
cargo build --release
# Output: Compiling tuxbox v0.1.0 (/path/to/tuxbox)
#         Finished `release` profile [optimized] target(s) in X.XXs
```

**Troubleshooting:**
- Errori di linking: verifica Xcode command line tools (`xcode-select --install`)
- Missing dependencies: verifica `Cargo.toml`
- Edition errors: conferma `edition = "2024"` in `Cargo.toml`

---

### 🧪 Task 2: Test Help Commands
**Priority:** 🔴 Critico
**Estimated effort:** 15 min
**Depends on:** Task 1

**Checklist:**
- [ ] Test: `cargo run -- --help`
  - Verifica output mostra 5 comandi: init, list, run, update, status
- [ ] Test: `cargo run -- run --help`
  - Verifica usage: `tbox run <TOOL> [ARGS]...`
- [ ] Test: `cargo run -- list --help`
- [ ] Test: `cargo run -- status --help`
- [ ] Test: `cargo run -- update --help`

**Expected output:**
```
TuxBox - Meta-tool manager for distributed Git tools

Usage: tbox <COMMAND>

Commands:
  init    Initialize TuxBox with a registry URL
  list    List all available tools
  run     Run a specific tool
  update  Update one or all tools
  status  Show TuxBox status
  help    Print this message
```

---

### 🚀 Task 3: Test Clone e Run sshmenuc
**Priority:** 🔴 Critico
**Estimated effort:** 45-60 min
**Depends on:** Task 2

**Checklist:**
- [ ] Verifica connessione internet
- [ ] Esegui: `cargo run -- run sshmenuc`
- [ ] Osserva comportamento:
  - Directory `~/.tuxbox/tools/` creata?
  - Clone da `https://github.com/disoardi/sshmenuc` eseguito?
  - Tool directory `~/.tuxbox/tools/sshmenuc/` presente?
  - Tentativo esecuzione `python sshmenuc.py`?
- [ ] Gestisci errori comuni:
  - Python non trovato → messaggio chiaro
  - requirements.txt non installato → messaggio con istruzioni
  - Network failure → error handling

**Expected behavior:**
```bash
$ cargo run -- run sshmenuc
Cloning tool 'sshmenuc'...
Running tool 'sshmenuc'...
Error: No module named 'paramiko'

[Messaggio utile: "Run: cd ~/.tuxbox/tools/sshmenuc && pip install -r requirements.txt"]
```

**Fix da implementare se necessario:**
- Migliorare error messages in `runner.rs`
- Aggiungere check esistenza Python prima di run
- Implementare colored output per progress

---

### 🛠️ Task 4: Test Setup Manuale Python
**Priority:** 🟡 Alto
**Estimated effort:** 30 min
**Depends on:** Task 3

**Checklist:**
- [ ] Setup manuale:
  ```bash
  cd ~/.tuxbox/tools/sshmenuc
  pip install -r requirements.txt  # o pip3
  ```
- [ ] Re-test: `cargo run -- run sshmenuc`
- [ ] Verifica che sshmenuc parta correttamente
- [ ] Test passaggio argomenti: `cargo run -- run sshmenuc --help`
- [ ] Conferma che args vengano passati al tool

**Success criteria:**
- sshmenuc eseguito senza errori Python
- Help di sshmenuc mostrato (non help di tbox)

---

### 📝 Task 5: Test Comandi List e Status
**Priority:** 🟡 Alto
**Estimated effort:** 30 min
**Depends on:** Task 3

**Checklist:**
- [ ] Test: `cargo run -- list`
  - Output atteso: mostra sshmenuc se clonato
  - Se nessun tool: messaggio "No tools installed yet"
- [ ] Test: `cargo run -- status`
  - Mostra base directory
  - Mostra tools directory
  - Lista tool installati con info Git (branch, status)
- [ ] Verifica colored output funzioni
- [ ] Fix eventuali bug di visualizzazione

**Expected output (list):**
```
Available tools:
  • sshmenuc (https://github.com/disoardi/sshmenuc)
```

**Expected output (status):**
```
TuxBox Status
=============

Base directory: /Users/davide/.tuxbox
Tools directory: /Users/davide/.tuxbox/tools

Installed tools: 1
  • sshmenuc (branch: main, status: clean)
```

---

### 🔄 Task 6: Test Comando Update
**Priority:** 🟢 Medio
**Estimated effort:** 30 min
**Depends on:** Task 5

**Checklist:**
- [ ] Test: `cargo run -- update sshmenuc`
  - Verifica git pull funzioni
  - Output: "Already up to date" o "Updated: X files changed"
- [ ] Test: `cargo run -- update` (senza args)
  - Verifica update di tutti i tool in ~/.tuxbox/tools/
- [ ] Test edge case:
  - Update tool non installato → errore chiaro
  - Uncommitted changes → warning appropriato
- [ ] Implementa gestione errori se mancante

---

### 🎨 Task 7: Polish UI/UX (Phase 0)
**Priority:** 🟢 Medio
**Estimated effort:** 45-60 min
**Depends on:** Task 1-6

**Checklist:**
- [ ] Verifica colored output usato ovunque:
  - Success: verde (`.green()`)
  - Error: rosso (`.red()`)
  - Info: blu/cyan (`.cyan()`)
  - Warning: giallo (`.yellow()`)
- [ ] Aggiungi progress indicators:
  - "Cloning tool 'X'..." durante git clone
  - "Running tool 'X'..." prima esecuzione
  - "Updating tool 'X'..." durante update
- [ ] Migliora error messages:
  - "Tool 'xyz' not found in registry"
  - "Failed to clone: [reason]"
  - "Python not found. Install Python 3.x"
- [ ] Aggiungi esempi in help text (cli.rs):
  ```rust
  #[command(about = "Run a specific tool", long_about = None)]
  #[command(after_help = "Examples:\n  tbox run sshmenuc\n  tbox run sshmenuc --help")]
  ```

---

### ✅ Task 8: Testing Completo Phase 0
**Priority:** 🔴 Critico
**Estimated effort:** 30-45 min
**Depends on:** Task 1-7

**Esegui test checklist completa:**
```bash
# Test 1: Compilazione
cargo build --release
# ✅ Compilazione pulita

# Test 2: Help generale
cargo run -- --help
# ✅ Mostra tutti i comandi

# Test 3: Help specifico run
cargo run -- run --help
# ✅ Mostra usage con esempi

# Test 4: Tool non esistente
cargo run -- run nonexistent
# ✅ Errore: "Tool 'nonexistent' not found"

# Test 5: Clone e run sshmenuc (primo run)
cargo run -- run sshmenuc
# ✅ Clone + tentativo esecuzione

# Test 6: Run già clonato
cargo run -- run sshmenuc
# ✅ Skip clone, esecuzione diretta

# Test 7: Passaggio argomenti
cargo run -- run sshmenuc --help
# ✅ Help di sshmenuc (non tbox)

# Test 8: List tools
cargo run -- list
# ✅ Mostra sshmenuc

# Test 9: Status
cargo run -- status
# ✅ Mostra config e tool installati

# Test 10: Update tool
cargo run -- update sshmenuc
# ✅ Git pull eseguito

# Test 11: Colored output
# ✅ Verde/Rosso/Blu visibili
```

**Documenta risultati:**
- Crea file `TEST_RESULTS_PHASE0.md` con output di tutti i test
- Nota eventuali warning o comportamenti anomali
- Elenca bug noti da fixare in iterazioni successive

---

## PHASE 1 - Virtual Environment Support

### 🐍 Task 9: Design Venv Strategy
**Priority:** 🟡 Alto
**Estimated effort:** 45-60 min
**Depends on:** Phase 0 completata

**Design decisions:**
- [ ] Strategia creazione venv:
  - Opzione A: `python -m venv ~/.tuxbox/tools/<tool>/venv`
  - Opzione B: Tool centralizzato `~/.tuxbox/venvs/<tool>/`
  - **Decisione:** A (co-located con tool)
- [ ] Detect Python version:
  - Check `python3 --version`
  - Fallback `python --version`
  - Support Python 3.8+
- [ ] Requirements.txt handling:
  - Auto-detect presenza `requirements.txt`
  - Auto-install con pip in venv
  - Cache per evitare re-install

**Documenta design in:** `docs/PHASE1_DESIGN.md`

---

### 🔧 Task 10: Implementa Python Detection
**Priority:** 🔴 Critico
**Estimated effort:** 30-45 min
**Files:** `src/runner.rs`, nuovo `src/python.rs`

**Implementation:**
- [ ] Crea modulo `python.rs`:
  ```rust
  pub fn detect_python() -> Result<String> {
      // Try python3, then python
      // Return path to python executable
  }

  pub fn get_python_version(python_cmd: &str) -> Result<String> {
      // Execute python --version
      // Parse output
  }
  ```
- [ ] Integra in `runner.rs`:
  - Check Python prima di run tool type Python
  - Error chiaro se Python mancante
- [ ] Unit test in `python.rs`:
  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn test_detect_python() { ... }
  }
  ```

---

### 🏗️ Task 11: Implementa Venv Creation
**Priority:** 🔴 Critico
**Estimated effort:** 60-90 min
**Files:** `src/python.rs`, `src/runner.rs`

**Implementation:**
- [ ] Aggiungi funzione in `python.rs`:
  ```rust
  pub fn create_venv(tool_dir: &Path) -> Result<PathBuf> {
      let venv_path = tool_dir.join("venv");
      if venv_path.exists() {
          return Ok(venv_path);  // Already exists
      }

      let python = detect_python()?;
      Command::new(python)
          .args(["-m", "venv", "venv"])
          .current_dir(tool_dir)
          .output()?;

      Ok(venv_path)
  }

  pub fn install_requirements(venv_path: &Path, tool_dir: &Path) -> Result<()> {
      let requirements = tool_dir.join("requirements.txt");
      if !requirements.exists() {
          return Ok(());  // No requirements
      }

      let pip = venv_path.join("bin/pip");  // macOS/Linux
      // TODO: Windows support (Scripts/pip.exe)

      Command::new(pip)
          .args(["install", "-r", "requirements.txt"])
          .current_dir(tool_dir)
          .output()?;

      Ok(())
  }
  ```
- [ ] Integra in `runner.rs`:
  - Dopo clone, prima di run: check/create venv
  - Install requirements automaticamente
  - Progress indicators: "Setting up Python environment..."
- [ ] Test manualmente con sshmenuc

---

### 🎯 Task 12: Integrazione Venv in Run Command
**Priority:** 🔴 Critico
**Estimated effort:** 45-60 min
**Files:** `src/runner.rs`, `src/config.rs`

**Implementation:**
- [ ] Modifica `ToolConfig` per includere venv info:
  ```rust
  pub struct ToolConfig {
      // ...existing fields...
      pub use_venv: bool,  // Default: true per Python tools
  }
  ```
- [ ] Update `run_tool()` in `runner.rs`:
  ```rust
  pub fn run_tool(tool_name: &str, args: &[String]) -> Result<()> {
      let config = get_tool_config(tool_name)?;
      let ctx = Context::new()?;
      let tool_dir = ctx.tools_dir.join(&config.name);

      // Clone if needed
      if !is_tool_cloned(&tool_dir) {
          clone_tool(&config, &tool_dir)?;
      }

      // Setup venv for Python tools
      if config.tool_type == Some("python") && config.use_venv {
          let venv_path = create_venv(&tool_dir)?;
          install_requirements(&venv_path, &tool_dir)?;
      }

      // Execute tool
      execute_tool(&config, &tool_dir, args)?;
      Ok(())
  }
  ```
- [ ] Test workflow completo:
  - Clone tool nuovo
  - Auto-create venv
  - Auto-install requirements
  - Run tool

---

### ✅ Task 13: Testing Phase 1
**Priority:** 🔴 Critico
**Estimated effort:** 45 min
**Depends on:** Task 9-12

**Test checklist:**
```bash
# Test 1: Venv creation (tool nuovo)
rm -rf ~/.tuxbox/tools/sshmenuc
cargo run -- run sshmenuc
# ✅ Clone + venv created + requirements installed + run

# Test 2: Venv già esistente
cargo run -- run sshmenuc
# ✅ Skip venv creation, run diretto

# Test 3: Tool senza requirements.txt
cargo run -- run test-tool
# ✅ Venv created, no requirements install

# Test 4: Python non trovato
# (simula con PATH modificato)
# ✅ Error: "Python not found. Install Python 3.x"

# Test 5: Requirements install fail
# (simula con requirements.txt invalido)
# ✅ Error chiaro con pip output
```

**Documenta:** `TEST_RESULTS_PHASE1.md`

---

## PHASE 2 - Registry System

### 📋 Task 14: Design Registry Format
**Priority:** 🟡 Alto
**Estimated effort:** 60-90 min
**Depends on:** Phase 1 completata

**Design decisions:**
- [ ] Formato registry: TOML vs YAML
  - **Decisione:** TOML (più Rust-friendly, serde support)
- [ ] Schema registry:
  ```toml
  # tools.toml
  [tools.sshmenuc]
  name = "sshmenuc"
  repo = "https://github.com/disoardi/sshmenuc"
  branch = "main"
  type = "python"
  description = "SSH connection manager with menu UI"
  version = "1.2.0"

  [tools.sshmenuc.commands]
  run = "python sshmenuc.py"

  [tools.sshmenuc.dependencies]
  python = ">=3.8"
  requirements = "requirements.txt"

  [tools.another-tool]
  # ...
  ```
- [ ] Registry repository structure:
  ```
  tuxbox-registry/
  ├── tools.toml          # Main registry file
  ├── README.md
  └── tools/              # Optional: tool-specific configs
      ├── sshmenuc.toml
      └── another.toml
  ```

**Crea:**
- [ ] Repository `tuxbox-registry` su GitHub
- [ ] File `tools.toml` iniziale con sshmenuc
- [ ] README con istruzioni per contribuire

**Documenta design in:** `docs/PHASE2_DESIGN.md`

---

### 🏗️ Task 15: Implementa Registry Module
**Priority:** 🔴 Critico
**Estimated effort:** 90-120 min
**Files:** nuovo `src/registry.rs`, `Cargo.toml`

**Dependencies da aggiungere:**
```toml
[dependencies]
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

**Implementation:**
- [ ] Crea `src/registry.rs`:
  ```rust
  use serde::{Deserialize, Serialize};
  use std::collections::HashMap;

  #[derive(Debug, Deserialize, Serialize)]
  pub struct Registry {
      pub tools: HashMap<String, RegistryTool>,
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct RegistryTool {
      pub name: String,
      pub repo: String,
      pub branch: Option<String>,
      #[serde(rename = "type")]
      pub tool_type: Option<String>,
      pub description: Option<String>,
      pub version: Option<String>,
      pub commands: Option<ToolCommands>,
      pub dependencies: Option<ToolDependencies>,
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct ToolCommands {
      pub run: String,
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct ToolDependencies {
      pub python: Option<String>,
      pub requirements: Option<String>,
  }

  pub fn load_registry(registry_path: &Path) -> Result<Registry> {
      let content = fs::read_to_string(registry_path)?;
      let registry: Registry = toml::from_str(&content)?;
      Ok(registry)
  }

  pub fn get_tool_from_registry(
      registry: &Registry,
      tool_name: &str
  ) -> Option<&RegistryTool> {
      registry.tools.get(tool_name)
  }
  ```
- [ ] Update `src/lib.rs`:
  ```rust
  pub mod registry;
  ```
- [ ] Unit tests in `registry.rs`

---

### 🔄 Task 16: Implementa Registry Clone (init command)
**Priority:** 🔴 Critico
**Estimated effort:** 60-90 min
**Files:** `src/registry.rs`, `src/main.rs`, `src/config.rs`

**Implementation:**
- [ ] Aggiungi funzione in `registry.rs`:
  ```rust
  pub fn clone_registry(registry_url: &str, dest: &Path) -> Result<()> {
      if dest.exists() {
          // Already cloned, do git pull instead
          update_registry(dest)?;
          return Ok(());
      }

      println!("{}", "Initializing TuxBox registry...".cyan());
      git::clone_repository(registry_url, dest)?;
      println!("{}", "Registry initialized successfully!".green());
      Ok(())
  }

  pub fn update_registry(registry_dir: &Path) -> Result<()> {
      git::update_repository(registry_dir)
  }
  ```
- [ ] Implementa comando `init` in `main.rs`:
  ```rust
  Commands::Init { registry_url } => {
      let ctx = config::Context::new()?;
      registry::clone_registry(&registry_url, &ctx.registry_dir)?;

      // Save registry URL in config
      config::save_config(&ctx, &registry_url)?;
  }
  ```
- [ ] Aggiungi `config.toml` support in `config.rs`:
  ```rust
  #[derive(Deserialize, Serialize)]
  pub struct TuxBoxConfig {
      pub registry_url: String,
      pub initialized: bool,
  }

  pub fn save_config(ctx: &Context, registry_url: &str) -> Result<()> {
      let config = TuxBoxConfig {
          registry_url: registry_url.to_string(),
          initialized: true,
      };
      let config_path = ctx.base_dir.join("config.toml");
      let toml = toml::to_string(&config)?;
      fs::write(config_path, toml)?;
      Ok(())
  }

  pub fn load_config(ctx: &Context) -> Result<TuxBoxConfig> {
      let config_path = ctx.base_dir.join("config.toml");
      if !config_path.exists() {
          return Err(TuxBoxError::NotInitialized);
      }
      let content = fs::read_to_string(config_path)?;
      let config: TuxBoxConfig = toml::from_str(&content)?;
      Ok(config)
  }
  ```

---

### 🔌 Task 17: Integra Registry in Runner
**Priority:** 🔴 Critico
**Estimated effort:** 90 min
**Files:** `src/runner.rs`, `src/config.rs`

**Implementation:**
- [ ] Modifica `get_tool_config()` in `runner.rs`:
  ```rust
  pub fn get_tool_config(tool_name: &str) -> Result<ToolConfig> {
      let ctx = Context::new()?;

      // Try loading from registry first
      if let Ok(config) = load_config(&ctx) {
          let registry_path = ctx.registry_dir.join("tools.toml");
          if registry_path.exists() {
              let registry = registry::load_registry(&registry_path)?;
              if let Some(tool) = registry::get_tool_from_registry(&registry, tool_name) {
                  return Ok(tool_to_config(tool));
              }
          }
      }

      // Fallback to hardcoded (backward compatibility)
      get_hardcoded_tool_config(tool_name)
  }

  fn tool_to_config(tool: &RegistryTool) -> ToolConfig {
      ToolConfig {
          name: tool.name.clone(),
          repo: tool.repo.clone(),
          branch: tool.branch.clone(),
          tool_type: tool.tool_type.clone(),
          // ...map other fields...
      }
  }

  fn get_hardcoded_tool_config(tool_name: &str) -> Result<ToolConfig> {
      // Existing hardcoded logic
      match tool_name {
          "sshmenuc" => Ok(ToolConfig { ... }),
          _ => Err(TuxBoxError::ToolNotFound(tool_name.to_string()))
      }
  }
  ```
- [ ] Update comando `list`:
  ```rust
  pub fn list_tools() -> Result<()> {
      let ctx = Context::new()?;

      // List from registry if initialized
      if let Ok(config) = load_config(&ctx) {
          let registry_path = ctx.registry_dir.join("tools.toml");
          if registry_path.exists() {
              let registry = registry::load_registry(&registry_path)?;
              println!("{}", "Available tools in registry:".cyan());
              for (name, tool) in &registry.tools {
                  println!("  • {} - {}", name.green(), tool.description.as_deref().unwrap_or(""));
              }
              return Ok(());
          }
      }

      // Fallback to listing installed tools
      list_installed_tools(&ctx)
  }
  ```

---

### ✅ Task 18: Testing Phase 2
**Priority:** 🔴 Critico
**Estimated effort:** 60 min
**Depends on:** Task 14-17

**Setup test registry:**
```bash
# Create test registry locally
mkdir -p /tmp/test-tuxbox-registry
cd /tmp/test-tuxbox-registry
git init
# Create tools.toml con sshmenuc + test-tool
git add tools.toml
git commit -m "Initial registry"
```

**Test checklist:**
```bash
# Test 1: Init con registry locale
cargo run -- init file:///tmp/test-tuxbox-registry
# ✅ Registry clonato in ~/.tuxbox/registry/
# ✅ config.toml creato

# Test 2: List tools da registry
cargo run -- list
# ✅ Mostra tutti i tool nel registry (anche non installati)

# Test 3: Run tool da registry
rm -rf ~/.tuxbox/tools/sshmenuc
cargo run -- run sshmenuc
# ✅ Config caricato da registry
# ✅ Clone e run come Phase 1

# Test 4: Backward compatibility (no registry)
rm ~/.tuxbox/config.toml
rm -rf ~/.tuxbox/registry/
cargo run -- run sshmenuc
# ✅ Fallback a hardcoded configs

# Test 5: Registry update
cargo run -- update-registry  # New command?
# ✅ Git pull del registry

# Test 6: Tool non in registry
cargo run -- run nonexistent-tool
# ✅ Error: "Tool 'nonexistent-tool' not found in registry"
```

**Documenta:** `TEST_RESULTS_PHASE2.md`

---

### 🚀 Task 19: Setup Repository GitHub
**Priority:** 🟡 Alto
**Estimated effort:** 45 min

**Checklist:**
- [ ] Crea repository `tuxbox` su GitHub
- [ ] Aggiungi remote:
  ```bash
  git remote add origin https://github.com/disoardi/tuxbox.git
  ```
- [ ] Push initial commit:
  ```bash
  git push -u origin main
  ```
- [ ] Crea repository `tuxbox-registry`
- [ ] Push registry con tools.toml
- [ ] Setup branch protection su main (opzionale)
- [ ] Aggiungi topics: `rust`, `cli`, `tool-manager`, `git`
- [ ] Crea GitHub releases per versioni

---

### 🔬 Task 20: Setup Testing Infrastructure
**Priority:** 🟢 Medio
**Estimated effort:** 90-120 min

**Unit Tests:**
- [ ] Test per `config.rs`:
  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn test_context_creation() { ... }

      #[test]
      fn test_save_load_config() { ... }
  }
  ```
- [ ] Test per `registry.rs`:
  ```rust
  #[test]
  fn test_load_registry_valid_toml() { ... }

  #[test]
  fn test_get_tool_from_registry() { ... }
  ```
- [ ] Test per `python.rs`:
  ```rust
  #[test]
  fn test_detect_python() { ... }

  #[test]
  fn test_python_version_parsing() { ... }
  ```
- [ ] Run: `cargo test`

**Integration Tests:**
- [ ] Crea `tests/integration_test.rs`:
  ```rust
  use assert_cmd::Command;
  use predicates::prelude::*;

  #[test]
  fn test_help_command() {
      let mut cmd = Command::cargo_bin("tbox").unwrap();
      cmd.arg("--help")
          .assert()
          .success()
          .stdout(predicate::str::contains("TuxBox"));
  }

  #[test]
  fn test_run_nonexistent_tool() {
      let mut cmd = Command::cargo_bin("tbox").unwrap();
      cmd.args(["run", "fake-tool"])
          .assert()
          .failure()
          .stderr(predicate::str::contains("not found"));
  }
  ```
- [ ] Aggiungi dependencies:
  ```toml
  [dev-dependencies]
  assert_cmd = "2.0"
  predicates = "3.0"
  ```

---

### 🔄 Task 21: Setup CI/CD (GitHub Actions)
**Priority:** 🟢 Medio
**Estimated effort:** 60-90 min

**Create `.github/workflows/ci.yml`:**
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable]

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Check formatting
      run: cargo fmt -- --check

    - name: Clippy
      run: cargo clippy -- -D warnings

    - name: Build
      run: cargo build --release

    - name: Run tests
      run: cargo test

    - name: Upload binary
      if: matrix.os == 'ubuntu-latest'
      uses: actions/upload-artifact@v3
      with:
        name: tbox-linux
        path: target/release/tbox
```

**Create `.github/workflows/release.yml`:**
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin

    steps:
    - uses: actions/checkout@v3

    - name: Build release
      run: cargo build --release --target ${{ matrix.target }}

    - name: Create release
      uses: softprops/action-gh-release@v1
      with:
        files: target/${{ matrix.target }}/release/tbox
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

### 📚 Task 22: Documentazione Utente
**Priority:** 🟢 Medio
**Estimated effort:** 60 min

**Update README.md:**
- [ ] Installation instructions
- [ ] Quick start guide
- [ ] Usage examples per ogni comando
- [ ] Contributing guidelines
- [ ] License (MIT?)

**Create additional docs:**
- [ ] `docs/INSTALLATION.md` - Setup dettagliato
- [ ] `docs/USAGE.md` - Esempi avanzati
- [ ] `docs/REGISTRY.md` - Come creare registry custom
- [ ] `docs/CONTRIBUTING.md` - Guidelines per contributor

---

## 🎯 PHASE 3 - Future Roadmap (Backlog)

### Task 23: Docker Isolation Support
**Estimated effort:** 4-6 hours
**Description:** Eseguire tool in container Docker isolati

**Features:**
- Auto-build Dockerfile se presente nel tool
- Volume mounting per accesso filesystem
- Network isolation opzionale

---

### Task 24: Multi-Language Support
**Estimated effort:** 3-4 hours
**Description:** Supporto per tool Go, Node.js, Ruby, etc.

**Features:**
- Auto-detect linguaggio da file marker (go.mod, package.json, Gemfile)
- Setup environment specifico per linguaggio
- Dependency management per ogni linguaggio

---

### Task 25: Plugin System
**Estimated effort:** 6-8 hours
**Description:** Sistema plugin per estendere TuxBox

**Features:**
- Plugin in Rust con dylib
- Hooks per lifecycle events (pre-run, post-run, on-update)
- Plugin registry separato

---

### Task 26: GUI/TUI (Optional)
**Estimated effort:** 8-12 hours
**Description:** Interfaccia testuale interattiva (ratatui/cursive)

**Features:**
- Menu navigabile per tool selection
- Live output durante esecuzione
- Config editor interattivo

---

## 📋 Guidelines di Sviluppo

### Principi Architetturali
1. **Modularità:** Ogni feature in modulo separato
2. **Error handling robusto:** Usa `Result<T, TuxBoxError>` ovunque
3. **User-friendly:** Error messages chiari e actionable
4. **Backward compatibility:** Non rompere workflow esistenti
5. **Documentation:** Commenti, doc-comments, README sempre aggiornati

### Coding Standards
```rust
// ✅ DO: Use Result with custom errors
pub fn do_something() -> Result<Value, TuxBoxError> {
    let value = risky_operation()?;
    Ok(value)
}

// ✅ DO: Pattern matching per clarity
match result {
    Ok(val) => handle_success(val),
    Err(e) => handle_error(e),
}

// ✅ DO: Colored output
println!("{}", "Success!".green());
eprintln!("{}", "Error!".red());

// ❌ DON'T: unwrap/expect in production code
let value = some_function().unwrap();  // NO!

// ❌ DON'T: Ignore errors silently
let _ = risky_operation();  // NO!
```

### Git Workflow
```bash
# Feature branch per ogni task
git checkout -b feature/task-10-python-detection

# Commit frequenti con messaggi descrittivi
git commit -m "feat: implement Python version detection

- Add detect_python() function
- Support python3 and python fallback
- Add unit tests"

# Push e merge tramite PR (opzionale per progetto personale)
git push origin feature/task-10-python-detection
```

### Testing Strategy
- **Unit tests:** Ogni modulo deve avere test in `#[cfg(test)] mod tests`
- **Integration tests:** In `tests/` directory per workflow end-to-end
- **Manual testing:** Checklist dopo ogni phase
- **CI:** Run automatico di tutti i test su push

---

## 🚀 GitHub Actions & CI/CD

### GitHub Actions Best Practices

#### Permission Requirements
- **Always specify** `permissions:` esplicitamente nei workflows
- Per releases: `permissions: contents: write`
- Per PRs: `permissions: pull-requests: write`
- Test workflows locally con `act` tool quando possibile

**Example:**
```yaml
jobs:
  release:
    permissions:
      contents: write  # Required for creating releases
    runs-on: ubuntu-latest
```

#### Multi-Platform Builds
- **Preferire native runners** a cross-compilation
  - ✅ Use `macos-14` per ARM64 (M1/M2/M3)
  - ✅ Use `ubuntu-latest` per Linux x86_64
  - ⚠️ macOS Intel builds: verificare runner availability
- **Cross-compilation issues:**
  - OpenSSL linking failures su macOS cross-compile
  - Richiede setup complesso (target installation, env vars)
- **Fallback strategy:**
  - Documentare uso Rosetta 2 per Intel Macs
  - Fornire alternative se runner non disponibili

#### Workflow Patterns
- **Use `gh CLI`** invece di deprecated GitHub Actions
  - ✅ `gh release create` invece di `actions/create-release@v1`
  - ✅ `gh release upload` per asset management
- **Implement retry logic** per upload assets (concurrent builds)
- **Create release** solo da primo job, altri upload assets
- **Asset naming convention:** `<tool>-<version>-<platform>-<arch>.tar.gz`

**Example:**
```bash
# Good: Using gh CLI
gh release create ${{ github.ref_name }} --generate-notes
gh release upload ${{ github.ref_name }} tbox-linux-x86_64.tar.gz

# Bad: Using deprecated action (will fail)
- uses: actions/create-release@v1  # DEPRECATED!
```

#### Common Pitfalls
- ❌ Non specificare permissions → "Resource not accessible by integration"
- ❌ Cross-compile macOS → OpenSSL linking errors
- ❌ Assumere runner availability → infinite queue times
- ❌ Usare actions deprecate → workflow failures
- ❌ Dimenticare `cargo fmt` → CI formatting failures

---

### Release Workflow Checklist

#### Pre-Release
- [ ] Run `cargo fmt` per evitare CI failures
- [ ] Run `cargo clippy -- -D warnings` per catch warnings
- [ ] Run `cargo test` per verificare tutti i test passano
- [ ] Test build locale: `cargo build --release`
- [ ] Verificare GitHub runner availability per piattaforme target
- [ ] Update CHANGELOG.md con nuove features (opzionale)

#### Release Process
1. **Bump version** in `Cargo.toml`
   ```bash
   # Edit Cargo.toml: version = "0.3.0"
   cargo build  # Update Cargo.lock
   ```

2. **Commit version bump**
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "chore: bump version to 0.3.0"
   ```

3. **Create annotated tag**
   ```bash
   git tag -a v0.3.0 -m "Release v0.3.0

   - Feature 1
   - Feature 2
   - Bug fix 3"
   ```

4. **Push tag** (trigger release workflow)
   ```bash
   git push origin main
   git push origin v0.3.0
   ```

5. **Monitor GitHub Actions**
   ```bash
   # Watch workflow progress
   gh run watch

   # Or check online
   open https://github.com/disoardi/tuxbox/actions
   ```

#### Post-Release
- [ ] Verificare release creata: https://github.com/disoardi/tuxbox/releases
- [ ] Download e test binari da release per ogni piattaforma
- [ ] Test self-update da versione precedente: `tbox self-update`
- [ ] Update documentazione se necessario
- [ ] Sync con enterprise remote: `git push origin-dxc main --tags`

---

### Dual Repository Strategy

Quando serve backup enterprise + repository pubblico per self-update e community.

#### Setup Remotes
```bash
# Add public remote (primary)
git remote add origin git@github.com:disoardi/tuxbox.git

# Add enterprise backup remote
git remote add origin-dxc git@github.dxc.com:disoardi/tuxbox.git

# Verify remotes
git remote -v
```

#### Sync Workflow
```bash
# Push to both remotes
git push origin main
git push origin-dxc main

# Push tags to both
git push origin --tags
git push origin-dxc --tags

# Create alias for convenience
git config alias.pushall '!git push origin main && git push origin-dxc main'
git config alias.pushall-tags '!git push origin --tags && git push origin-dxc --tags'

# Use alias
git pushall
git pushall-tags
```

#### Benefits
- ✅ **Automatic backup** su enterprise GitHub
- ✅ **Public visibility** per self-update e community contributions
- ✅ **Simple sync** con comandi standard git
- ✅ **Fallback** se un remote è down

#### Considerations
- Enterprise remote è **backup only** (no CI/CD, no releases)
- Releases e self-update usano solo public remote
- Keep both in sync per consistency

---

## 🦀 Rust / git2 Best Practices

### git2 StatusOptions — Always Explicit

**Never** use `repo.statuses(None)` to check if a working directory is clean.
The default options include **untracked files**, causing false positives for
tbox-managed directories like `venv/` that live inside tool repos.

**Correct pattern:**
```rust
let mut opts = git2::StatusOptions::new();
opts.include_untracked(false);
opts.include_ignored(false);
let statuses = repo.statuses(Some(&mut opts))?;
if !statuses.is_empty() {
    // actual tracked-file modifications only
}
```

---

## 📋 Registry: Python Version Policy

### When to set `python = ">=X.Y"` in a registry entry

The field must reflect the **minimum version the tool actually works with**,
not the version declared in the tool's own `pyproject.toml`.

**Rules:**
- ✅ Set `python = ">=3.8"` only when a key dependency **genuinely does not
  exist** for older Python (e.g. `cryptography>=41.0.0` requires Python 3.8+)
- ❌ Do NOT copy the pyproject.toml constraint blindly — Poetry projects often
  declare `python = "^3.9"` but their deps install fine on 3.6 via pip
- If a tool works on Python 3.6 via the Poetry deps fallback, **omit** the
  `python` field entirely or add a comment explaining why

**Example:**
```toml
[tools.sshmenuc.dependencies]
# No minimum Python enforced — works on 3.6+ via Poetry deps fallback
requirements = "requirements.txt"

[tools.cert_checker.dependencies]
python = ">=3.8"   # cryptography>=41.0.0 does not exist for Python 3.6
poetry = true
```

---

## 🐍 Python Venv Execution — Generic Activation Pattern

### The Rule

**Never dispatch tool commands with special-case logic** (e.g. "if it's in venv/bin do X,
otherwise do Y"). This breaks portability and couples tbox to tool internals.

**The only correct approach**: simulate `source venv/bin/activate` by prepending `venv/bin`
to PATH in the **child process environment**. This resolves all command types generically:
- `python3 -m tool` → uses venv's python3
- `cert-checker` → console script in venv/bin, found via PATH
- `any-script` → shell script in venv/bin, found via PATH

The change is **scoped to the child process only** — tbox's own process is unaffected,
no cleanup needed.

**Correct implementation:**
```rust
fn execute_in_venv(venv_path: &Path, tool_path: &Path, run_command: &str, args: &[String]) -> Result<()> {
    let current_path = std::env::var("PATH").unwrap_or_default();
    #[cfg(unix)]
    let venv_bin = venv_path.join("bin");
    let activated_path = format!("{}:{}", venv_bin.display(), current_path);

    let parts: Vec<&str> = run_command.split_whitespace().collect();
    let mut cmd = Command::new(parts[0]);
    cmd.current_dir(tool_path);
    cmd.env("PATH", &activated_path);      // venv activation
    cmd.env("VIRTUAL_ENV", venv_path);     // standard venv marker
    if parts.len() > 1 { cmd.args(&parts[1..]); }
    cmd.args(args);
    // spawn/exec...
}
```

**Never do this:**
```rust
// BAD: special-casing
let venv_bin_cmd = venv_path.join("bin").join(&run_cmd);
if venv_bin_cmd.exists() {
    Command::new(venv_bin_cmd)  // fragile, non-generic
} else {
    Command::new("python").arg(&run_cmd)  // wrong for console scripts
}
```

---

## 💾 Tool State File — Skip Reinstall on Subsequent Runs

### The Problem

Without state tracking, every `tbox run` re-checks Python version, re-validates the venv,
and potentially re-installs dependencies. On slow servers this is minutes of wasted time.

### The Pattern

After a **successful first install**, write `.tuxbox-state.toml` to the tool directory:
```toml
version = "1"
method = "venv"

[venv]
path = "/home/user/.tuxbox/tools/my_tool/venv"
python = "/home/user/.pyenv/versions/3.9.25/bin/python3"
```

On **subsequent runs**, load the state file → if valid (venv dir still exists), skip
all setup and jump directly to execution.

**State lifecycle:**
- **Written**: after `run_in_venv` completes first install successfully
- **Read**: at the start of every `tbox run` — fast path if valid
- **Invalidated** (file deleted): when `tbox update` pulls new commits, so the
  next run re-installs deps against updated source

**Key invariant**: `ToolState::invalidate()` MUST be called after every successful
fast-forward merge in `update_tool`.

**Validation on load** (always check, never trust blindly):
```rust
pub fn load(tool_path: &Path) -> Option<Self> {
    let state: ToolState = toml::from_str(&content).ok()?;
    // Validate venv still exists on disk
    if let Some(ref venv) = state.venv {
        if !venv.path.exists() { return None; }
    }
    Some(state)
}
```

---

## 🔐 SSH Repository Support

### SSH Clone Strategy

**Automatic detection**: Repository URLs starting with `git@` or `ssh://` automatically use system git instead of git2.

**Why system git for SSH:**
- Handles SSH keys automatically without callback setup
- Uses configured SSH keys from `~/.ssh/` directory
- Simpler and more robust than git2 SSH callbacks
- Supports SSH agent and key passphrases

**Implementation details:**
- HTTPS URLs continue using git2 crate (faster, native)
- SSH URLs use `git clone` command via system git
- No configuration needed if SSH keys are properly set up

### Testing SSH Tools

When adding private SSH repositories to registry:

1. **Verify SSH key is configured** for the GitHub instance:
   ```bash
   ssh -T git@github.com
   # or for enterprise
   ssh -T git@github.company.com
   ```

2. **Test manual clone first**:
   ```bash
   git clone git@github.com:user/private-repo.git /tmp/test
   ```

3. **Add to registry** with full SSH URL:
   ```toml
   [tools.my_private_tool]
   repo = "git@github.com:user/private-repo"
   private = true
   ```

4. **Test with tbox**:
   ```bash
   rm -rf ~/.tuxbox/tools/my_private_tool
   tbox run my_private_tool
   ```

### Common SSH Issues

**Problem:** "Permission denied (publickey)"
- **Solution:** Add SSH key to GitHub account and ssh-agent
- **Check:** `ssh-add -l` to list loaded keys

**Problem:** "Host key verification failed"
- **Solution:** Add host to known_hosts: `ssh-keyscan github.com >> ~/.ssh/known_hosts`

---

## 🐚 Bash Script Execution

### Tool Type: bash

For shell scripts, Ansible playbooks, or any bash-executable tool that doesn't require Python/Docker isolation.

**Execution strategy:**
- Direct execution via `bash -c` in tool directory
- No Docker container or Python venv required
- Runs in host environment with full access
- Inherits user's environment variables

**When to use type "bash":**
- ✅ Shell scripts with menu/CLI interfaces
- ✅ Ansible playbooks with wrapper scripts
- ✅ Tools managing their own dependencies (Poetry, npm, etc.)
- ✅ System utilities requiring host access
- ❌ Python tools → use `type = "python"` instead
- ❌ Complex environments needing isolation → consider Docker

### Registry Configuration

**Example bash tool entry:**
```toml
[tools.my_bash_tool]
name = "my_bash_tool"
repo = "https://github.com/user/bash-tool"
type = "bash"
description = "Shell script utility with interactive menu"

[tools.my_bash_tool.commands]
run = "./scripts/run.sh"

[tools.my_bash_tool.dependencies]
# Dependencies managed by tool itself
poetry = true  # or npm, conda, etc.
```

**Entry point options:**
- Relative path from repo root: `./scripts/menu.sh`
- Simple command: `bash main.sh`
- With args: `bash run.sh --config config.toml`

### Environment and Permissions

**What bash tools have access to:**
- Full filesystem access (same as user)
- All environment variables
- Network access
- System commands (docker, kubectl, etc.)

**Security considerations:**
- Bash tools run with your user permissions
- Private repos should be from trusted sources
- Review script code before first run
- Consider Docker isolation for untrusted tools

---

## 🐳 Docker Custom Dockerfile Support

### Custom vs Auto-Generated Dockerfiles

TuxBox automatically detects whether a tool provides its own Dockerfile and adjusts execution strategy accordingly.

**Detection:** Checks for `Dockerfile` existence in tool repository root.

### Execution Strategies

#### Custom Dockerfile (tool provides Dockerfile)

When a tool has its own Dockerfile, tbox trusts the tool's configuration:

**Container execution:**
- ✅ **Run as root** (no `--user` flag)
  - Custom Dockerfiles may install packages in `/root/.local`
  - User-specific paths may not be accessible to non-root users
- ✅ **Preserve HOME=/root** (no custom `HOME` env)
  - Python looks for packages in `$HOME/.local/lib/pythonX.X/site-packages`
  - Overriding HOME breaks package resolution
- ✅ **Use Dockerfile ENTRYPOINT** (no command override)
  - Respects tool's intended entry point
  - Only passes user arguments to ENTRYPOINT

**Example:**
```bash
# tbox executes:
docker run --rm -it \
  -v ~/.ssh:~/.ssh:ro \
  -v ~/.config:~/.config \
  tuxbox-cert_checker check --host google.com
  # Uses ENTRYPOINT from Dockerfile, just passes args
```

#### Auto-Generated Dockerfile (no Dockerfile in tool)

For tools without Dockerfile, tbox generates `Dockerfile.tuxbox`:

**Container execution:**
- ✅ **Run as current user** (`--user uid:gid`)
  - Preserves file permissions for created files
  - Security: non-root container execution
- ✅ **Set custom HOME** (`-e HOME=$USER_HOME`)
  - Maintains user environment consistency
- ✅ **Execute explicit command** (`python3 -m tool_name`)
  - Auto-generated images have no ENTRYPOINT
  - Full command must be provided

**Example:**
```bash
# tbox executes:
docker run --rm -it \
  --user 501:502 \
  -e HOME=/Users/username \
  -v ~/.ssh:~/.ssh:ro \
  -v ~/.config:~/.config \
  tuxbox-sshmenuc python3 -m sshmenuc [args]
  # Explicit command + args
```

### Common Issues with Custom Dockerfiles

#### Issue: "ModuleNotFoundError" in Custom Dockerfile

**Symptom:**
```
ModuleNotFoundError: No module named 'click'
```

**Cause:**
Dockerfile installs packages with `pip install --user` in `/root/.local`, but:
1. Container runs as non-root user → cannot access `/root/`
2. HOME is overridden → Python looks in wrong directory

**Solutions:**

**Option 1: Fix Dockerfile (RECOMMENDED)**
Install packages system-wide instead of user-local:
```dockerfile
# ❌ BEFORE (broken with tbox)
RUN pip install --no-cache-dir --user -r requirements.txt
COPY --from=builder /root/.local /root/.local
ENV PATH=/root/.local/bin:$PATH

# ✅ AFTER (works with tbox)
RUN pip install --no-cache-dir -r requirements.txt
# Packages go to /usr/local/lib/pythonX.X/site-packages
# Accessible to all users
```

**Option 2: Trust Custom Dockerfile (tbox already does this)**
TuxBox automatically:
- Runs as root for custom Dockerfiles
- Preserves HOME=/root
- Uses Dockerfile ENTRYPOINT

### Best Practices for Custom Dockerfiles

**✅ DO:**
```dockerfile
# Install system-wide (accessible to all users)
RUN pip install --no-cache-dir -r requirements.txt

# Use ENTRYPOINT for consistent execution
ENTRYPOINT ["python", "-m", "your_tool"]
CMD ["--help"]

# Multi-stage build for smaller images
FROM python:3.11-slim AS builder
# ... build stage ...
FROM python:3.11-slim
COPY --from=builder /usr/local/lib /usr/local/lib
```

**❌ DON'T:**
```dockerfile
# Avoid user-local installations
RUN pip install --user -r requirements.txt  # Breaks with non-root

# Avoid hardcoding user/home
USER appuser  # May conflict with tbox volume mounts
ENV HOME=/home/appuser  # May be overridden
```

### Testing Custom Dockerfile Tools

When adding a tool with custom Dockerfile:

1. **Test directly first:**
   ```bash
   cd ~/.tuxbox/tools/tool_name
   docker build -t test-tool .
   docker run --rm test-tool --help
   ```

2. **Test with tbox:**
   ```bash
   tbox run tool_name --help
   ```

3. **Verify packages accessible:**
   ```bash
   docker run --rm --entrypoint python tuxbox-tool_name \
     -c "import sys; print('\n'.join(sys.path)); import click"
   ```

4. **Check as non-root (if suspicious):**
   ```bash
   docker run --rm --user $(id -u):$(id -g) \
     --entrypoint python tuxbox-tool_name \
     -c "import click; print('OK')"
   # Should work if Dockerfile uses system-wide install
   # Should fail if Dockerfile uses --user flag
   ```

### Registry Configuration

No special configuration needed - tbox auto-detects:

```toml
[tools.tool_with_dockerfile]
name = "tool_with_dockerfile"
repo = "https://github.com/user/tool"
type = "python"  # or any type
# tbox checks for Dockerfile automatically
```

---

## 📋 Registry Expansion Workflow

### Adding New Tools - Step by Step

#### 1. Analyze Tool Structure

Clone and explore the tool locally:
```bash
cd /tmp
git clone <repo-url>
cd <tool-name>
ls -la
```

**Identify:**
- Entry point: script file, Python module, binary
- Dependencies: requirements.txt, pyproject.toml, package.json, Makefile
- Documentation: README, USAGE, etc.

#### 2. Determine Tool Type

**Type decision matrix:**

| Tool Characteristics | Type | Execution |
|---------------------|------|-----------|
| Python package/module | `python` | Docker → venv fallback |
| Shell script, Ansible | `bash` | Direct bash execution |
| Binary/compiled | `bash` | Direct execution |
| Complex (multi-lang) | Consider Docker | Custom Dockerfile |

**Check for:**
- Python: `*.py`, `requirements.txt`, `pyproject.toml`, `setup.py`
- Bash: `*.sh`, `Makefile`, shell scripts
- Poetry: `pyproject.toml` with `[tool.poetry]`

#### 3. Add to Registry

**Template:**
```toml
[tools.tool_name]
name = "tool_name"
repo = "https://github.com/user/tool"  # or git@... for private
branch = "main"
version = "0.1.0"
type = "python" | "bash"
description = "Brief description"
private = false  # true for SSH repos

[tools.tool_name.commands]
run = "command-name" | "./script.sh" | "python -m module"

[tools.tool_name.dependencies]
python = ">=3.8"  # for Python tools
poetry = true     # if uses Poetry
requirements = "requirements.txt"  # if has requirements
```

#### 4. Test Workflow

**Complete test sequence:**
```bash
# 1. Verify tool appears in registry
tbox list | grep tool_name

# 2. Clean slate (force fresh clone)
rm -rf ~/.tuxbox/tools/tool_name
docker rmi tool_name_* 2>/dev/null

# 3. Test clone + run
tbox run tool_name --help

# 4. Test idempotence (should skip clone)
tbox run tool_name --help

# 5. Test update
tbox update tool_name
```

**Expected outcomes:**
- ✅ First run: clones repo, sets up environment, runs command
- ✅ Second run: skips clone, runs directly
- ✅ Update: pulls latest changes, preserves local setup

#### 5. Commit and Push

**Registry repository workflow:**
```bash
cd ~/.tuxbox/registry/tuxbox-registry-private

# Review changes
git diff tools.toml

# Commit with descriptive message
git add tools.toml
git commit -m "feat: add <tool-name> to registry

- Type: <python|bash>
- Repository: <url>
- Description: <what it does>

Tools now: <count>"

# Push to registry
git push
```

### Common Patterns

**Python tool with Poetry:**
```toml
type = "python"
[commands]
run = "tool-command"
[dependencies]
python = ">=3.8"
poetry = true
```

**Bash script (Ansible, shell):**
```toml
type = "bash"
[commands]
run = "./scripts/menu.sh"
[dependencies]
poetry = true  # if tool uses Poetry for deps
```

**Private SSH repository:**
```toml
repo = "git@github.com:user/private-tool"
private = true
```

**Public HTTPS repository:**
```toml
repo = "https://github.com/user/public-tool"
private = false
```

### Troubleshooting

**Tool doesn't appear in `tbox list`:**
- Check tools.toml syntax with `toml` validator
- Verify registry was synced: `tbox list` should show update message
- Try: `rm -rf ~/.tuxbox/registry && tbox list` to force re-clone

**Clone fails:**
- HTTPS: Check repository is public or credentials are set
- SSH: Verify SSH key is configured (`ssh -T git@github.com`)
- Check network/firewall

**Execution fails:**
- Docker build error: Check Dockerfile in tool repo
- Python import error: Verify dependencies are installed
- Bash script error: Check script has execute permission and shebang

**Update doesn't pull changes:**
- Check for uncommitted changes: `cd ~/.tuxbox/tools/<tool> && git status`
- Force re-clone: `rm -rf ~/.tuxbox/tools/<tool> && tbox run <tool>`

---

## 📖 Riferimenti e Risorse

### Documentazione Progetto
- `.claude/README.md` - Hub documentazione
- `.claude/quick-start.md` - Quick reference comandi
- `.claude/cli-reference.md` - Reference completa CLI
- `.claude/architecture-notes.md` - Deep dive architettura
- `CLAUDE_CODE_HANDOFF.md` - Contesto handoff originale

### Codebase
- `src/main.rs` - Entry point
- `src/cli.rs` - Clap CLI definitions
- `src/config.rs` - Configuration e Context
- `src/error.rs` - Custom error types
- `src/git.rs` - Git operations
- `src/runner.rs` - Tool execution
- `src/python.rs` - (Phase 1) Python support
- `src/registry.rs` - (Phase 2) Registry system

### External Resources
- Rust Book: https://doc.rust-lang.org/book/
- Clap docs: https://docs.rs/clap/latest/clap/
- git2 docs: https://docs.rs/git2/latest/git2/
- serde docs: https://serde.rs/
- thiserror: https://docs.rs/thiserror/latest/thiserror/

### Repositories
- **TuxBox:** https://github.com/disoardi/tuxbox (da creare)
- **TuxBox Registry:** https://github.com/disoardi/tuxbox-registry (da creare)
- **sshmenuc:** https://github.com/disoardi/sshmenuc

---

## ✅ Success Criteria Globali

### Phase 0 MVP - DONE quando:
- [x] Compila senza errori
- [x] `tbox --help` funziona
- [x] `tbox run sshmenuc` clona e esegue tool
- [x] Comandi list/status/update funzionano
- [x] Error handling base implementato
- [x] Colored output ovunque

### Phase 1 Venv - DONE quando:
- [ ] Auto-detect Python version
- [ ] Auto-create venv per tool Python
- [ ] Auto-install requirements.txt
- [ ] Tool eseguiti in venv isolato
- [ ] Backward compatibility mantenuta

### Phase 2 Registry - DONE quando:
- [ ] `tbox init <url>` clona registry
- [ ] Tool configs caricati da tools.toml
- [ ] `tbox list` mostra tutti i tool registry
- [ ] Fallback a hardcoded se registry non presente
- [ ] Registry repository su GitHub pubblico

### Infrastructure - DONE quando:
- [ ] Repository GitHub pubblico con codice
- [ ] CI/CD setup con GitHub Actions
- [ ] Tests (unit + integration) passing
- [ ] README.md completo e aggiornato
- [ ] Primo release tag v0.1.0

---

## 🎯 Come Usare Questo File

### Per Claude Code (AI Assistant)
Questo file è la tua **guida di implementazione**. Segui i task in ordine, usa le checklist, riferisci ai file indicati. Ogni task ha:
- Priority (🔴 Critico, 🟡 Alto, 🟢 Medio)
- Estimated effort
- Dependencies
- Checklist dettagliata
- Success criteria

### Per Davide (Owner)
Questo file è il **contratto del progetto**. Puoi:
- Verificare progress task per task
- Modificare priorità se necessario
- Aggiungere/rimuovere task
- Fornire feedback su implementation choices

### Workflow Iterativo
1. Claude Code lavora su un task alla volta
2. Completa checklist del task
3. Testa secondo success criteria
4. Commit changes
5. Report a Davide
6. Passa al task successivo

---

**Ultimo aggiornamento:** 2026-02-12
**Versione:** 1.0
**Status:** Ready for implementation
**Next task:** Task 1 - Prima Compilazione

Buon coding! 🦀✨
