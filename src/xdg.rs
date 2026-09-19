//! xdg.rs — Linux XDG Base Directory specification paths for studio2201.
use std::env;
use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."))
}

pub fn bin_dir() -> PathBuf {
    env::var("XDG_BIN_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".local").join("bin"))
}

pub fn config_dir() -> PathBuf {
    env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".config"))
        .join("studio2201")
}

pub fn state_dir() -> PathBuf {
    env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".local").join("state"))
        .join("studio2201")
}

pub fn cache_dir() -> PathBuf {
    env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".cache"))
        .join("studio2201")
}

pub fn data_dir() -> PathBuf {
    env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home_dir().join(".local").join("share"))
        .join("studio2201")
}

pub fn tool_path(tool: &str) -> PathBuf {
    bin_dir().join(tool)
}
