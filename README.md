# studio2201 CLI

[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)
[![Version](https://img.shields.io/badge/version-v0.1.7-blue.svg)](https://github.com/studio2201/cli/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

<details>
<summary><a href="https://studio2201.com/agents#badges"><img src="https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield" alt="studio2201 Suite"></a> <b>Detailed Governance Scorecard</b></summary>

| Tool | Focus | Verdict | Status Badge |
| :--- | :--- | :---: | :---: |
| [**Snip**](https://studio2201.com/snip) | Vibe-Code & Secrets Gate | `SHIP` | [![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip) |
| [**Vigil**](https://studio2201.com/vigil) | Supply-Chain Dormancy | `HEALTHY` | [![Dormancy](https://img.shields.io/badge/dormancy-healthy-2f6f5e.svg)](https://studio2201.com/vigil) |
| [**Aegis**](https://studio2201.com/aegis) | PQC & Post-Quantum Scans | `QUANTUM-SAFE` | [![PQC](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis) |
| [**Proven**](https://studio2201.com/proven) | ML-DSA-65 Attestation | `VERIFIED` | [![SLSA](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven) |
| [**Boneyard**](https://studio2201.com/boneyard) | Tech-Debt Radar | `0/100 DEBT` | [![Boneyard](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard) |

</details>

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
