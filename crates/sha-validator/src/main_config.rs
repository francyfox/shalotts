use crate::consts::ValidationError;
use log::warn;
use serde::Deserialize;
use sha_utils::get_workspace_root;
use std::collections::HashMap;
use std::fs;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct TabEntry {
    #[validate(required)]
    pub required: Option<bool>,
    pub packages: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ShaMainConfig {
    #[validate(length(min = 1))]
    pub seed: Option<String>,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub ecosystem: Option<String>,

    pub tab: HashMap<String, HashMap<String, TabEntry>>,
}

impl ShaMainConfig {
    pub fn validate_tab_structure(&self) -> Result<(), ValidationError> {
        for (tab_name, tab_entries) in &self.tab {
            for (entry_name, entry) in tab_entries {
                if entry.required.unwrap_or(false) {
                    if entry.packages.is_none() || entry.packages.as_ref().unwrap().is_empty() {
                        return Err(ValidationError::SchemaError(format!("Required tab entry '{}' in '{}' has no packages", entry_name, tab_name)));
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn validate(file_path: &str) -> Result<ShaMainConfig, ValidationError> {
    let mut path = get_workspace_root();
    path.push(file_path.strip_prefix("/").unwrap_or(file_path));

    let content = fs::read_to_string(&path).map_err(ValidationError::IoError)?;
    let main_config: ShaMainConfig = toml::from_str(&content).map_err(ValidationError::ParseError)?;

    main_config.validate().map_err(|errors| {
        warn!("Validation errors: {:?}", errors);
        ValidationError::SchemaError(format!("{:?}", errors))
    })?;

    main_config.validate_tab_structure().map_err(|e| {
        warn!("Tab structure validation failed: {}", e);
        e
    })?;

    log::info!("main config validated");
    Ok(main_config)
}