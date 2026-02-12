# TuxBox - Note di Architettura

## 🏗️ Panoramica Architettura

TuxBox usa un'architettura modulare a 3 livelli:

```
┌─────────────────────────────────────────┐
│         CLI Layer (cli.rs)              │
│  Clap derive API - Parsing comandi      │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│      Business Logic Layer               │
│  ┌──────────┐  ┌──────────┐            │
│  │ runner.rs│  │ config.rs│            │
│  │ git.rs   │  │ error.rs │            │
│  └──────────┘  └──────────┘            │
└─────────────────┬───────────────────────┘
                  │
┌─────────────────▼───────────────────────┐
│       Infrastructure Layer              │
│  Git (git2), Filesystem, External CMD   │
└─────────────────────────────────────────┘
```

## 📦 Dettaglio Moduli

### main.rs - Entry Point
**Responsabilità:**
- Parse CLI args con Clap
- Dispatch comandi ai moduli appropriati
- Gestione output colored
- Error handling top-level

**Pattern usato:**
```rust
fn main() -> Result {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { tool, args } => runner::run_tool(&tool, &args),
        Commands::List => config::list_tools(),
        // ...
    }
}
```

**Nota:** main.rs è thin - tutta la logica nei moduli

### cli.rs - Command Line Interface
**Responsabilità:**
- Definizione struct Cli con Clap derive
- Subcommands: Init, List, Run, Update, Status
- Argument validation automatica Clap

**Pattern usato:** Clap 4.5 Derive API
```rust
#[derive(Parser)]
#[command(name = "tbox")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        tool: String,
        #[arg(trailing_var_arg = true)]
        args: Vec,
    },
    // ...
}
```

**Estendibilità:** Aggiungi comandi come varianti di `Commands` enum

### config.rs - Configuration Management
**Responsabilità:**
- Gestione directory ~/.tuxbox/
- ToolConfig struct (definizione tool)
- Context pattern (2026 best practice)
- List e status operations

**Strutture principali:**
```rust
pub struct ToolConfig {
    pub name: String,
    pub repo: String,
    pub branch: Option,
    pub tool_type: Option,
    pub isolation: Option,
    pub commands: Option,
}

pub struct Context {
    pub base_dir: PathBuf,      // ~/.tuxbox/
    pub tools_dir: PathBuf,     // ~/.tuxbox/tools/
    pub registry_dir: PathBuf,  // ~/.tuxbox/registry/
}
```

**Phase Evolution:**
- Phase 0: Hardcoded configs in runner.rs
- Phase 2: Load da registry TOML
- Phase 3: Dynamic discovery

### error.rs - Error Types
**Responsabilità:**
- Custom error types con thiserror
- Conversioni automatiche con From trait
- User-friendly error messages

**Pattern usato:** thiserror derive
```rust
#[derive(Error, Debug)]
pub enum TuxBoxError {
    #[error("Tool '{0}' not found")]
    ToolNotFound(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

**Estendibilità:** Aggiungi varianti per nuovi error case

### git.rs - Git Operations
**Responsabilità:**
- Clone repository (lazy loading)
- Update tools (git pull)
- Check tool existence
- Branch management

**Dependencies:** git2 crate

**Funzioni principali:**
```rust
pub fn clone_tool(config: &ToolConfig, dest: &Path) -> Result
pub fn is_tool_cloned(tool_dir: &Path) -> bool
pub fn update_tool(tool_dir: &Path) -> Result
pub fn update_all_tools(tools_dir: &Path) -> Result
```

**Edge cases gestiti:**
- Repository già esistente
- Network failures
- Branch non esistente
- Authentication (solo HTTPS public in MVP)

### runner.rs - Tool Execution
**Responsabilità:**
- Get tool config (hardcoded MVP, registry in future)
- Lazy clone se tool non presente
- Execute tool command
- Pass-through arguments

**Flow esecuzione:**
```
run_tool()
  ├─> get_tool_config()  [hardcoded per MVP]
  ├─> is_tool_cloned()?
  │   └─> NO: clone_tool()
  └─> std::process::Command
      └─> execute with args
```

**Configurazioni hardcoded (MVP):**
- `sshmenuc`: Python tool principale
- `test-tool`: Tool di test

**Phase 1 TODO:** Gestione venv per Python
**Phase 2 TODO:** Load da registry invece di hardcoded

## 🔄 Data Flow

### Scenario: `tbox run sshmenuc --help`

```
1. main.rs
   ├─> Clap parse args
   └─> Commands::Run { tool: "sshmenuc", args: ["--help"] }

2. runner::run_tool("sshmenuc", ["--help"])
   ├─> get_tool_config("sshmenuc")
   │   └─> Return ToolConfig { repo: "...", ... }
   │
   ├─> config::get_context()
   │   └─> Return Context { tools_dir: "~/.tuxbox/tools", ... }
   │
   ├─> git::is_tool_cloned("~/.tuxbox/tools/sshmenuc")?
   │   ├─> NO: git::clone_tool(config, dest)
   │   │   └─> git2::Repository::clone(...)
   │   └─> YES: skip clone
   │
   └─> std::process::Command::new("python")
       .arg("sshmenuc.py")
       .args(["--help"])
       .current_dir("~/.tuxbox/tools/sshmenuc")
       .spawn()
```

## 🎯 Design Decisions

### 1. Perché Rust e non Bash?
- **Type safety:** Catch errors at compile time
- **Cross-platform:** Windows + macOS + Linux
- **Performance:** Binary veloce, no interpreter overhead
- **Dependency management:** Cargo gestisce tutto
- **Ecosystem:** Crates mature (clap, git2, serde)

### 2. Perché Clap derive API?
- **Declarative:** Struct = CLI interface
- **Automatic validation:** Type system + attributes
- **Auto-generated help:** No maintenance
- **Best practice 2026:** Più idiomatico del builder API

### 3. Perché git2 e non git CLI?
- **No external dependency:** git non serve installato
- **Programmatic control:** Gestione errori migliore
- **Cross-platform:** Consistent behavior
- **Performance:** No subprocess overhead

### 4. Perché hardcoded in MVP?
- **Faster iteration:** No parser, no format decisions
- **Prove concept first:** Valida architettura base
- **Add complexity gradually:** Registry quando serve davvero
- **User feedback:** Capire use case prima di over-engineer

## 🚀 Evolution Path

### MVP (Phase 0) - Current
```rust
// runner.rs
fn get_tool_config(tool_name: &str) -> Result {
    match tool_name {
        "sshmenuc" => Ok(ToolConfig { ... }),
        _ => Err(ToolNotFound)
    }
}
```

### Phase 2 - Registry System
```rust
// registry.rs (new module)
fn load_registry() -> Result<HashMap> {
    let content = fs::read_to_string("~/.tuxbox/registry/tools.toml")?;
    toml::from_str(&content)
}

// runner.rs
fn get_tool_config(tool_name: &str) -> Result {
    let registry = load_registry()?;
    registry.get(tool_name)
        .cloned()
        .ok_or(ToolNotFound)
}
```

### Phase 3 - Dynamic Discovery
```rust
// registry.rs
async fn fetch_registry(url: &str) -> Result {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    serde_yaml::from_str(&content)
}

// Support multiple registries, caching, versioning
```

## 🔒 Security Considerations

### Current (MVP)
- ✅ No arbitrary code execution (tool configs hardcoded)
- ✅ HTTPS only (git2 default)
- ⚠️ No input validation on tool names (TODO)
- ⚠️ No sandboxing (tools run with user perms)

### TODO Future Phases
- [ ] Sanitize tool names (prevent path traversal)
- [ ] Verify registry signatures (GPG)
- [ ] Docker isolation option (Phase 3)
- [ ] Checksum verification per tools
- [ ] Allowlist trusted repositories

## 🧪 Testing Strategy

### Unit Tests (TODO)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tool_config_existing() {
        let config = get_tool_config("sshmenuc");
        assert!(config.is_ok());
    }

    #[test]
    fn test_get_tool_config_nonexistent() {
        let config = get_tool_config("fake-tool");
        assert!(matches!(config, Err(TuxBoxError::ToolNotFound(_))));
    }
}
```

### Integration Tests (TODO)
```rust
// tests/integration_test.rs
#[test]
fn test_clone_and_run_tool() {
    // Setup temp dir
    // Run tbox run test-tool
    // Verify output
    // Cleanup
}
```

### Manual Testing Checklist
Vedi `.claude/quick-start.md` sezione "Test Checklist"

## 🔧 Extending TuxBox

### Aggiungere un nuovo comando CLI
1. Aggiungi variante a `Commands` enum in `cli.rs`
2. Implementa logica in modulo appropriato o crea nuovo modulo
3. Aggiungi match arm in `main.rs`
4. Update README.md

Esempio:
```rust
// cli.rs
#[derive(Subcommand)]
pub enum Commands {
    // ...existing...
    Info { tool: String },  // NEW
}

// info.rs (new file)
pub fn show_info(tool_name: &str) -> Result {
    // Implementation
}

// main.rs
Commands::Info { tool } => info::show_info(&tool)?,
```

### Aggiungere un nuovo tool type
1. Estendi `tool_type` in `ToolConfig`
2. Implementa detection logic in `runner.rs`
3. Aggiungi setup commands specifici

Esempio per Node.js:
```rust
pub fn run_tool(tool_name: &str, args: &[String]) -> Result {
    let config = get_tool_config(tool_name)?;

    let command = match config.tool_type.as_deref() {
        Some("python") => "python",
        Some("node") => "node",  // NEW
        _ => return Err(TuxBoxError::UnsupportedToolType),
    };

    // ...
}
```

### Aggiungere isolation strategy
Vedi `IsolationStrategy` enum in `config.rs`:
```rust
pub enum IsolationStrategy {
    None,
    Venv,      // Python virtualenv
    Docker,    // Docker container (Phase 3)
    Nix,       // Nix shell (future?)
}
```

Implementa in `runner.rs` con pattern matching.

## 📚 Riferimenti Codice

### Pattern Context (config.rs)
Rust 2026 best practice per dependency injection:
```rust
pub struct Context {
    pub base_dir: PathBuf,
    // ...altri campi...
}

impl Context {
    pub fn new() -> Result {
        // Initialization logic
    }
}

// Uso:
pub fn list_tools() -> Result {
    let ctx = Context::new()?;
    // usa ctx.tools_dir, etc.
}
```

### Pattern Builder per Commands
```rust
std::process::Command::new("python")
    .arg("script.py")
    .args(user_args)
    .current_dir(tool_dir)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()?
    .wait()?;
```

### Pattern Error Propagation
```rust
// ✅ Use ? operator
pub fn run_tool(name: &str) -> Result {
    let config = get_tool_config(name)?;  // Propagate error
    let ctx = Context::new()?;
    clone_if_needed(&config, &ctx.tools_dir)?;
    execute(&config, &ctx)?;
    Ok(())
}

// ❌ Don't unwrap in library code
let config = get_tool_config(name).unwrap();  // NO!
```

---

**Note:** Questo documento evolve con il progetto. Update quando aggiungi features o cambi architettura.