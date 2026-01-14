use nanoid::nanoid;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn is_workspace_root(path: &Path) -> bool {
    std::fs::read_to_string(path.join("Cargo.toml"))
        .map(|content| content.contains("[workspace]"))
        .unwrap_or(false)
}

pub fn get_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest_dir.ancestors() {
        if ancestor.join("Cargo.toml").exists() && is_workspace_root(ancestor) {
            return ancestor.to_path_buf();
        }
    }
    manifest_dir
}

pub fn is_core_dir(path: &Path) -> bool {
    path.components().any(|i| i.as_os_str() == "core")
}

pub fn parse_dir(path: &Path, _ignore: Option<&Path>) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|i| i.ok())
        .map(|i| i.into_path())
        .collect()
}

fn random(size: usize) -> Vec<u8> {
    let result: Vec<u8> = vec![0; size];

    result
}

pub fn generate_seed() {
    nanoid!(10, &nanoid::alphabet::SAFE, random);
}
