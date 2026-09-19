//! cli.rs — Command-line interface definitions and parser for studio2201
use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

pub const VERSION: &str = "0.1.0";

pub enum Action {
    Install { tool: String, dest: Option<PathBuf> },
    Remove { tool: String, dest: Option<PathBuf> },
    Upgrade { tool: String, dest: Option<PathBuf> },
    Status { dest: Option<PathBuf> },
    Check { tools: Vec<String>, path: PathBuf },
    Init { path: PathBuf },
    Forward { tool: String, args: Vec<String> },
    Help,
    Version,
}

pub fn print_help() {
    println!(
        r#"studio2201 {} — Unified CLI and Toolchain Manager for studio2201

USAGE:
  studio2201 [COMMAND] [OPTIONS]

TOOLCHAIN COMMANDS:
  install [tool | all]     Install specified tool(s) (default: all)
  remove  [tool | all]     Remove specified tool(s) from ~/.local/bin
  upgrade [tool | all]     Upgrade tool(s) to latest release (alias: update)
  list                     List installed tools and versions (alias: status)

AUDIT & WORKFLOW COMMANDS:
  check   [OPTIONS]        Run audit suite against project (alias: audit)
  init    [PATH]           Scaffold GitHub Action workflow and AGENTS.md

SUBCOMMAND FORWARDING:
  snip     [ARGS]...       Invoke snip with arguments
  vigil    [ARGS]...       Invoke vigil with arguments
  aegis    [ARGS]...       Invoke aegis with arguments
  proven   [ARGS]...       Invoke proven with arguments
  boneyard [ARGS]...       Invoke boneyard with arguments

OPTIONS:
  --dest <DIR>             Custom installation destination
  --path <PATH>            Project path for audit/init (default: .)
  --tools <LIST>           Comma-separated list of tools for check (default: all)
  -h, --help               Print help information
  -V, --version            Print version information
"#,
        VERSION
    );
}

pub fn parse_args() -> Action {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Action::Help;
    }

    match args[0].as_str() {
        "-h" | "--help" | "help" => Action::Help,
        "-V" | "--version" | "version" => Action::Version,
        "install" => {
            let mut tool = "all".to_string();
            let mut dest = None;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--dest" && i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else if !args[i].starts_with('-') {
                    tool = args[i].clone();
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Action::Install { tool, dest }
        }
        "remove" | "uninstall" | "rm" => {
            let mut tool = "all".to_string();
            let mut dest = None;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--dest" && i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else if !args[i].starts_with('-') {
                    tool = args[i].clone();
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Action::Remove { tool, dest }
        }
        "upgrade" | "update" => {
            let mut tool = "all".to_string();
            let mut dest = None;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--dest" && i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else if !args[i].starts_with('-') {
                    tool = args[i].clone();
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Action::Upgrade { tool, dest }
        }
        "list" | "status" | "ls" => {
            let mut dest = None;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--dest" && i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    i += 1;
                }
            }
            Action::Status { dest }
        }
        "check" | "audit" | "run" => {
            let mut path = PathBuf::from(".");
            let mut tools = Vec::new();
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--path" && i + 1 < args.len() {
                    path = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else if args[i] == "--tools" && i + 1 < args.len() {
                    tools = args[i + 1].split(',').map(|s| s.trim().to_string()).collect();
                    i += 2;
                } else if !args[i].starts_with('-') {
                    path = PathBuf::from(&args[i]);
                    i += 1;
                } else {
                    i += 1;
                }
            }
            Action::Check { tools, path }
        }
        "init" => {
            let path = if args.len() > 1 && !args[1].starts_with('-') {
                PathBuf::from(&args[1])
            } else {
                PathBuf::from(".")
            };
            Action::Init { path }
        }
        "snip" | "vigil" | "aegis" | "proven" | "boneyard" => {
            Action::Forward {
                tool: args[0].clone(),
                args: args[1..].to_vec(),
            }
        }
        other => {
            eprintln!("Unknown command: '{}'\nRun 'studio2201 --help' for usage.", other);
            exit(1);
        }
    }
}

pub fn forward_subcommand(tool: &str, args: &[String]) -> Result<i32, String> {
    let mut cmd = Command::new(tool);
    cmd.args(args);
    let status = cmd.status().map_err(|e| format!("Failed to invoke {}: {}", tool, e))?;
    Ok(status.code().unwrap_or(1))
}
