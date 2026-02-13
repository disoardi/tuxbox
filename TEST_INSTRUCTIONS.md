# Test TuxBox da Zero - Istruzioni per Davide

Esegui questi comandi in sequenza per testare TuxBox come se fosse la prima volta.

---

## ✅ Pre-requisiti Verificati

- [x] SSH passwordless configurato per github.dxc.com
- [x] Repository registry esistente: `tuxbox-registry-private`
- [x] TuxBox compilato in `~/Progetti/tuxbox`

---

## 🧪 Test Sequence

### 1. Verifica SSH (deve funzionare senza password)

```bash
ssh -T git@github.dxc.com
```

**Output atteso:** Connessione riuscita senza chiedere password

---

### 2. Inizializza TuxBox con il registry

```bash
cd ~/Progetti/tuxbox
cargo run -- init git@github.dxc.com:disoardi/tuxbox-registry-private.git
```

**Output atteso:**
```
→ Initializing TuxBox...
✓ TuxBox initialized successfully!
```

**Cosa succede:**
- TuxBox crea `~/.tuxbox/` directory
- Salva configurazione registry in `~/.tuxbox/config.toml`
- **NON clona ancora il registry** (lazy loading)

---

### 3. Verifica stato TuxBox

```bash
cargo run -- status
```

**Output atteso:**
```
TuxBox Status
=============

Base directory: /Users/disoardi/.tuxbox
Tools directory: /Users/disoardi/.tuxbox/tools

Registries: (1 configured):
  🔐 tuxbox-registry-private (priority: 100) - git@github.dxc.com:...

Tools: No tools installed yet.
```

**Cosa vedi:**
- Registry configurato (🔐 = SSH)
- Nessun tool installato ancora

---

### 4. Lista tools (trigger auto-sync registry)

```bash
cargo run -- list
```

**Output atteso:**
```
→ Available tools:
Configured registries:
  🔐 tuxbox-registry-private (priority: 100)

No tools found in registries. Run 'tbox registry sync' to fetch.
```

**Nota:** Il registry non è ancora clonato, quindi non mostra tool.

---

### 5. Sincronizza registry (clone automatico)

```bash
cargo run -- registry sync
```

**Output atteso:**
```
→ Syncing all registries...

→ Registry: tuxbox-registry-private
  → Cloning registry 'tuxbox-registry-private'...
  → git2 failed, trying system git command...
Cloning into '/Users/disoardi/.tuxbox/registry/tuxbox-registry-private'...
  ✓ Registry cloned successfully (via git command)
  ✓ 1 tools available

✓ All registries synced!
```

**Cosa succede:**
- TuxBox clona il registry da GitHub Enterprise
- Usa fallback git command (git2 fallisce con GHE)
- Legge tools.toml e trova 1 tool (sshmenuc)

---

### 6. Lista tools (ora mostra i tool dal registry)

```bash
cargo run -- list
```

**Output atteso:**
```
→ Available tools:
Configured registries:
  🔐 tuxbox-registry-private (priority: 100) - git@github.dxc.com:...

Available tools from registries:
  • sshmenuc - SSH connection manager with interactive TUI menu (from tuxbox-registry-private)
```

**Cosa vedi:**
- Tool caricato dal registry clonato
- Descrizione presa da tools.toml

---

### 7. Esegui tool (primo run - clone tool automatico)

```bash
cargo run -- run sshmenuc
```

**Output atteso:**
```
→ Running tool: sshmenuc
  → Found in registry: tuxbox-registry-private
  Tool not installed, cloning...
  → Cloning sshmenuc from https://github.com/disoardi/sshmenuc...
  ✓ Cloned successfully
  🐳 Using Docker for isolated execution
  → Building Docker image...
  [... Docker build output ...]
  ✓ Image built successfully
  → Running in container...
  [sshmenuc interface]
```

**Cosa succede (completamente automatico):**
1. TuxBox trova tool nel registry
2. Clona il repository sshmenuc
3. Rileva che è tool Python
4. Usa Docker per esecuzione isolata
5. Build immagine Docker
6. Esegue il tool in container

---

### 8. Esegui tool (secondo run - tutto già pronto)

```bash
cargo run -- run sshmenuc
```

**Output atteso:**
```
→ Running tool: sshmenuc
  → Found in registry: tuxbox-registry-private
  🐳 Using Docker for isolated execution
  → Running in container...
  [sshmenuc interface - istantaneo]
```

**Cosa succede:**
- Skip clone (già presente)
- Skip build Docker (immagine già creata)
- Esecuzione istantanea

---

### 9. Verifica stato finale

```bash
cargo run -- status
```

**Output atteso:**
```
TuxBox Status
=============

Base directory: /Users/disoardi/.tuxbox
Tools directory: /Users/disoardi/.tuxbox/tools

Registries: (1 configured):
  🔐 tuxbox-registry-private (priority: 100)

Tools: 1 installed tools:
  • sshmenuc
```

---

### 10. Verifica struttura filesystem

```bash
tree ~/.tuxbox -L 3
```

**Output atteso:**
```
/Users/disoardi/.tuxbox
├── config.toml                           # Config TuxBox
├── registry
│   └── tuxbox-registry-private          # Registry clonato
│       ├── README.md
│       └── tools.toml                   # Tool definitions
└── tools
    └── sshmenuc                         # Tool clonato
        ├── README.md
        ├── pyproject.toml
        └── sshmenuc/
```

---

## ✅ Checklist Risultati

Dopo aver eseguito tutti i comandi, verifica:

- [ ] TuxBox inizializzato senza errori
- [ ] Registry clonato automaticamente
- [ ] Tool sshmenuc visibile in `tbox list`
- [ ] Tool sshmenuc eseguito correttamente (primo run con build Docker)
- [ ] Tool sshmenuc eseguito istantaneamente (secondo run)
- [ ] Nessun intervento manuale richiesto
- [ ] Tutto automatico da `tbox init` a esecuzione

---

## 🎯 Cosa Dimostra Questo Test

1. **Zero-Config User Experience:**
   - `tbox init <url>` → salva solo config
   - `tbox run <tool>` → fa tutto automaticamente

2. **Lazy Loading:**
   - Registry clonato solo quando necessario
   - Tool clonati solo al primo run

3. **Smart Fallback:**
   - git2 fallisce → automatico fallback a git command
   - Nessun errore per l'utente

4. **Docker Automation:**
   - Auto-detect tool type
   - Auto-build immagine
   - Auto-run in container

5. **Registry-Based Resolution:**
   - Tool definiti in registry remoto
   - Update registry → nuovi tool disponibili
   - Nessun hardcode nel codice TuxBox

---

## 🐛 Troubleshooting

### Se SSH chiede password

```bash
# Rimuovi password dalla chiave SSH
ssh-keygen -p -f ~/.ssh/id_ed25519
# Premi Enter per passphrase vuota

# Aggiungi all'agent
ssh-add ~/.ssh/id_ed25519
```

### Se registry sync fallisce

```bash
# Test manuale git clone
git clone git@github.dxc.com:disoardi/tuxbox-registry-private.git /tmp/test-registry

# Se fallisce, problema SSH config
```

### Se tool non parte

```bash
# Verifica Docker running
docker ps

# Force rebuild immagine
rm -rf ~/.tuxbox/tools/sshmenuc
cargo run -- run sshmenuc
```

---

**Fine test!** 🎉

Se tutto funziona, TuxBox è pronto per uso produttivo.
