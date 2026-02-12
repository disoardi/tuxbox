# TuxBox - Claude Code Development Handoff

## 🎯 Contesto Progetto

TuxBox è un meta-tool CLI in Rust per gestire tool personali distribuiti su repository Git. Permette di scaricare, configurare e lanciare tool da un'unica interfaccia, con lazy loading automatico.

**Binary name:** `tbox` (da package `tuxbox`)
**Caso d'uso primario:** sshmenuc (Python tool con requirements.txt)

## 📊 Stato Attuale

### ✅ Completato
- [x] Struttura progetto creata con architettura modulare
- [x] Tutti i moduli Rust implementati (main, cli, config, error, git, runner)
- [x] Git repository inizializzato (commit d72ea7c su main)
- [x] Configurazione hardcoded per sshmenuc in MVP Phase 0
- [x] Dependencies 2026: Clap 4.5, git2, anyhow/thiserror

### 🔄 In Progress
- [ ] Prima compilazione e testing
- [ ] Verifica funzionamento comando `tbox run sshmenuc`
- [ ] Debug eventuali errori di compilazione

### 📋 TODO
- [ ] Testing end-to-end del workflow completo
- [ ] Gestione errori edge case
- [ ] Phase 1: Supporto venv per Python tools
- [ ] Setup GitHub Pages per documentazione
- [ ] Push su repository GitHub

## 🚀 Prompt di Sviluppo - Prima Compilazione

```
Ciao Claude Code! Sono Davide e sto sviluppando TuxBox, un CLI tool in Rust.

TASK IMMEDIATO:
1. Compila il progetto: cargo build --release
2. Verifica output per eventuali errori o warning
3. Se ci sono errori di compilazione, correggili seguendo Rust 2026 best practices
4. Testa il comando: cargo run -- --help
5. Verifica che mostri correttamente i 5 subcommands (init, list, run, update, status)

CONTESTO TECNICO:
- Edition 2024, dipendenze moderne (vedi Cargo.toml)
- Architettura modulare già implementata
- MVP Phase 0: focus su clone automatico + esecuzione tool

Se tutto compila correttamente, procedi con il prossimo prompt.
```

## 🧪 Prompt di Sviluppo - Testing End-to-End

```
TASK: Test del workflow completo con sshmenuc

1. Esegui: cargo run -- run sshmenuc
   - Dovrebbe clonare automaticamente da https://github.com/disoardi/sshmenuc
   - Storage: ~/.tuxbox/tools/sshmenuc/
   - Eseguire: python sshmenuc.py

2. Osserva l'output e verifica:
   - Clone Git funziona? (modulo git.rs)
   - Directory ~/.tuxbox/tools/ creata?
   - Tool viene eseguito correttamente?
   - Errori di permessi o path?

3. Se ci sono problemi:
   - Aggiungi logging per debug (usa colored crate già presente)
   - Gestisci errori con proper error messages (thiserror già configurato)
   - Verifica che git2 crate funzioni correttamente

4. Test comando list:
   cargo run -- list
   (dovrebbe mostrare solo sshmenuc se già clonato)

5. Test comando status:
   cargo run -- status
   (dovrebbe mostrare info repository)

IMPORTANTE: Non modificare l'architettura generale, solo fix di bug e miglioramenti incrementali.
```

## 🔧 Prompt di Sviluppo - Gestione Errori Python

```
TASK: Migliorare gestione esecuzione tool Python

Il caso d'uso primario è sshmenuc che richiede:
- requirements.txt da installare
- Esecuzione: python sshmenuc.py

PROBLEMI DA RISOLVERE:
1. Come gestire l'installazione di requirements.txt?
   - Opzione A (MVP): Messaggio all'utente "Run: cd ~/.tuxbox/tools/sshmenuc && pip install -r requirements.txt"
   - Opzione B (Phase 1): Auto-install in venv (TODO futuro)

2. Verifica che Python sia disponibile nel sistema
   - Check before execution
   - Error message chiaro se mancante

3. Passaggio argomenti da tbox a tool:
   - Verifica che args: Vec<String> vengano passati correttamente
   - Test: tbox run sshmenuc --help (dovrebbe mostrare help di sshmenuc)

IMPLEMENTAZIONE:
- Modifica runner.rs per aggiungere check Python
- Aggiungi messaggio informativo per setup manuale requirements
- Testa con sshmenuc reale

Phase 1 (futuro): Implementare auto-setup con venv, per ora mantieni approccio manuale.
```

## 🎨 Prompt di Sviluppo - Polish e UX

```
TASK: Migliorare UX e output utente

1. Colored output già configurato, assicurati sia usato:
   - Success messages: verde
   - Error messages: rosso
   - Info messages: blu/cyan
   - Warning messages: giallo

2. Progress indicators:
   - Durante git clone: "Cloning tool 'sshmenuc'..."
   - Durante esecuzione: "Running tool 'sshmenuc'..."

3. Help text migliorato:
   - Aggiungi esempi d'uso in cli.rs
   - Usa feature wrap_help di Clap (già in Cargo.toml)

4. Error messages user-friendly:
   - "Tool 'xyz' not found. Available tools: [list]"
   - "TuxBox not initialized. Run: tbox init <registry-url>"
   - "Git clone failed: [reason]. Check network connection."

5. First-run experience:
   - Se ~/.tuxbox/ non esiste, suggerisci: tbox init
   - (Nota: init non ancora implementato in Phase 0, può essere TODO)

RICORDA: Mantieni tono tecnico ma accessibile, questo è un tool per developer.
```

## 📚 Linee Guida Sviluppo

### Principi Architetturali
1. **Soluzione lean:** Implementa solo quello che serve, no over-engineering
2. **Multi-file organization:** Mantieni separazione moduli (già fatto)
3. **Documentazione esaustiva:** README e commenti in-code chiari
4. **Rust 2026 best practices:** Edition 2024, Clap derive API, thiserror

### Roadmap MVP
- **Phase 0** (CURRENT): Clone + Run base, configs hardcoded
- **Phase 1** (NEXT): Virtual env support per Python
- **Phase 2**: Registry system con TOML/YAML
- **Phase 3**: Docker isolation, più tool types

### Don't Break
- Architettura modulare esistente (7 moduli)
- API Clap derive (non tornare a builder)
- Storage path: ~/.tuxbox/tools/
- Binary name: tbox

### Code Style
```rust
// ✅ DO: Error handling esplicito
let tool = get_tool_config(tool_name)?;

// ✅ DO: Pattern matching per chiarezza
match result {
    Ok(val) => println!("Success: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}

// ❌ DON'T: unwrap() o expect() senza motivo
let tool = get_tool_config(tool_name).unwrap();  // NO!

// ✅ DO: Colored output dove appropriato
println!("{}", "Success!".green());

// ✅ DO: Documentazione funzioni pubbliche
/// Clones a tool from its Git repository
/// Returns the local path where the tool was cloned
pub fn clone_tool(config: &ToolConfig) -> Result
```

## 🐛 Known Issues / Edge Cases

1. **Git authentication:** Attualmente supporta solo HTTPS pubblici, no SSH keys o auth
   - TODO: Gestire repo privati in futuro

2. **Python version:** Non specifica quale Python (python, python3, python3.11?)
   - TODO: Permettere configurazione per tool

3. **Requirements.txt:** Non auto-installato in Phase 0
   - User deve fare setup manuale prima run

4. **Tool naming:** Nessuna validazione nomi tool (potenziali path traversal?)
   - TODO: Sanitize input in config.rs

5. **Concurrent access:** Nessun lock su ~/.tuxbox/
   - TODO: File locking se serve

## 📖 Risorse Utili

- **Cargo.toml:** Tutte le dipendenze già configurate
- **README.md:** Documentazione utente completa
- **src/error.rs:** Custom error types disponibili
- **src/config.rs:** ToolConfig struct e Context pattern

## 🎯 Success Criteria

### MVP Phase 0 è completo quando:
- [ ] `cargo build --release` compila senza errori
- [ ] `tbox --help` mostra tutti i comandi
- [ ] `tbox run sshmenuc` clona e lancia il tool
- [ ] Gestione errori chiara e user-friendly
- [ ] README allineato con implementazione reale

## 💡 Tips per Claude Code

1. **Leggi prima di modificare:** Controlla tutti i moduli per capire le interdipendenze
2. **Test incrementali:** Compila dopo ogni modifica, non accumulare cambiamenti
3. **Commit frequenti:** Git già inizializzato, usa commit descrittivi in italiano
4. **Mantieni coerenza:** Segui lo stile esistente nel codice
5. **Documentazione sync:** Se cambi codice, aggiorna README se necessario

## 🔗 Repository e Risorse

- **sshmenuc repo:** https://github.com/disoardi/sshmenuc
- **Future registry:** https://github.com/disoardi/tuxbox-registry (TODO)
- **Rust Book 2024:** https://doc.rust-lang.org/book/
- **Clap 4.5 docs:** https://docs.rs/clap/latest/clap/

---

**Ultimo aggiornamento:** 2026-02-12
**Creato da:** Claude (Cowork mode)
**Per:** Claude Code (IDE mode)

Buon coding! 🦀