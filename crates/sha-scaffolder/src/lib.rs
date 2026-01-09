use sha_validator::{validate, ShaMainConfig};

pub struct Scaffolder {
    config: ShaMainConfig
}

impl Scaffolder {
    pub fn new(path: &str) -> Self {              
        let validated = validate(path).unwrap();
        
        Self {
            config: validated
        }
    }
}