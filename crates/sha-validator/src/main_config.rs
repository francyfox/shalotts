use crate::consts::ValidationError;
use serde::Deserialize;
use sha_utils::get_workspace_root;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct ShaMainConfig {
    pub seed: String,
    pub name: String,
    pub ecosystem: String,
    pub tab: HashMap<String, HashMap<String, Value>>,
}

pub fn validate(file_path: &str) -> Result<ShaMainConfig, ValidationError> {
    let mut path = get_workspace_root();
    path.push(file_path.strip_prefix("/").unwrap_or(file_path));

    let content = fs::read_to_string(path).map_err(ValidationError::IoError)?;
    let main_config: ShaMainConfig =
        toml::from_str(&content).map_err(ValidationError::ParseError)?;

    log::info!("main config parsed");
    Ok(main_config)
}

// pub fn validate_tab_structure(path: PathBuf, tabs: HashMap<String, HashMap<String, Value>>) {
//     for tab in tabs  {

//     }
// }
