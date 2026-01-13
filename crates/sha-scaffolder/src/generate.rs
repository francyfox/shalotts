use sha_utils::{get_workspace_root, parse_dir};
use sha_validator::ShaMainConfig;
use std::{collections::HashMap, path::Path};

struct Node {
    is_dir: bool,
    content: String,
}

pub struct Generator {
    path: String,
    structure: HashMap<String, Node>,
}

impl Generator {
    pub fn new(path: String) -> Self {
        Self {
            path: path.to_string(),
            structure: HashMap::new(),
        }
    }

    pub fn make(&mut self, config: &ShaMainConfig) {
        self.parse_hash_tree(config);
        for (path, node) in &self.structure {
            // println!("Path: {}, is_dir: {}", path, node.is_dir);
        }
    }

    pub fn parse_hash_tree(&mut self, config: &ShaMainConfig) {
        let mut root = get_workspace_root();
        if let Some(parent) = Path::new(&self.path).parent() {
            root.push(parent.strip_prefix("/").unwrap_or(parent));
            root.push("packages");
        };

        println!("{:?}", root);
        let files = parse_dir(root.as_path(), None);
        println!("{:?}", files);
    }

    fn get_route(path: &str) {}
}
