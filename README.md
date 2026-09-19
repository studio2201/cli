# studio2201 CLI

<div align="center">

[![Version](https://img.shields.io/badge/version-v0.1.12-blue.svg)](https://github.com/studio2201/cli/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

| Security Pillar | Verification Badge |
| :--- | :---: |
| **Platform Standard** | [![secured by studio2201][b-studio]][u-home] |
| **Credential Defense** | [![snip][b-snip]][u-snip] |
| **Supply Chain Surface** | [![vigil][b-vigil]][u-vigil] |
| **Post-Quantum Cryptography** | [![aegis][b-aegis]][u-aegis] |
| **Build Provenance & SLSA** | [![proven][b-proven]][u-proven] |
| **Repository Governance** | [![boneyard][b-boneyard]][u-boneyard] |

[b-studio]: https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield
[u-home]: https://studio2201.com
[b-snip]: https://img.shields.io/badge/snip-0%20secrets-2f6f5e?logo=shield
[u-snip]: https://studio2201.com/snip
[b-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e?logo=shield
[u-vigil]: https://studio2201.com/vigil
[b-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e?logo=shield
[u-aegis]: https://studio2201.com/aegis
[b-proven]: https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e?logo=shield
[u-proven]: https://studio2201.com/proven
[b-boneyard]: https://img.shields.io/badge/boneyard-maintained-2f6f5e?logo=shield
[u-boneyard]: https://studio2201.com/boneyard

</div>

The unified command-line interface and toolchain manager for the **studio2201** security and governance ecosystem.

---

## Why This CLI Is Needed (For Humans)

Modern software teams face multiple overlapping integrity threats: leaked API keys in AI diffs,
abandoned open-source dependencies in the supply chain, quantum-vulnerable cryptography, and compounding technical debt.

The **`studio2201` CLI** acts as a unified toolchain driver and local pre-commit mirror of the GitHub Actions suite:
1. **Zero-Root Toolchain Management:** Install, upgrade, or remove any or all of the five studio2201 security tools
   (`snip`, `vigil`, `aegis`, `proven`, `boneyard`) directly into `~/.local/bin` without `sudo` or package manager conflicts.
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
