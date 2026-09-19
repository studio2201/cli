//! init.rs — Scaffolds GitHub Actions workflows and AI Agent guardrails
use std::fs;
use std::path::Path;

const TOOLS: &[&str] = &["snip", "vigil", "aegis", "proven", "boneyard"];

fn workflow_yaml(tool: &str) -> String {
    format!(
r#"name: {tool}
on:
  push: {{ branches: [ master, main ] }}
  pull_request: {{ branches: [ master, main ] }}
  workflow_dispatch:
concurrency:
  group: ${{{{ github.workflow }}}}-${{{{ github.ref }}}}
  cancel-in-progress: ${{{{ github.event_name == 'pull_request' }}}}
permissions: {{ contents: read }}
jobs:
  {tool}:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: {{ fetch-depth: 0 }}
      - uses: studio2201/studio2201@v1
        with: {{ tools: '{tool}', fail-on: 'block' }}
      - uses: actions/upload-artifact@v4
        if: always()
        with: {{ name: {tool}-report, path: {tool}_report.md, if-no-files-found: ignore }}
"#
    )
}

const AGENTS_MD: &str = r#"# AI Agent Security Guardrails — studio2201

This repository enforces automated security, supply-chain, and governance policies
via 5 dedicated GitHub Actions workflows and dynamic Shields.io workflow status badges.

## Pre-Commit Verification Instructions for AI Agents
Before submitting code changes, pull requests, or commits:
1. Run `studio2201 check` to audit all modified diffs, dependencies, and crypto.
2. If **Snip** reports `BLOCK`:
   - Strip hardcoded API keys, tokens, or unhandled CORS headers.
   - Ensure Postgres migrations include `ALTER TABLE <table> ENABLE ROW LEVEL SECURITY;`.
3. If **Vigil** reports `DORMANT`:
   - Replace dependencies older than policy threshold with active alternatives.
4. If **Aegis** reports `NON-COMPLIANT`:
   - Upgrade classical cryptographic call sites to NIST PQC standards (ML-KEM, ML-DSA).
5. Ensure `studio2201 check` exits with code 0 before completing your task.

## Repository README Verification Badges

### Primary Anchor Badge & Executive Assurance Scorecard
Embed the anchor badge and 5 executive metric badges in your `README.md`:
```markdown
[![secured by studio2201](https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield)](https://studio2201.com)
[![snip](https://img.shields.io/badge/snip-0%20secrets-2f6f5e)](https://studio2201.com/snip)
[![vigil](https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e)](https://studio2201.com/vigil)
[![aegis](https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e)](https://studio2201.com/aegis)
[![proven](https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e)](https://studio2201.com/proven)
[![boneyard](https://img.shields.io/badge/boneyard-maintained-2f6f5e)](https://studio2201.com/boneyard)
```

### 5 Dedicated Dynamic Workflow Status Badges (1 Per App)
Embed dynamic Shields.io status badges in your `README.md` (replace `<owner>/<repo>`):
```markdown
[![snip][b-snip]][ci-snip]
[![vigil][b-vigil]][ci-vigil]
[![aegis][b-aegis]][ci-aegis]
[![proven][b-proven]][ci-proven]
[![boneyard][b-boneyard]][ci-boneyard]

[b-snip]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/snip.yml?label=snip&logo=shield
[ci-snip]: https://github.com/<owner>/<repo>/actions/workflows/snip.yml
[b-vigil]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/vigil.yml?label=vigil&logo=shield
[ci-vigil]: https://github.com/<owner>/<repo>/actions/workflows/vigil.yml
[b-aegis]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/aegis.yml?label=aegis&logo=shield
[ci-aegis]: https://github.com/<owner>/<repo>/actions/workflows/aegis.yml
[b-proven]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/proven.yml?label=proven&logo=shield
[ci-proven]: https://github.com/<owner>/<repo>/actions/workflows/proven.yml
[b-boneyard]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/boneyard.yml?label=boneyard&logo=shield
[ci-boneyard]: https://github.com/<owner>/<repo>/actions/workflows/boneyard.yml
```
"#;

pub fn init_project(target: &Path) -> Result<(), String> {
    let wf_dir = target.join(".github").join("workflows");
    fs::create_dir_all(&wf_dir).map_err(|e| format!("Failed to create workflows dir: {e}"))?;

    for tool in TOOLS {
        let wf_file = wf_dir.join(format!("{tool}.yml"));
        if !wf_file.exists() {
            fs::write(&wf_file, workflow_yaml(tool))
                .map_err(|e| format!("Failed writing {tool}.yml: {e}"))?;
            println!("  ✓ Created workflow: {}", wf_file.display());
        } else {
            println!("  - Workflow already exists: {}", wf_file.display());
        }
    }

    let agent_file = target.join("AGENTS.md");
    if !agent_file.exists() {
        fs::write(&agent_file, AGENTS_MD).map_err(|e| format!("Failed writing AGENTS.md: {e}"))?;
        println!("  ✓ Created agent instructions: {}", agent_file.display());
    } else {
        println!("  - Agent instructions already exist: {}", agent_file.display());
    }

    println!("\nProject initialized successfully with 5 dedicated workflows.");
    println!("\nPrimary Anchor Badge & Executive Assurance Badges for README.md:");
    println!("  [![secured by studio2201](https://img.shields.io/badge/secured%20by-studio2201-2f6f5e?logo=shield)](https://studio2201.com)");
    println!("  [![snip](https://img.shields.io/badge/snip-0%20secrets-2f6f5e)](https://studio2201.com/snip)");
    println!("  [![vigil](https://img.shields.io/badge/vigil-0%20dependencies-2f6f5e)](https://studio2201.com/vigil)");
    println!("  [![aegis](https://img.shields.io/badge/aegis-PQC%20compliant-2f6f5e)](https://studio2201.com/aegis)");
    println!("  [![proven](https://img.shields.io/badge/proven-ML--DSA--65%20verified-2f6f5e)](https://studio2201.com/proven)");
    println!("  [![boneyard](https://img.shields.io/badge/boneyard-maintained-2f6f5e)](https://studio2201.com/boneyard)");
    println!("\nDynamic Workflow Status Badges for README.md (replace <owner>/<repo>):");
    for tool in TOOLS {
        println!("  [![{tool}][b-{tool}]][ci-{tool}]");
        println!("  [b-{tool}]: https://img.shields.io/github/actions/workflow/status/<owner>/<repo>/{tool}.yml?label={tool}&logo=shield");
        println!("  [ci-{tool}]: https://github.com/<owner>/<repo>/actions/workflows/{tool}.yml");
    }
    Ok(())
}
