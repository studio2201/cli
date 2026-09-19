# studio2201 CLI

[![secured by studio2201](https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield)](https://studio2201.com)
[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)
[![Version](https://img.shields.io/badge/version-v0.1.11-blue.svg)](https://github.com/studio2201/cli/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

<details>
<summary><b>Executive Assurance Scorecard</b> (EO 14028 · NIST SP 800-218 · OMB M-26-15)</summary>

| Tool | Mandate / Standard | Assurance Metric | Status |
| :--- | :--- | :--- | :---: |
| [**Snip**][u-snip] | EO 14028 §4 (Credential Defense) | `0 secrets` | [![snip][m-snip]][u-snip] |
| [**Vigil**][u-vigil] | NIST SP 800-218 (Supply Surface) | `0 dependencies` | [![vigil][m-vigil]][u-vigil] |
| [**Aegis**][u-aegis] | OMB M-26-15 (Post-Quantum Crypto) | `PQC compliant` | [![aegis][m-aegis]][u-aegis] |
| [**Proven**][u-proven] | NIST FIPS 204 (SLSA Level 3+) | `ML-DSA-65 verified` | [![proven][m-proven]][u-proven] |
| [**Boneyard**][u-boneyard] | Asset Ownership (Zombie Defense) | `maintained` | [![boneyard][m-boneyard]][u-boneyard] |

[u-snip]: https://studio2201.com/snip
[u-vigil]: https://studio2201.com/vigil
[u-aegis]: https://studio2201.com/aegis
[u-proven]: https://studio2201.com/proven
[u-boneyard]: https://studio2201.com/boneyard
[m-snip]: https://img.shields.io/badge/snip-0%20secrets-2f6f5e
[m-vigil]: https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e
[m-aegis]: https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e
[m-proven]: https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e
[m-boneyard]: https://img.shields.io/badge/boneyard-maintained-2f6f5e

</details>

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
