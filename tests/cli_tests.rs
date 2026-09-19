//! tests/cli_tests.rs — Comprehensive unit and integration tests for studio2201 CLI
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use studio2201_cli::init;
use studio2201_cli::toolchain::{self, ALL_TOOLS};
use studio2201_cli::xdg;

#[test]
fn test_all_tools_list() {
    assert_eq!(ALL_TOOLS.len(), 5);
    assert!(ALL_TOOLS.contains(&"snip"));
    assert!(ALL_TOOLS.contains(&"vigil"));
    assert!(ALL_TOOLS.contains(&"aegis"));
    assert!(ALL_TOOLS.contains(&"proven"));
    assert!(ALL_TOOLS.contains(&"boneyard"));
}

#[test]
fn test_xdg_paths() {
    let home = xdg::home_dir();
    assert!(!home.as_os_str().is_empty());
    let bin = xdg::bin_dir();
    assert!(!bin.as_os_str().is_empty());
    let config = xdg::config_dir();
    assert!(config.to_str().unwrap().contains("studio2201"));
}

#[test]
fn test_resolve_dest() {
    let custom = PathBuf::from("/tmp/custom_bin");
    let resolved = toolchain::resolve_dest(Some(&custom));
    assert_eq!(resolved, custom);

    let default = toolchain::resolve_dest(None);
    assert_eq!(default, xdg::bin_dir());
}

#[test]
fn test_init_scaffolding() {
    let tmp = std::env::temp_dir().join("test_studio2201_init");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let res = init::init_project(&tmp);
    assert!(res.is_ok());

    let tools = ["snip", "vigil", "aegis", "proven", "boneyard"];
    for tool in tools {
        let wf = tmp.join(".github").join("workflows").join(format!("{tool}.yml"));
        assert!(wf.is_file(), "Workflow {tool}.yml should exist");
        let content = fs::read_to_string(&wf).unwrap();
        assert!(content.contains(&format!("name: {tool}")));
        assert!(content.contains("uses: studio2201/studio2201@v1"));
        assert!(content.contains(&format!("tools: '{tool}'")));
    }

    let agent_md = tmp.join("AGENTS.md");
    assert!(agent_md.is_file());
    let agent_content = fs::read_to_string(&agent_md).unwrap();
    assert!(agent_content.contains("studio2201 check"));
    assert!(agent_content.contains("Primary Anchor Badge & Executive Assurance Scorecard"));
    assert!(agent_content.contains("secured by studio2201"));
    assert!(agent_content.contains("snip-0%20secrets-2f6f5e"));
    assert!(agent_content.contains("vigil-0%20dependencies-2f6f5e"));
    assert!(agent_content.contains("aegis-PQC%20compliant-2f6f5e"));
    assert!(agent_content.contains("proven-ML--DSA--65%20verified-2f6f5e"));
    assert!(agent_content.contains("boneyard-maintained-2f6f5e"));
    assert!(agent_content.contains("5 Dedicated Dynamic Workflow Status Badges"));
    for tool in tools {
        assert!(agent_content.contains(&format!("[b-{tool}]:")));
    }

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_init_idempotent_existing_files() {
    let tmp = std::env::temp_dir().join("test_studio2201_init_idempotent");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(tmp.join(".github").join("workflows")).unwrap();

    let wf = tmp.join(".github").join("workflows").join("snip.yml");
    fs::write(&wf, "# Pre-existing custom workflow").unwrap();
    let agent_md = tmp.join("AGENTS.md");
    fs::write(&agent_md, "# Pre-existing custom instructions").unwrap();

    let res = init::init_project(&tmp);
    assert!(res.is_ok());

    assert_eq!(fs::read_to_string(&wf).unwrap(), "# Pre-existing custom workflow");
    assert_eq!(
        fs::read_to_string(&agent_md).unwrap(),
        "# Pre-existing custom instructions"
    );

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_tool_installed_check() {
    let tmp = std::env::temp_dir().join("test_bin_check");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    assert!(toolchain::check_installed("non_existent_tool", &tmp).is_none());

    let fake_bin = tmp.join("fake_tool");
    fs::write(&fake_bin, "#!/bin/sh\necho fake 1.0.0\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&fake_bin).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&fake_bin, perms).unwrap();
    }

    assert!(toolchain::check_installed("fake_tool", &tmp).is_some());
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_boneyard_graceful_skip() {
    let tmp = std::env::temp_dir().join("test_boneyard_skip");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    let res = studio2201_cli::runner::run_suite(
        &["boneyard".to_string()],
        &tmp,
    );
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0);

    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_invalid_argument_exit_code_2() {
    let bin = env!("CARGO_BIN_EXE_studio2201");
    let test_cases: &[&[&str]] = &[
        &["--invalid-arg"],
        &["-z"],
        &["--unknown-flag-123"],
        &["nonexistent-command"],
        &["install", "--unknown-flag"],
        &["check", "--invalid-flag"],
    ];

    for args in test_cases {
        let output = Command::new(bin)
            .args(*args)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute studio2201 binary: {}", e));

        assert_eq!(
            output.status.code(),
            Some(2),
            "expected exit code 2 for args {:?}, got {:?}",
            args,
            output.status.code()
        );
        assert!(
            !output.stderr.is_empty(),
            "expected non-empty stderr for args {:?}",
            args
        );
    }
}

#[test]
fn test_cli_parsing_errors_and_exit_code_2_cases() {
    let to_args = |slice: &[&str]| -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    };

    assert!(studio2201_cli::cli::parse_from(&to_args(&["--invalid-arg"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["-z"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["nonexistent-command"])).is_err());

    assert!(studio2201_cli::cli::parse_from(&to_args(&["install", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["remove", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["upgrade", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["list", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["check", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["init", "--unknown-flag"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["install", "--dest"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["check", "--path"])).is_err());
    assert!(studio2201_cli::cli::parse_from(&to_args(&["check", "--tools"])).is_err());
}

