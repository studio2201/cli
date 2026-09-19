//! tests/cli_tests.rs — Comprehensive unit and integration tests for studio2201 CLI
use std::fs;
use std::path::PathBuf;
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

    let wf = tmp.join(".github").join("workflows").join("studio2201.yml");
    assert!(wf.is_file());
    let content = fs::read_to_string(&wf).unwrap();
    assert!(content.contains("uses: studio2201/studio2201@master"));

    let agent_md = tmp.join("AGENTS.md");
    assert!(agent_md.is_file());
    let agent_content = fs::read_to_string(&agent_md).unwrap();
    assert!(agent_content.contains("studio2201 check"));

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

