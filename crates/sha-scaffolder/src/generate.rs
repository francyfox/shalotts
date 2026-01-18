use sha_utils::consts::Error;
use sha_utils::{create_boilerplate_files, get_workspace_root, parse_dir, write_boilerplate_files};
use std::{
    collections::HashMap,
    fs::remove_dir_all,
    path::{Path, PathBuf},
};

pub struct Generator<'a> {
    root: PathBuf,
    path: &'a Path,
    output: &'a Path,
    parsed: HashMap<String, String>,
    structure: Vec<PathBuf>,
}

impl<'a> Generator<'a> {
    pub fn new(path: &'a Path, output: &'a Path) -> Self {
        Self {
            root: PathBuf::new(),
            path,
            output,
            parsed: HashMap::new(),
            structure: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        // path points to sha.toml file, get its parent for packages
        if let Some(example_dir) = self.path.parent() {
            // If path is relative, resolve it from workspace root
            let absolute_path = if self.path.is_absolute() {
                example_dir.to_path_buf()
            } else {
                get_workspace_root().join(example_dir)
            };
            self.root = absolute_path.join("packages");
        }

        self.structure = parse_dir(self.root.as_path(), None);

        let boilerplate_files =
            create_boilerplate_files(self.structure.clone(), self.root.as_path()).unwrap();

        self.parsed = boilerplate_files;
    }

    pub fn write(&self) -> Result<(), Error> {
        write_boilerplate_files(self.parsed.clone(), self.output.to_str().unwrap())
    }

    pub fn rm_output_dir(&mut self) {
        if self.output.exists() {
            remove_dir_all(self.output).unwrap();
        }
    }
}
