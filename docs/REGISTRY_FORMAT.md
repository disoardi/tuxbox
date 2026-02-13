# TuxBox Registry Format

## Overview

TuxBox registries are Git repositories containing a `tools.toml` file that defines available tools.

---

## tools.toml Structure

```toml
# Registry metadata (optional)
[metadata]
name = "TuxBox Personal Registry"
description = "Personal tools collection"
version = "1.0"

# Tool definitions
[tools.sshmenuc]
name = "sshmenuc"
repo = "https://github.com/disoardi/sshmenuc"
branch = "main"
version = "1.1.0"
type = "python"
description = "SSH connection manager with interactive menu"

[tools.sshmenuc.commands]
run = "python3 -m sshmenuc"

[tools.sshmenuc.dependencies]
python = ">=3.8"
requirements = "requirements.txt"

[tools.another-tool]
name = "another-tool"
repo = "git@github.com:user/private-tool.git"
type = "python"
description = "Another tool example"

[tools.another-tool.commands]
run = "python -m another_tool"
setup = "pip install -e ."
```

---

## Field Reference

### Tool Section `[tools.<tool-name>]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ Yes | Tool identifier (must match section name) |
| `repo` | String | ✅ Yes | Git repository URL (SSH or HTTPS) |
| `branch` | String | ❌ No | Git branch to clone (default: main) |
| `version` | String | ❌ No | Tool version for tracking |
| `type` | String | ❌ No | Tool type: `python`, `bash`, `node`, etc. |
| `description` | String | ❌ No | Human-readable description |

### Commands Section `[tools.<tool-name>.commands]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `run` | String | ✅ Yes | Command to execute the tool |
| `setup` | String | ❌ No | Setup command (runs once after clone) |

### Dependencies Section `[tools.<tool-name>.dependencies]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `python` | String | ❌ No | Python version requirement (e.g., ">=3.8") |
| `requirements` | String | ❌ No | Requirements file path (default: requirements.txt) |

---

## Example Registry Repository Structure

```
tuxbox-registry-private/
├── README.md
├── tools.toml          # Main registry file
└── .git/
```

---

## Creating a Registry

### Step 1: Create Git Repository

```bash
# Create new repo locally
mkdir tuxbox-registry-private
cd tuxbox-registry-private
git init

# Or clone existing
git clone git@github.com:user/tuxbox-registry-private.git
cd tuxbox-registry-private
```

### Step 2: Create tools.toml

```bash
cat > tools.toml << 'EOF'
[tools.sshmenuc]
name = "sshmenuc"
repo = "https://github.com/disoardi/sshmenuc"
branch = "main"
version = "1.1.0"
type = "python"
description = "SSH connection manager"

[tools.sshmenuc.commands]
run = "python3 -m sshmenuc"
EOF
```

### Step 3: Commit and Push

```bash
git add tools.toml
git commit -m "Initial registry with sshmenuc"
git push origin main
```

### Step 4: Configure in TuxBox

```bash
# SSH authentication (recommended for private repos)
tbox init git@github.com:user/tuxbox-registry-private.git

# Or HTTPS (for public repos)
tbox init https://github.com/user/tuxbox-registry-public.git
```

---

## Multi-Registry Setup

TuxBox supports multiple registries with priority-based resolution.

### Example: Private + Public Registries

```bash
# Add primary private registry (priority 200 - checked first)
tbox registry add personal git@github.com:disoardi/tuxbox-registry-private.git --priority 200

# Add public community registry (priority 100 - fallback)
tbox registry add community https://github.com/tuxbox/registry-public.git --priority 100

# List configured registries
tbox registry list

# Sync all registries
tbox registry sync
```

### Tool Resolution Order

When running `tbox run <tool>`, TuxBox searches registries in priority order (highest first):

1. **personal** (priority 200) - checked first
2. **community** (priority 100) - checked if not found in personal

If tool found in multiple registries, highest priority wins.

---

## Registry Authentication

### SSH (Recommended for Private Repos)

TuxBox uses your existing SSH configuration:

```bash
# ~/.ssh/config
Host github.com
    IdentityFile ~/.ssh/id_ed25519_github
    User git
```

```bash
# Add SSH registry
tbox init git@github.com:user/private-registry.git
```

### HTTPS (For Public Repos)

No authentication needed:

```bash
# Add HTTPS registry
tbox init https://github.com/user/public-registry.git
```

---

## Best Practices

### 1. Tool Naming
- Use lowercase with hyphens: `ssh-tool`, `my-utility`
- Keep names short and descriptive
- Match tool section name: `[tools.ssh-tool]` → `name = "ssh-tool"`

### 2. Versioning
- Always specify `version` field for tracking
- Use semantic versioning: `1.2.3`
- Update version when tool changes

### 3. Descriptions
- Provide clear, concise descriptions
- Help users understand what tool does
- Include use cases or key features

### 4. Commands
- Prefer `python3 -m module` over direct script paths
- Use relative paths: `./script.sh` not `/full/path/script.sh`
- Document required arguments in description

### 5. Repository Organization
- One tool per repository (recommended)
- Keep tool repo separate from registry repo
- Use meaningful branch names

---

## Troubleshooting

### Error: "Tool not found in registry"

**Cause:** Tool not defined in any configured registry's tools.toml

**Solution:**
```bash
# Verify registries are configured
tbox registry list

# Sync registries to fetch latest
tbox registry sync

# Check tools in registry
tbox list
```

### Error: "Failed to clone registry"

**Cause:** SSH authentication failed or URL incorrect

**Solution:**
```bash
# Test SSH connection
ssh -T git@github.com

# Verify SSH config
cat ~/.ssh/config

# Check registry URL
tbox registry list

# Re-add registry with correct URL
tbox registry remove personal
tbox registry add personal git@github.com:user/repo.git
```

### Error: "Failed to parse tools.toml"

**Cause:** Invalid TOML syntax

**Solution:**
```bash
# Validate tools.toml manually
cd ~/.tuxbox/registry/<registry-name>
cat tools.toml

# Check for syntax errors:
# - Missing quotes
# - Incorrect section names
# - Invalid characters
```

---

## Advanced: Registry Structure Options

### Option 1: Monolithic (Recommended)

All tools in single `tools.toml`:

```
tuxbox-registry/
├── tools.toml          # All tools defined here
└── README.md
```

**Pros:**
- Simple structure
- Easy to maintain
- Fast to parse

**Cons:**
- Large file for many tools

### Option 2: Split (Future)

Separate files per tool:

```
tuxbox-registry/
├── tools.toml          # Tool list
├── tools/
│   ├── sshmenuc.toml
│   └── other-tool.toml
└── README.md
```

**Note:** Not yet implemented in TuxBox

---

## Example: Complete Registry

See [example-registry/](example-registry/) for a complete working example.

```bash
# Clone example registry
git clone https://github.com/tuxbox/example-registry.git

# Explore structure
cd example-registry
cat tools.toml
cat README.md
```

---

**Last updated:** 2026-02-13
**TuxBox Version:** 0.1.0 (Phase 2)
