# studio2201 CLI

[![Version](https://img.shields.io/badge/version-v0.1.0-blue.svg)](https://github.com/studio2201/cli/releases)
[![Pure std::](https://img.shields.io/badge/pure-std%3A%3A-success.svg)](https://studio2201.com)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Max LOC](https://img.shields.io/badge/max%20LOC-%E2%89%A4256-brightgreen.svg)](https://studio2201.com)

The unified command-line interface and toolchain manager for the **studio2201** security and governance ecosystem.

---

## Why This CLI Is Needed (For Humans)

Modern software teams face multiple overlapping integrity threats: leaked API keys in AI diffs, abandoned open-source dependencies in the supply chain, quantum-vulnerable cryptography, and compounding technical debt.

The **`studio2201` CLI** acts as a unified toolchain driver and local pre-commit mirror of the GitHub Actions suite:
1. **Zero-Root Toolchain Management:** Install, upgrade, or remove any or all of the five studio2201 security tools (`snip`, `vigil`, `aegis`, `proven`, `boneyard`) directly into `~/.local/bin` without `sudo` or package manager conflicts.
2. **Unified Pre-Commit Verification:** Run `studio2201 check` to audit all modified diffs, manifests, and crypto primitives locally before pushing to GitHub.
3. **Instant Project Bootstrapping:** Run `studio2201 init` to scaffold `.github/workflows/studio2201.yml` and agent instructions into any repository.

---

## Prompt for your AI Agent (Cursor / Claude / Devin)

Copy and paste this prompt to instruct your coding agent to set up and enforce studio2201:

```markdown
Install the studio2201 toolchain and initialize security enforcement:
1. Install the studio2201 CLI:
   curl -fsSL https://studio2201.com/install.sh | sh -s all
2. Initialize the repository with GitHub Actions and agent guardrails:
   studio2201 init
3. Verify the working tree:
   studio2201 check
Ensure all checks pass with exit code 0 before completing your task.
```

---

## Commands

### Toolchain Management
```bash
# Install tools into ~/.local/bin
studio2201 install all
studio2201 install snip

# Check status of installed tools
studio2201 list

# Upgrade installed tools
studio2201 upgrade all

# Remove tools
studio2201 remove vigil
```

### Audit & Governance
```bash
# Run all installed tools against current repository
studio2201 check

# Audit a specific directory or target
studio2201 check --path /path/to/project

# Scaffold GitHub Action and agent guardrails
studio2201 init
```

### Subcommand Forwarding
```bash
studio2201 snip audit
studio2201 vigil scan
studio2201 aegis scan
studio2201 proven doctor
studio2201 boneyard scan
```

---

## Technical Specifications
- **Pure `std::` Rust:** Zero external `crates.io` dependencies.
- **Strict Line Bounds:** Every source file strictly $\le 256$ LOC.
- **XDG Base Directory Compliance:** Installs to `${XDG_BIN_HOME:-$HOME/.local/bin}`.
