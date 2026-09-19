//! toolchain.rs — Toolchain management (install, remove, upgrade, list)
use crate::xdg;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const ALL_TOOLS: &[&str] = &["snip", "vigil", "aegis", "proven", "boneyard"];

pub fn resolve_dest(dest: Option<&Path>) -> PathBuf {
    dest.map(PathBuf::from).unwrap_or_else(xdg::bin_dir)
}

pub fn check_installed(tool: &str, dest: &Path) -> Option<String> {
    let p = dest.join(tool);
    if !p.is_file() {
        return None;
    }
    if let Ok(out) = Command::new(&p).arg("--version").output() {
        if out.status.success() {
            let ver = String::from_utf8_lossy(&out.stdout)
                .trim()
                .split_whitespace()
                .last()
                .unwrap_or("installed")
                .to_string();
            return Some(ver);
        }
    }
    Some("installed".to_string())
}

pub fn query_remote_version(tool: &str) -> String {
    let url = format!("https://github.com/studio2201/{}/releases/latest", tool);
    if let Ok(out) = Command::new("curl").args(["-sI", "--max-time", "3", &url]).output() {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.to_lowercase().starts_with("location:") {
                if let Some(pos) = line.rfind('/') {
                    let tag = line[pos + 1..].trim();
                    let stripped = tag.strip_prefix('v').unwrap_or(tag);
                    if !stripped.is_empty() {
                        return stripped.to_string();
                    }
                }
            }
        }
    }
    "v0.2.6".to_string()
}

pub fn install_tool(tool: &str, dest: &Path) -> Result<(), String> {
    if tool == "all" {
        for t in ALL_TOOLS {
            install_tool(t, dest)?;
        }
        return Ok(());
    }
    if !ALL_TOOLS.contains(&tool) {
        return Err(format!("Unknown tool: '{}'. Valid tools: {}", tool, ALL_TOOLS.join(", ")));
    }
    fs::create_dir_all(dest).map_err(|e| format!("Failed creating bin dir: {}", e))?;
    let mut cmd = Command::new("sh");
    let script = format!(
        "curl -fsSL https://studio2201.com/install.sh | INSTALL_DIR=\"{}\" sh -s {}",
        dest.display(),
        tool
    );
    cmd.args(["-c", &script]);
    let status = cmd.status().map_err(|e| format!("Install failed: {}", e))?;
    if status.success() {
        println!("  ✓ Successfully installed {} to {}", tool, dest.join(tool).display());
        Ok(())
    } else {
        Err(format!("Installation of {} failed with exit code {:?}", tool, status.code()))
    }
}

pub fn remove_tool(tool: &str, dest: &Path) -> Result<(), String> {
    if tool == "all" {
        for t in ALL_TOOLS {
            remove_tool(t, dest)?;
        }
        return Ok(());
    }
    if !ALL_TOOLS.contains(&tool) {
        return Err(format!("Unknown tool: '{}'. Valid tools: {}", tool, ALL_TOOLS.join(", ")));
    }
    let p = dest.join(tool);
    if p.exists() {
        fs::remove_file(&p).map_err(|e| format!("Failed to remove {}: {}", p.display(), e))?;
        println!("  ✓ Removed {}", p.display());
    } else {
        println!("  - Tool {} is not installed in {}", tool, dest.display());
    }
    Ok(())
}

pub fn upgrade_tool(tool: &str, dest: &Path) -> Result<(), String> {
    if tool == "all" {
        for t in ALL_TOOLS {
            upgrade_tool(t, dest)?;
        }
        return Ok(());
    }
    if !ALL_TOOLS.contains(&tool) {
        return Err(format!("Unknown tool: '{}'. Valid tools: {}", tool, ALL_TOOLS.join(", ")));
    }
    println!("Checking latest release for {}...", tool);
    install_tool(tool, dest)
}

pub fn print_status(dest: &Path) -> Result<(), String> {
    let border = "+----------+----------------------+--------------------+--------------------+";
    println!("{}", border);
    println!("| Tool     | Installed Version    | Status             | Binary Path        |");
    println!("{}", border);

    for tool in ALL_TOOLS {
        let (ver, status_str, path_str) = match check_installed(tool, dest) {
            Some(v) => (v, "INSTALLED", dest.join(tool).display().to_string()),
            None => ("-".to_string(), "NOT INSTALLED", "-".to_string()),
        };
        let p_display = if path_str.len() > 18 {
            format!("...{}", &path_str[path_str.len() - 15..])
        } else {
            path_str
        };
        println!("| {:<8} | {:<20} | {:<18} | {:<18} |", tool, ver, status_str, p_display);
    }
    println!("{}", border);
    println!("Install directory: {}", dest.display());
    Ok(())
}
