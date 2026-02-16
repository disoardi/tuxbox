# Git Hooks for TuxBox

This directory contains Git hooks to improve development workflow and prevent CI failures.

## Pre-commit Hook

The pre-commit hook automatically formats Rust code using `cargo fmt` before each commit.

### Installation

```bash
# From the repository root
cp .githooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

Or use a one-liner:

```bash
cp .githooks/pre-commit .git/hooks/ && chmod +x .git/hooks/pre-commit
```

### What it does

1. Runs `cargo fmt --all` before commit
2. Automatically stages formatted files
3. Prevents CI formatting failures

### Disable temporarily

If you need to commit without formatting (not recommended):

```bash
git commit --no-verify -m "your message"
```

## Future Hooks

Additional hooks can be added here:
- `pre-push`: Run tests before push
- `commit-msg`: Enforce commit message format

## Benefits

✅ **Automatic formatting** - No manual `cargo fmt` needed
✅ **Consistent style** - All commits follow Rust style guidelines
✅ **CI success** - Formatting checks always pass
✅ **Developer friendly** - Works silently in the background
