//! runner.rs — Orchestrates audit runs across installed studio2201 tools
use crate::toolchain::ALL_TOOLS;
use crate::xdg;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct ToolResult {
    pub tool: String,
    pub target: String,
    pub verdict: String,
    pub exit_code: i32,
    pub passed: bool,
    pub output: String,
}

fn run_snip(target: &Path) -> ToolResult {
    let mut diff = String::new();
    if let Ok(out) = Command::new("git").args(["diff", "--staged"]).output() {
        if out.status.success() && !out.stdout.is_empty() {
            diff = String::from_utf8_lossy(&out.stdout).to_string();
        }
    }
    if diff.is_empty() {
        if let Ok(out) = Command::new("git").args(["diff", "HEAD~1"]).output() {
            if out.status.success() && !out.stdout.is_empty() {
                diff = String::from_utf8_lossy(&out.stdout).to_string();
            }
        }
    }

    let mut cmd = Command::new(xdg::tool_path("snip"));
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.args(["--format", "text"]);

    let (ec, out_str) = match cmd.spawn() {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(diff.as_bytes());
            }
            match child.wait_with_output() {
                Ok(out) => (out.status.code().unwrap_or(1), String::from_utf8_lossy(&out.stdout).to_string()),
                Err(e) => (1, format!("Execution failed: {}", e)),
            }
        }
        Err(e) => (1, format!("Failed to spawn snip: {}", e)),
    };

    let passed = ec == 0;
    let verdict = if passed { "SHIP".to_string() } else { "BLOCK".to_string() };
    ToolResult {
        tool: "snip".into(),
        target: target.display().to_string(),
        verdict,
        exit_code: ec,
        passed,
        output: out_str,
    }
}

fn run_vigil(target: &Path) -> ToolResult {
    let manifests = ["Cargo.lock", "package.json", "requirements.txt", "go.mod", "Cargo.toml"];
    let mut found_manifest = None;
    for m in manifests {
        let p = target.join(m);
        if p.exists() {
            found_manifest = Some(p);
            break;
        }
    }

    let manifest_path = match found_manifest {
        Some(p) => p,
        None => {
            return ToolResult {
                tool: "vigil".into(),
                target: target.display().to_string(),
                verdict: "SKIPPED".into(),
                exit_code: 0,
                passed: true,
                output: "No supported manifest found".into(),
            }
        }
    };

    let mut cmd = Command::new(xdg::tool_path("vigil"));
    cmd.args(["scan", manifest_path.to_str().unwrap_or(".")]);
    let (ec, out_str) = match cmd.output() {
        Ok(out) => (out.status.code().unwrap_or(1), String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e) => (1, format!("Failed to run vigil: {}", e)),
    };

    let passed = ec == 0;
    let verdict = if passed { "COMPLIANT".to_string() } else { "DORMANT".to_string() };
    ToolResult {
        tool: "vigil".into(),
        target: manifest_path.display().to_string(),
        verdict,
        exit_code: ec,
        passed,
        output: out_str,
    }
}

fn run_aegis(target: &Path) -> ToolResult {
    let mut cmd = Command::new(xdg::tool_path("aegis"));
    cmd.args(["scan", target.to_str().unwrap_or(".")]);
    let (ec, out_str) = match cmd.output() {
        Ok(out) => (out.status.code().unwrap_or(1), String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e) => (1, format!("Failed to run aegis: {}", e)),
    };

    let passed = ec == 0;
    let verdict = if passed { "QUANTUM-SAFE".to_string() } else { "NON-COMPLIANT".to_string() };
    ToolResult {
        tool: "aegis".into(),
        target: target.display().to_string(),
        verdict,
        exit_code: ec,
        passed,
        output: out_str,
    }
}

fn run_proven(target: &Path) -> ToolResult {
    let mut cmd = Command::new(xdg::tool_path("proven"));
    cmd.args(["doctor"]);
    let (ec, out_str) = match cmd.output() {
        Ok(out) => (out.status.code().unwrap_or(1), String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e) => (1, format!("Failed to run proven: {}", e)),
    };

    let passed = ec == 0;
    let verdict = if passed { "VERIFIED".to_string() } else { "TAMPERED".to_string() };
    ToolResult {
        tool: "proven".into(),
        target: target.display().to_string(),
        verdict,
        exit_code: ec,
        passed,
        output: out_str,
    }
}

fn run_boneyard(target: &Path) -> ToolResult {
    let mut cmd = Command::new(xdg::tool_path("boneyard"));
    cmd.args(["scan", target.to_str().unwrap_or(".")]);
    let (ec, out_str) = match cmd.output() {
        Ok(out) => (out.status.code().unwrap_or(1), String::from_utf8_lossy(&out.stdout).to_string()),
        Err(e) => (1, format!("Failed to run boneyard: {}", e)),
    };

    let passed = ec == 0;
    let verdict = if passed { "HEALTHY".to_string() } else { "DEBT BREACH".to_string() };
    ToolResult {
        tool: "boneyard".into(),
        target: target.display().to_string(),
        verdict,
        exit_code: ec,
        passed,
        output: out_str,
    }
}

pub fn run_suite(tools: &[String], target: &Path) -> Result<i32, String> {
    let active_tools: Vec<&str> = if tools.is_empty() || tools.contains(&"all".to_string()) {
        ALL_TOOLS.to_vec()
    } else {
        tools.iter().map(|s| s.as_str()).collect()
    };

    println!("\nstudio2201 Suite Check: {}", target.display());
    let mut results = Vec::new();
    let mut overall_success = true;

    for t in active_tools {
        let bin = xdg::tool_path(t);
        if !bin.exists() {
            println!("  ! Warning: Tool '{}' not installed at {}. Skipping.", t, bin.display());
            continue;
        }
        let res = match t {
            "snip" => run_snip(target),
            "vigil" => run_vigil(target),
            "aegis" => run_aegis(target),
            "proven" => run_proven(target),
            "boneyard" => run_boneyard(target),
            _ => continue,
        };
        if !res.passed {
            overall_success = false;
        }
        results.push(res);
    }

    let border = "+----------+--------------------------+-----+-----+---------------+--------+";
    println!("\n{}", border);
    println!("| Tool     | Target                   | Exp | Act | Verdict       | Status |");
    println!("{}", border);

    for r in &results {
        let t_display = if r.target.len() > 24 {
            format!("...{}", &r.target[r.target.len() - 21..])
        } else {
            r.target.clone()
        };
        let status_str = if r.passed { " PASS " } else { " FAIL " };
        println!(
            "| {:<8} | {:<24} | {:>3} | {:>3} | {:<13} | {} |",
            r.tool, t_display, 0, r.exit_code, r.verdict, status_str
        );
    }
    println!("{}\n", border);

    if let Ok(summary_path) = env::var("GITHUB_STEP_SUMMARY") {
        if let Ok(mut f) = fs::OpenOptions::new().append(true).create(true).open(summary_path) {
            let _ = writeln!(f, "### studio2201 Suite Scorecard\n");
            let _ = writeln!(f, "| Tool | Target | Verdict | Exit Code |");
            let _ = writeln!(f, "| :--- | :--- | :--- | :--- |");
            for r in &results {
                let badge = if r.passed { "🟢 PASS" } else { "🔴 FAIL" };
                let _ = writeln!(f, "| **{}** | `{}` | {} `{}` | `{}` |", r.tool, r.target, badge, r.verdict, r.exit_code);
            }
        }
    }

    if overall_success {
        println!("✓ ALL CHECKS PASSED: Suite is compliant.");
        Ok(0)
    } else {
        println!("✗ CHECKS FAILED: Policy violations detected.");
        Ok(1)
    }
}
