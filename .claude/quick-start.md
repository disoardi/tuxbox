# TuxBox - Quick Start per Claude Code

## 🚀 Primi 5 Minuti

### 1. Prima Compilazione
```bash
cd ~/Progetti/tuxbox
cargo build --release
```

**Output atteso:** Compilazione pulita con possibili warning (normali)
**Se errori:** Leggi CLAUDE_CODE_HANDOFF.md sezione "Prompt di Sviluppo - Prima Compilazione"

### 2. Verifica Help
```bash
cargo run -- --help
```

**Output atteso:**
```
Usage: tbox <COMMAND>

Commands:
  init    Initialize TuxBox with a registry URL
  list    List all available tools
  run     Run a specific tool
  update  Update one or all tools
  status  Show TuxBox status
  help    Print this message or the help of the given subcommand(s)
```

### 3. Test Run sshmenuc
```bash
cargo run -- run sshmenuc
```

**Comportamento atteso:**
1. Clone automatico da GitHub in `~/.tuxbox/tools/sshmenuc/`
2. Tentativo esecuzione `python sshmenuc.py`
3. Possibile errore "requirements.txt not installed" → NORMALE in Phase 0

### 4. Setup Manuale (se necessario)
```bash
cd ~/.tuxbox/tools/sshmenuc
pip install -r requirements.txt
```

Poi ri-testa:
```bash
cd ~/Progetti/tuxbox
cargo run -- run sshmenuc
```

### 5. Verifica Altri Comandi
```bash
cargo run -- list     # Lista tool clonati
cargo run -- status   # Status TuxBox
```

## 🎯 Task Priority List

### Priority 1 (Must Have - MVP)
- [ ] Fix eventuali errori di compilazione
- [ ] Verifica git clone funzioni
- [ ] Verifica esecuzione tool Python base
- [ ] Error messages user-friendly

### Priority 2 (Should Have - Polish)
- [ ] Colored output implementato ovunque
- [ ] Progress indicators durante clone
- [ ] Gestione edge case (tool non esiste, network fail, etc.)
- [ ] Help text con esempi

### Priority 3 (Nice to Have - Future)
- [ ] Comando `init` funzionante
- [ ] Auto-detect Python version (python vs python3)
- [ ] Validazione input (security)
- [ ] Tests unitari

## 🧪 Test Checklist

```bash
# Test 1: Compilazione
cargo build --release
# ✅ Deve compilare senza errori

# Test 2: Help generale
cargo run -- --help
# ✅ Mostra tutti i comandi

# Test 3: Help specifico
cargo run -- run --help
# ✅ Mostra usage: tbox run  [ARGS]...

# Test 4: Tool non esistente
cargo run -- run nonexistent
# ✅ Errore chiaro: "Tool 'nonexistent' not found"

# Test 5: Clone e run
cargo run -- run sshmenuc
# ✅ Clone + tentativo esecuzione

# Test 6: Secondo run (già clonato)
cargo run -- run sshmenuc
# ✅ Skip clone, esecuzione diretta

# Test 7: List tools
cargo run -- list
# ✅ Mostra sshmenuc se clonato

# Test 8: Status
cargo run -- status
# ✅ Mostra configurazione e tool installati
```

## 🔧 Comandi Utili Rust

```bash
# Build release ottimizzato
cargo build --release

# Run con args
cargo run -- run sshmenuc --help

# Check senza compilare (veloce)
cargo check

# Format code
cargo fmt

# Lint con clippy
cargo clippy

# Run tests (quando ci saranno)
cargo test

# Clean build artifacts
cargo clean

# Tree dipendenze
cargo tree

# Update dependencies
cargo update
```

## 📝 Git Workflow

```bash
# Check status
git status

# Commit dopo fix
git add .
git commit -m "fix: descrizione del fix

Dettagli del cambiamento...

Co-Authored-By: Claude Sonnet 4.5 "

# Push quando pronto
git remote add origin https://github.com/disoardi/tuxbox.git
git push -u origin main
```

## 🐛 Common Issues & Solutions

### Issue: "cargo: command not found"
**Solution:** Installa Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Issue: "error: linking with `cc` failed"
**Solution:** Installa build essentials: `xcode-select --install` (macOS)

### Issue: "failed to clone: authentication failed"
**Solution:** sshmenuc è pubblico, verifica connessione internet

### Issue: "python: command not found"
**Solution:** Tool Python richiede Python installato, usa `python3` se necessario

### Issue: "No such file or directory: requirements.txt"
**Solution:** Normal in Phase 0, fai setup manuale (vedi Quick Start punto 4)

## 📊 Next Steps dopo MVP

1. **Phase 1 - Venv Support:**
   - Auto-creazione venv per Python tools
   - Auto-install requirements.txt
   - Isolation completa dipendenze

2. **Phase 2 - Registry System:**
   - TOML registry su GitHub
   - Auto-discovery nuovi tool
   - Versioning tool

3. **Phase 3 - Advanced Features:**
   - Docker isolation
   - Multi-language support (Go, Node.js, etc.)
   - Plugin system

## 🎓 Learning Resources

- **Clap Examples:** `cargo doc --open` poi cerca clap
- **git2 Examples:** https://github.com/rust-lang/git2-rs/tree/master/examples
- **Error Handling:** https://doc.rust-lang.org/book/ch09-00-error-handling.html
- **Testing in Rust:** https://doc.rust-lang.org/book/ch11-00-testing.html

---

**Pro Tip:** Usa `cargo watch -x run` per auto-rebuild durante sviluppo (richiede `cargo install cargo-watch`)
