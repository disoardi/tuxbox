# 🤖 Claude Code Documentation Hub

Benvenuto nella documentazione per lo sviluppo di TuxBox con Claude Code!

## 📚 Documenti Disponibili

### 🚀 [quick-start.md](quick-start.md)
**Inizia da qui!** Primi 5 minuti, checklist test, comandi essenziali.

**Contiene:**
- ✅ Primi 3 comandi da eseguire
- ✅ Test checklist completa
- ✅ Common issues & solutions
- ✅ Git workflow
- ✅ Next steps dopo MVP

**Quando usarlo:** Prima compilazione, test rapidi, troubleshooting comuni

---

### 📖 [CLAUDE_CODE_HANDOFF.md](../CLAUDE_CODE_HANDOFF.md)
**Documento principale** con contesto completo del progetto e prompt di sviluppo strutturati.

**Contiene:**
- 🎯 Contesto e obiettivi progetto
- 📊 Stato attuale e TODO
- 🚀 4 prompt di sviluppo guidati:
  1. Prima compilazione
  2. Testing end-to-end
  3. Gestione errori Python
  4. Polish e UX
- 📚 Linee guida sviluppo
- 🐛 Known issues
- 💡 Tips per Claude Code

**Quando usarlo:** Per capire il big picture, prendere decisioni architetturali, seguire roadmap

---

### 🏗️ [architecture-notes.md](architecture-notes.md)
**Deep dive** nell'architettura TuxBox, pattern usati, design decisions.

**Contiene:**
- 📦 Dettaglio tutti i moduli (main, cli, config, error, git, runner)
- 🔄 Data flow diagrams
- 🎯 Design decisions e rationale
- 🚀 Evolution path (Phase 0 → 1 → 2 → 3)
- 🔒 Security considerations
- 🧪 Testing strategy
- 🔧 How to extend TuxBox

**Quando usarlo:** Prima di modifiche complesse, aggiungere features, refactoring

---

### 📖 [cli-reference.md](cli-reference.md)
**Reference completa** di tutti i comandi CLI, comportamento atteso, esempi.

**Contiene:**
- 📖 Tutti i comandi con syntax e output
- 🎯 Tool hardcoded (sshmenuc, test-tool)
- 🌈 Colored output e exit codes
- 📂 Directory structure
- 💡 Tips & tricks
- 📝 Esempi uso reale

**Quando usarlo:** Per verificare comportamento atteso, scrivere tests, debugging

---

## 🎯 Quick Navigation

### "Devo compilare per la prima volta"
→ [quick-start.md](quick-start.md) sezione "Prima Compilazione"

### "Devo capire come funziona il modulo X"
→ [architecture-notes.md](architecture-notes.md) sezione "Dettaglio Moduli"

### "Devo aggiungere una nuova feature"
→ [architecture-notes.md](architecture-notes.md) sezione "Extending TuxBox"

### "Ho un errore durante il run"
→ [quick-start.md](quick-start.md) sezione "Common Issues"

### "Voglio seguire il workflow guidato"
→ [CLAUDE_CODE_HANDOFF.md](../CLAUDE_CODE_HANDOFF.md) sezione "Prompt di Sviluppo"

### "Devo verificare come dovrebbe comportarsi un comando"
→ [cli-reference.md](cli-reference.md)

---

## 🧭 Workflow Consigliato

### 1️⃣ Primo Approccio (Mai visto il progetto)
```
1. Leggi CLAUDE_CODE_HANDOFF.md (contesto generale)
2. Segui quick-start.md (primi 3 comandi)
3. Se compila: procedi con testing
4. Se errori: consulta "Common Issues" in quick-start.md
```

### 2️⃣ Development Flow (Progetto già compilato)
```
1. Identifica task (da TODO in CLAUDE_CODE_HANDOFF.md)
2. Leggi architecture-notes.md per modulo coinvolto
3. Modifica codice
4. Test con quick-start.md checklist
5. Commit con git workflow (quick-start.md)
```

### 3️⃣ Debug Flow (Qualcosa non funziona)
```
1. Consulta cli-reference.md per comportamento atteso
2. Verifica con test checklist in quick-start.md
3. Se issue noto: vedi "Common Issues" in quick-start.md
4. Se issue nuovo: aggiungi a "Known Issues" in architecture-notes.md
```

### 4️⃣ Feature Addition (Nuova funzionalità)
```
1. Leggi "Extending TuxBox" in architecture-notes.md
2. Segui evolution path appropriato (Phase 0/1/2/3)
3. Mantieni coerenza con design decisions esistenti
4. Update tutti i documenti rilevanti:
   - architecture-notes.md (se cambia architettura)
   - cli-reference.md (se nuovo comando CLI)
   - README.md del progetto (user-facing)
```

---

## 📋 Checklist Modifiche Documentazione

Quando modifichi TuxBox, aggiorna anche:

- [ ] `README.md` (root del progetto) - se cambia user experience
- [ ] `CLAUDE_CODE_HANDOFF.md` - se cambia stato progetto o TODO
- [ ] `architecture-notes.md` - se cambi architettura o moduli
- [ ] `cli-reference.md` - se cambi comandi o comportamento CLI
- [ ] `quick-start.md` - se cambi workflow o aggiungi common issues

**Principio:** Documentation is code. Deve evolvere insieme al progetto.

---

## 🎓 Contesto IdeaFlow

TuxBox nasce da IdeaFlow framework (5 fasi: CAPTURE → ELABORATE → VALIDATE → DOCUMENT → PREPARE).

**Storia progetto:**
1. **CAPTURE** (idea-001-raw.md): Meta-tool per gestire tool distribuiti
2. **ELABORATE** (idea-001-elaborated.md): Analisi 1000+ righe, architettura, roadmap
3. **VALIDATE**: Decisione GO - implementazione immediata in Rust
4. **IMPLEMENT** (current): MVP Phase 0 con Claude Code

**Tracking:**
- File tracker: `~/Progetti/silverbullet/space/Idee/idea-tracker.md`
- Raw idea: `~/Progetti/silverbullet/space/Idee/ideas/idea-001-toolbox-raw.md`
- Elaboration: `~/Progetti/silverbullet/space/Idee/ideas/idea-001-toolbox-elaborated.md`

---

## 🤝 Handoff Protocol

### Questo progetto è stato iniziato da Claude (Cowork mode)

**Setup completato:**
- ✅ Ricerca best practices Rust 2026
- ✅ Struttura progetto completa (9 file Rust + docs)
- ✅ Git inizializzato con commit iniziale
- ✅ Documentazione handoff completa

**Ora tocca a te (Claude Code):**
- [ ] Prima compilazione
- [ ] Testing end-to-end
- [ ] Bug fixing e polish
- [ ] Iterazione features

**Se hai dubbi:**
1. Consulta docs in `.claude/`
2. Leggi commenti in codice
3. Verifica git history: `git log --oneline`

**Principio chiave:** Questo è un progetto vero, non un esercizio. Davide lo userà davvero per gestire i suoi tool. Quality matters! 🚀

---

## 📞 Contatti & Risorse

**Owner:** Davide Isoardi
**Email:** davide.isoardi@dxc.com
**GitHub:** https://github.com/disoardi

**Repository (TODO - non ancora creato):**
- Main: https://github.com/disoardi/tuxbox
- Registry: https://github.com/disoardi/tuxbox-registry

**Tool di riferimento:**
- sshmenuc: https://github.com/disoardi/sshmenuc

---

**Ultimo aggiornamento:** 2026-02-12
**Versione docs:** 1.0
**Project phase:** MVP Phase 0 - Implementation

Buon coding! 🦀✨