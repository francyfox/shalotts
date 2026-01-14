use std::path::Path;

use sha_validator::{ShaMainConfig, validate};

use crate::generate::Generator;
mod generate;

pub struct Scaffolder {
    config: ShaMainConfig,
}

impl Scaffolder {
    pub fn new(path: &str, output: &str) -> Self {
        let validated = validate(path).unwrap();
        let mut generator = Generator::new(Path::new(path));

        Self { config: validated }
    }
}
