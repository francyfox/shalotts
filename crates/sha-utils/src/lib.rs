use std::path::{Path, PathBuf};

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
