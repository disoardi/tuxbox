# TuxBox - Claude Code Implementation Guide

## Obiettivo Progetto

**TuxBox** è un meta-tool CLI in Rust per gestire tool personali distribuiti su repository Git. Permette di scaricare, configurare e lanciare tool da un'unica interfaccia, con lazy loading automatico.

**Binary name:** `tbox`  
**Storage:** `~/.tuxbox/`

---

## Stato Corrente

- **Versione:** v0.2.25
- **Branch:** main
- **Remote primario:** Forgejo locale (`origin`) — vedi [.claude/directives-git.md](.claude/directives-git.md)
- **Release pubbliche:** https://github.com/disoardi/tuxbox/releases

### Features Implementate
- Dual-mode execution (Docker-first + venv fallback)
- Auto-setup completo (zero-config per utente)
- Docker container management (auto-build, custom Dockerfile detection)
- `system_deps` nel registry — pacchetti apt-get installati nell'auto-generated Dockerfile
- `isolation` field nel registry — override esplicito della strategia di esecuzione per tool
- Python venv fallback (auto-create, auto-install requirements, pyenv support)
- Multi-registry support (public + private)
- Self-update mechanism (GitHub API)
- SSH repository support (system git fallback)
- Bash script execution (direct)
- Native binary download (type = "native")
- Tool state file per fast path (skip reinstall)
- `tbox status` con versione + branch per ogni tool
- `tbox reinstall <tool>` — rimuove dir e ri-scarica
- `tbox delete <tool>` — rimuove dir con conferma

### isolation Field

Tool che spawnan processi interattivi (SSH, tmux, editor) dichiarano `isolation = "venv"` per evitare Docker-first.

| Valore | Comportamento |
|--------|---------------|
| `venv` | sempre LocalVenv, Docker ignorato |
| `docker` | sempre Docker |
| `none` | LocalVenv (no container) |
| assente | auto-detect (Docker se disponibile) |

### Roadmap

| Phase | Status |
|-------|--------|
| Phase 0 — MVP (clone + run) | ✅ Done |
| Phase 1 — Python venv + Docker | ✅ Done |
| Phase 2a — Multi-Registry | ✅ Done |
| Infrastructure (CI/CD, self-update, SSH, bash, native) | ✅ Done |
| Phase 3 — Future (plugin, TUI, multi-lang) | 📅 Backlog |

### Pending
- [ ] Test self-update end-to-end
- [ ] CI su Forgejo (Gitea Actions in `.forgejo/workflows/`)
- [ ] Espandere registry con tool personali
- [ ] Validazione registry in CI

---

## Direttive Operative

Le regole operative sono nei file dedicati in `.claude/`:

| File | Contenuto |
|------|-----------|
| [directives-git.md](.claude/directives-git.md) | Setup 3-remote, workflow quotidiano, commit convention, issue tracking, gotcha Forgejo |
| [directives-cicd.md](.claude/directives-cicd.md) | GitHub Actions best practices, release checklist |
| [directives-rust-patterns.md](.claude/directives-rust-patterns.md) | Coding standards, git2 StatusOptions, venv activation, tool state file |
| [directives-tool-types.md](.claude/directives-tool-types.md) | Matrice python/bash/native/docker, SSH support |
| [directives-registry.md](.claude/directives-registry.md) | Python version policy, aggiungere tool, troubleshooting |

---

## Codebase

### Moduli Rust
- `src/main.rs` — Entry point
- `src/cli.rs` — Clap CLI definitions
- `src/config.rs` — Configuration, Context, `show_status()`
- `src/error.rs` — Custom error types
- `src/git.rs` — Git operations (clone, update, SSH/HTTPS dispatch)
- `src/runner.rs` — Tool execution dispatch
- `src/python.rs` — Python detection, venv setup, execute_in_venv
- `src/tool_state.rs` — State file read/write/invalidate
- `src/registry.rs` — Registry load, tool lookup
- `src/docker.rs` — Docker build/run
- `src/native.rs` — Native binary download/update/execute
- `src/selfupdate.rs` — Self-update via GitHub API

### Documentazione in `.claude/`
- [README.md](.claude/README.md) — Hub documentazione
- [quick-start.md](.claude/quick-start.md) — Comandi essenziali, common issues
- [architecture-notes.md](.claude/architecture-notes.md) — Deep dive architettura
- [cli-reference.md](.claude/cli-reference.md) — Riferimento comandi CLI

---

## PR Workflow

1. Crea issue su Forgejo via API curl (vedi `.claude/directives-git.md`)
2. `git checkout -b feature/<nome>`
3. Implementa + `cargo clippy -- -D warnings` + `cargo test`
4. Push branch + crea PR via API Forgejo
5. Review con `Agent(subagent_type="code-reviewer")` — passa diff completo e contesto
6. Fix feedback → push → secondo giro review fino ad APPROVE
7. Merge via API Forgejo (`POST /pulls/{n}/merge`)
8. `git checkout main && git pull origin main`
9. Bump versione + tag + push origin + push github-public

---

## Principi Architetturali

1. Modularità — ogni feature in modulo separato
2. Error handling robusto — `Result<T, TuxBoxError>` ovunque, no `unwrap` in produzione
3. Error messages chiari e actionable
4. Backward compatibility — non rompere workflow esistenti
5. SSH → sempre system git (mai git2 per SSH)
6. Subprocess output: `.output()` per catturare, `.status()` solo se si vuole output visibile

---

## Testing

- **Unit tests:** `#[cfg(test)] mod tests` in ogni modulo
- **Integration tests:** `tests/` per workflow end-to-end
- **Manual testing:** `tbox list / run / update / status` dopo ogni modifica
- **CI:** `cargo fmt && cargo clippy -- -D warnings && cargo test` su ogni push

---

**Ultimo aggiornamento:** 2026-05-22  
**Versione:** v0.2.25
