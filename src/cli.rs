//! cli.rs — Command-line interface definitions and parser for studio2201
use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

pub const VERSION: &str = "0.1.4";

#[derive(Debug, PartialEq, Eq)]
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

fn parse_tool_and_dest(args: &[String]) -> Result<(String, Option<PathBuf>), String> {
    let mut tool = None;
    let mut dest = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Err("HELP".into()),
            "--dest" => {
                if i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    return Err("missing argument for '--dest'".into());
                }
            }
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option '{}'", arg));
            }
            arg => {
                if tool.is_some() {
                    return Err(format!("unexpected argument '{}'", arg));
                }
                tool = Some(arg.to_string());
                i += 1;
            }
        }
    }
    Ok((tool.unwrap_or_else(|| "all".to_string()), dest))
}

fn parse_dest_only(args: &[String]) -> Result<Option<PathBuf>, String> {
    let mut dest = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Err("HELP".into()),
            "--dest" => {
                if i + 1 < args.len() {
                    dest = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    return Err("missing argument for '--dest'".into());
                }
            }
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option '{}'", arg));
            }
            arg => {
                return Err(format!("unexpected argument '{}'", arg));
            }
        }
    }
    Ok(dest)
}

fn parse_check(args: &[String]) -> Result<(Vec<String>, PathBuf), String> {
    let mut path = None;
    let mut tools = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Err("HELP".into()),
            "--path" => {
                if i + 1 < args.len() {
                    path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    return Err("missing argument for '--path'".into());
                }
            }
            "--tools" => {
                if i + 1 < args.len() {
                    tools = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    i += 2;
                } else {
                    return Err("missing argument for '--tools'".into());
                }
            }
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option '{}'", arg));
            }
            arg => {
                if path.is_some() {
                    return Err(format!("unexpected argument '{}'", arg));
                }
                path = Some(PathBuf::from(arg));
                i += 1;
            }
        }
    }
    Ok((tools, path.unwrap_or_else(|| PathBuf::from("."))))
}

fn parse_init(args: &[String]) -> Result<PathBuf, String> {
    let mut path = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Err("HELP".into()),
            arg if arg.starts_with('-') => {
                return Err(format!("unknown option '{}'", arg));
            }
            arg => {
                if path.is_some() {
                    return Err(format!("unexpected argument '{}'", arg));
                }
                path = Some(PathBuf::from(arg));
                i += 1;
            }
        }
    }
    Ok(path.unwrap_or_else(|| PathBuf::from(".")))
}

pub fn parse_from(args: &[String]) -> Result<Action, String> {
    if args.is_empty() {
        return Ok(Action::Help);
    }

    match args[0].as_str() {
        "-h" | "--help" | "help" => Ok(Action::Help),
        "-V" | "--version" | "version" => Ok(Action::Version),
        "install" => match parse_tool_and_dest(&args[1..]) {
            Ok((tool, dest)) => Ok(Action::Install { tool, dest }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "remove" | "uninstall" | "rm" => match parse_tool_and_dest(&args[1..]) {
            Ok((tool, dest)) => Ok(Action::Remove { tool, dest }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "upgrade" | "update" => match parse_tool_and_dest(&args[1..]) {
            Ok((tool, dest)) => Ok(Action::Upgrade { tool, dest }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "list" | "status" | "ls" => match parse_dest_only(&args[1..]) {
            Ok(dest) => Ok(Action::Status { dest }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "check" | "audit" | "run" => match parse_check(&args[1..]) {
            Ok((tools, path)) => Ok(Action::Check { tools, path }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "init" => match parse_init(&args[1..]) {
            Ok(path) => Ok(Action::Init { path }),
            Err(e) if e == "HELP" => Ok(Action::Help),
            Err(e) => Err(e),
        },
        "snip" | "vigil" | "aegis" | "proven" | "boneyard" => Ok(Action::Forward {
            tool: args[0].clone(),
            args: args[1..].to_vec(),
        }),
        other => Err(format!("unknown command or flag '{}'", other)),
    }
}

pub fn parse_args() -> Action {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_from(&args) {
        Ok(action) => action,
        Err(e) => {
            eprintln!("error: {}\nRun 'studio2201 --help' for usage.", e);
            exit(2);
        }
    }
}

pub fn forward_subcommand(tool: &str, args: &[String]) -> Result<i32, String> {
    let mut cmd = Command::new(tool);
    cmd.args(args);
    let status = cmd.status().map_err(|e| format!("Failed to invoke {}: {}", tool, e))?;
    Ok(status.code().unwrap_or(1))
}
