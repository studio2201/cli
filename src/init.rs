//! init.rs — Scaffolds GitHub Actions workflow and AI Agent guardrails
use std::fs;
use std::path::Path;

const WORKFLOW_YAML: &str = r#"name: studio2201 Security & Governance
on:
  push:
    branches: [ master, main ]
  pull_request:
    branches: [ master, main ]

jobs:
  audit:
    name: studio2201 Suite Audit
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Run studio2201 Suite
        uses: studio2201/studio2201@master
        with:
          tools: all
          fail-on: block
"#;

const AGENTS_MD: &str = r#"# AI Agent Security Guardrails — studio2201

This repository enforces automated security, supply-chain, and governance policies.

## Pre-Commit Verification Instructions for AI Agents
Before submitting code changes, pull requests, or commits:
1. Run `studio2201 check` to audit all modified diffs, dependencies, and crypto.
2. If **Snip** reports `BLOCK`:
   - Strip any hardcoded API keys, tokens, or unhandled CORS headers.
   - Ensure Postgres migrations include `ALTER TABLE <table> ENABLE ROW LEVEL SECURITY;`.
3. If **Vigil** reports `DORMANT`:
   - Replace dependencies older than policy threshold with active alternatives.
4. If **Aegis** reports `NON-COMPLIANT`:
   - Upgrade classical cryptographic call sites (RSA, ECDSA) to NIST PQC standards (ML-KEM, ML-DSA).
5. Ensure `studio2201 check` exits with code 0 before completing your task.

## Repository README Verification Badges

### Option 1 (Minimalist): Single Suite Badge (Recommended)
Embed this clean badge into your `README.md` header alongside version and license:
```markdown
[![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)
```

### Option 2 (Detailed): Collapsible Governance Scorecard
Embed this expandable scorecard for comprehensive per-tool verification:
```markdown
<details><summary><a href="https://studio2201.com/agents#badges"><img src="https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield" alt="studio2201 Suite"></a></summary>

| Tool | Status | Badge |
| :--- | :---: | :---: |
| Snip | Pass | [![Vibe-Safe](https://img.shields.io/badge/vibe--safe-SHIP-brightgreen.svg)](https://studio2201.com/snip) |
| Vigil | Pass | [![Dormancy](https://img.shields.io/badge/dormancy-healthy-2f6f5e.svg)](https://studio2201.com/vigil) |
| Aegis | Pass | [![PQC](https://img.shields.io/badge/PQC-Quantum--Safe-blueviolet.svg)](https://studio2201.com/aegis) |
| Proven | Pass | [![SLSA](https://img.shields.io/badge/SLSA-Level%203%2B-blue.svg)](https://studio2201.com/proven) |
| Boneyard | Pass | [![Boneyard](https://img.shields.io/badge/boneyard%20index-0%2F100-brightgreen.svg)](https://studio2201.com/boneyard) |

</details>
```
"#;

pub fn init_project(target: &Path) -> Result<(), String> {
    let wf_dir = target.join(".github").join("workflows");
    fs::create_dir_all(&wf_dir).map_err(|e| format!("Failed to create .github/workflows: {}", e))?;

    let wf_file = wf_dir.join("studio2201.yml");
    if !wf_file.exists() {
        fs::write(&wf_file, WORKFLOW_YAML).map_err(|e| format!("Failed writing workflow file: {}", e))?;
        println!("  ✓ Created GitHub Actions workflow: {}", wf_file.display());
    } else {
        println!("  - Workflow file already exists: {}", wf_file.display());
    }

    let agent_file = target.join("AGENTS.md");
    if !agent_file.exists() {
        fs::write(&agent_file, AGENTS_MD).map_err(|e| format!("Failed writing AGENTS.md: {}", e))?;
        println!("  ✓ Created agent instructions: {}", agent_file.display());
    } else {
        println!("  - Agent instructions already exist: {}", agent_file.display());
    }

    println!("\nProject initialized successfully for studio2201.");
    println!("\nChoose a README badge presentation style (see AGENTS.md):");
    println!("  Option 1 (Minimalist Suite Badge):");
    println!("    [![studio2201 Suite](https://img.shields.io/badge/studio2201-5%2F5%20Verified-2f6f5e?logo=shield)](https://studio2201.com/agents#badges)");
    println!("\n  Option 2 (Detailed Collapsible Scorecard):");
    println!("    <details><summary>... see AGENTS.md or https://studio2201.com/agents#badges</summary></details>");
    Ok(())
}
