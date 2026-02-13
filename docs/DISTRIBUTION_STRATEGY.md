# TuxBox - Distribution Strategy

## 🎯 Obiettivo

Definire la strategia di distribuzione di TuxBox e i suoi tool, bilanciando facilità d'uso, sicurezza e flessibilità.

---

## 📦 Due Modelli di Distribuzione

### **Modello 1: Tool Standalone Configurabile**
TuxBox è distribuito come strumento vuoto che ogni utente configura autonomamente.

#### Caratteristiche
- Binary `tbox` distribuito standalone
- Utente esegue `tbox init <registry-url>` per configurare
- Registry può essere pubblico o privato (SSH)
- Utente sceglie quali tool installare

#### Pro
- ✅ Massima flessibilità per l'utente
- ✅ Utente controlla quali tool installare
- ✅ Supporta registry multipli (pubblici + privati)
- ✅ Aggiornamenti tool indipendenti da aggiornamenti tbox
- ✅ Utente può creare registry custom

#### Contro
- ❌ Richiede configurazione iniziale
- ❌ Utente deve conoscere URL registry
- ❌ Barrier to entry più alta

#### Use Cases
- Tool interno aziendale (registry privato SSH)
- Community tool con registry pubblico
- Developer che vogliono massimo controllo

---

### **Modello 2: Tool Bundled con Registry Pre-configurato**
TuxBox è distribuito con registry pre-configurato e tool "raccomandati".

#### Caratteristiche
- Binary `tbox` + config.toml embedded
- Registry URL già configurato (es: github.com:disoardi/tuxbox-registry-private)
- `tbox list` mostra tool disponibili immediatamente
- Tool installati on-demand al primo run

#### Pro
- ✅ Zero-config per utente finale
- ✅ Esperienza "plug-and-play"
- ✅ Barrier to entry minima
- ✅ Tool "curati" dall'owner

#### Contro
- ❌ Meno flessibile
- ❌ Registry URL hardcoded (può essere limitante)
- ❌ Utente deve fidarsi del registry pre-configurato

#### Use Cases
- Tool personale per uso interno
- Suite di utility per team specifico
- "Appliance" tool (es: devops toolkit)

---

## 🎯 Strategia Consigliata: **Ibrida con Multi-Registry**

### Implementazione
Combina i vantaggi di entrambi i modelli:

1. **Default Registry (Opzionale)**
   - TuxBox può essere distribuito con un registry di default
   - Se `~/.tuxbox/config.toml` non esiste, usa default (se embedded)
   - Altrimenti, richiede `tbox init <url>`

2. **Multi-Registry Support**
   - Config supporta array di registry:
     ```toml
     [[registries]]
     name = "personal"
     url = "git@github.com:disoardi/tuxbox-registry-private.git"
     priority = 1  # Checked first

     [[registries]]
     name = "public"
     url = "https://github.com/tuxbox/registry-public.git"
     priority = 2  # Fallback
     ```

3. **Tool Resolution**
   - `tbox run <tool>` cerca nei registry in ordine di priority
   - Se tool presente in più registry, usa priority più alta
   - Permette override: `tbox run <tool> --registry public`

### Benefits
- ✅ Distribuzione flessibile (con o senza default registry)
- ✅ Supporta registry privati SSH + pubblici HTTPS
- ✅ Utente può aggiungere registry custom
- ✅ Tool possono provenire da fonti diverse

---

## 🔐 Registry Security Models

### Registry Privato (SSH)
**Setup richiesto:**
```bash
# Utente deve avere SSH key configurato per GitHub
# ~/.ssh/config:
Host github.com
    IdentityFile ~/.ssh/id_ed25519_github
    User git
```

**Comando init:**
```bash
tbox init git@github.com:disoardi/tuxbox-registry-private.git
```

**Pro:**
- Autenticazione SSH robusta
- Nessun token da gestire
- Permessi Git repository standard

**Contro:**
- Richiede setup SSH key
- Non funziona su ambienti senza SSH

---

### Registry Pubblico (HTTPS)
**Setup richiesto:** Nessuno

**Comando init:**
```bash
tbox init https://github.com/tuxbox/registry-public.git
```

**Pro:**
- Zero configuration
- Funziona ovunque
- Accessibile senza autenticazione

**Contro:**
- Chiunque può leggere
- Non adatto per tool proprietari

---

## 🚀 Roadmap di Distribuzione

### **Phase 2a: Multi-Registry Core**
- Implementa config.toml con array di registry
- Supporta sia SSH che HTTPS
- Tool resolution con priority
- Comandi: `tbox registry list`, `tbox registry add`, `tbox registry remove`

### **Phase 2b: Registry Privato SSH**
- Crea `tuxbox-registry-private` su GitHub
- Popola con tool personali (sshmenuc, etc.)
- Testa init + clone con SSH

### **Phase 3: Registry Pubblico (Optional)**
- Crea `tuxbox-registry-public` su GitHub
- Community tools
- Contribuzioni esterne

### **Phase 4: Default Registry Embedded (Optional)**
- Embed config.toml in binary con macro
- Distribuzione "pre-configured"
- Utente può sempre override con `tbox init`

---

## 📊 Decision Matrix

| Scenario | Registry Type | Distribution Model | Multi-Registry |
|----------|---------------|-------------------|----------------|
| **Uso personale** | SSH privato | Standalone + init | Sì (1 privato) |
| **Team interno** | SSH privato | Bundled con default | Sì (1 privato + 1 pubblico) |
| **Open source** | HTTPS pubblico | Standalone + init | Sì (multipli pubblici) |
| **Enterprise** | SSH privato | Bundled con default | Sì (multipli privati) |

---

## ✅ Decisione per MVP (Phase 2)

**Implementazione iniziale:**
1. ✅ Multi-registry support (core feature)
2. ✅ SSH authentication (user's SSH config)
3. ✅ HTTPS fallback support
4. ✅ Registry privato iniziale (`tuxbox-registry-private`)
5. 📅 Registry pubblico future (Phase 3)
6. 📅 Embedded default registry optional (Phase 4)

**Vantaggi:**
- Massima flessibilità
- Supporta sia uso personale che team
- Facile aggiungere registry pubblico in futuro
- SSH security per tool privati

---

**Last updated:** 2026-02-13
**Status:** Design approved, ready for implementation
