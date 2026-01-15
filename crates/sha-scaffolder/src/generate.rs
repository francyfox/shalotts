use sha_utils::{create_boilerplate_files, get_workspace_root, parse_dir, write_boilerplate_files};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use sha_utils::consts::Error;

pub struct Generator<'a> {
    root: PathBuf,
    path: &'a Path,
    output: &'a Path,
    parsed: HashMap<String, String>,
    structure: Vec<PathBuf>,
}

impl<'a> Generator<'a> {
    pub fn new(path: &'a Path, output: &'a Path) -> Self {
        let root = get_workspace_root();

        Self {
            root,
            path,
            output,
            parsed: HashMap::new(),
            structure: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        if let Some(parent) = self.path.parent() {
            self.root.push(parent.strip_prefix("/").unwrap_or(parent));
            self.root.push("packages");
        };

        self.structure = parse_dir(self.root.as_path(), None);
        
        let boilerplate_files =
            create_boilerplate_files(self.structure.clone(), self.root.as_path()).unwrap();

        self.parsed = boilerplate_files;
    }
    
    pub fn write(&self) -> Result<(), Error> {
        write_boilerplate_files(self.parsed.clone(), self.output.to_str().unwrap())
    }
}
