use sha_utils::{get_workspace_root, parse_dir};
use sha_validator::ShaMainConfig;
use std::collections::HashMap;

struct Node {
    is_dir: bool,
    content: String,
}

pub struct Generator {
    structure: HashMap<String, Node>,
}

impl Generator {
    pub fn new() -> Self {
        return Self {
            structure: HashMap::new(),
        };
    }

    pub fn make(&mut self, config: &ShaMainConfig) {
        self.parse_hash_tree(config);
        for (path, node) in &self.structure {
            // println!("Path: {}, is_dir: {}", path, node.is_dir);
        }
    }

    pub fn parse_hash_tree(&mut self, config: &ShaMainConfig) {
        self.structure.insert(
            "/shalotts".into(),
            Node {
                is_dir: true,
                content: "".into(),
            },
        );
        self.structure.insert(
            "/shalotts/modules".into(),
            Node {
                is_dir: true,
                content: "".into(),
            },
        );

        let mut root = get_workspace_root();
        root.push("packages");
        parse_dir(root.as_path(), None);
    }

    fn get_route(path: &str) {}
}
