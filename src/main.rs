//! main.rs — studio2201 unified CLI entry point
use std::process::exit;
use studio2201_cli::cli::{self, Action};
use studio2201_cli::init;
use studio2201_cli::runner;
use studio2201_cli::toolchain;

fn main() {
    let action = cli::parse_args();

    let res = match action {
        Action::Help => {
            cli::print_help();
            Ok(0)
        }
        Action::Version => {
            println!("studio2201 {}", cli::VERSION);
            Ok(0)
        }
        Action::Install { tool, dest } => {
            let d = toolchain::resolve_dest(dest.as_deref());
            println!("Installing studio2201 tool '{}' to {}...", tool, d.display());
            toolchain::install_tool(&tool, &d).map(|_| 0)
        }
        Action::Remove { tool, dest } => {
            let d = toolchain::resolve_dest(dest.as_deref());
            println!("Removing studio2201 tool '{}' from {}...", tool, d.display());
            toolchain::remove_tool(&tool, &d).map(|_| 0)
        }
        Action::Upgrade { tool, dest } => {
            let d = toolchain::resolve_dest(dest.as_deref());
            println!("Upgrading studio2201 tool '{}' in {}...", tool, d.display());
            toolchain::upgrade_tool(&tool, &d).map(|_| 0)
        }
        Action::Status { dest } => {
            let d = toolchain::resolve_dest(dest.as_deref());
            toolchain::print_status(&d).map(|_| 0)
        }
        Action::Check { tools, path } => {
            runner::run_suite(&tools, &path)
        }
        Action::Init { path } => {
            init::init_project(&path).map(|_| 0)
        }
        Action::Forward { tool, args } => {
            cli::forward_subcommand(&tool, &args)
        }
    };

    match res {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}
