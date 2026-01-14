use sha_utils::{create_boilerplate_files, get_workspace_root, parse_dir};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Generator<'a> {
    root: PathBuf,
    path: &'a Path,
    parsed: HashMap<String, String>,
    structure: Vec<PathBuf>,
}

impl<'a> Generator<'a> {
    pub fn new(path: &'a Path) -> Self {
        let root = get_workspace_root();

        Self {
            root,
            path,
            parsed: HashMap::new(),
            structure: Vec::new(),
        }
    }

    pub fn parse_packages(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            self.root.push(parent.strip_prefix("/").unwrap_or(parent));
            self.root.push("packages");
        };

        self.structure = parse_dir(self.root.as_path(), None);
    }

    pub fn parse(&mut self) -> HashMap<String, String> {
        let boilerplate_files =
            create_boilerplate_files(self.structure.clone(), self.root.as_path()).unwrap();

        boilerplate_files
    }
}
