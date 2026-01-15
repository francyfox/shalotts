use std::path::Path;

use sha_validator::{ShaMainConfig, validate};

use crate::generate::Generator;
mod generate;

pub struct Scaffolder {
    config: ShaMainConfig,
}

impl Scaffolder {
    pub fn new(path: &str) -> Self {
        let validated = validate(path).unwrap();

        Self { config: validated }
    }
    
    pub fn generate(&self, config_path: &str, output: &str) -> Result<(), sha_utils::consts::Error> {
        let mut generator = Generator::new(Path::new(config_path), Path::new(output));
        generator.parse();
        generator.write()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_elysia_boilerplate() {
        // Path to elysia example config
        let config_path = "examples/elysia/sha.toml";
        
        // Temporary output directory
        let output_path = "./target/test-output/elysia-project";
        
        // Clean up previous test output
        let _ = fs::remove_dir_all(output_path);
        
        // Create scaffolder
        let scaffolder = Scaffolder::new(config_path);
        
        // Generate boilerplate
        let result = scaffolder.generate(config_path, output_path);
        
        // Assert generation succeeded
        assert!(result.is_ok(), "Failed to generate boilerplate: {:?}", result.err());
        
        // Check that output directory was created
        assert!(Path::new(output_path).exists(), "Output directory not created");
        
        println!("✓ Boilerplate generated successfully at: {}", output_path);
    }
}
