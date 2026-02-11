use crate::consts::Error;
use crate::oxc::fmt;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};

pub fn create_boilerplate_files(
    files: Vec<PathBuf>,
    _root: &std::path::Path,
) -> Result<HashMap<String, String>, Error> {
    let mut tera = Tera::default();
    let ctx = Context::new();
    let files_clone = files.clone();
    let tera_files: Vec<PathBuf> = files
        .into_iter()
    .filter(|i| i.extension().is_some_and(|ext| ext == "jinja2"))
        .collect();
    let mut result: HashMap<String, String> = HashMap::new();

    for i in &tera_files {
        let package_name = i
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let file_stem = i.file_stem().and_then(|s| s.to_str());

        let template_name = match (package_name, file_stem) {
            (Some(pkg), Some(stem)) => format!("{}/{}", pkg, stem),
            _ => i.to_str().unwrap().to_string(),
        };

        tera.add_template_file(i, Some(&template_name))
            .map_err(Error::TemplateParseError)?;
    }

    for file in files_clone {
        let is_template = file.extension().is_some_and(|ext| ext == "jinja2");
        let is_core = file
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n == "core")
            .unwrap_or(false);

        if is_template && is_core {
            let content = fs::read_to_string(&file).map_err(Error::IoError)?;
            let has_extends = content.contains("{% extends");
            let has_blocks = content.contains("{% block");

            if has_blocks && !has_extends {
                continue;
            }
        }

        let output_name: String = if is_template {
            file.with_extension("").to_string_lossy().to_string()
        } else {
            file.to_string_lossy().to_string()
        };

        if is_template {
            let package_name = file
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str());

            let file_stem = file.file_stem().and_then(|s| s.to_str());

            let template_name = match (package_name, file_stem) {
                (Some(pkg), Some(stem)) => format!("{}/{}", pkg, stem),
                _ => file.to_str().unwrap().to_string(),
            };

            let rendered = tera
                .render(&template_name, &ctx)
                .map_err(Error::TemplateParseError)?;

            // Always put app.ts in core directory, regardless of which template rendered it
            let output_path = if file_stem == Some("app.ts") {
                file.with_extension("")
                    .to_string_lossy()
                    .to_string()
                    .replace(&format!("/{}", package_name.unwrap_or("")), "/core")
            } else {
                output_name.clone()
            };

            result.insert(output_path, rendered);
        } else {
            result.insert(
                output_name,
                fs::read_to_string(&file).map_err(Error::IoError)?,
            );
        }
    }

    Ok(result)
}

pub fn write_boilerplate_files(files: HashMap<String, String>, output: &str) -> Result<(), Error> {
    if files.is_empty() {
        return Err(Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No files to write",
        )));
    }

    fs::create_dir_all(output).map_err(Error::IoError)?;

    for (path, content) in files {
        let file_path = Path::new(&path);

        let components: Vec<_> = file_path.components().collect();
        let packages_idx = components.iter().position(|c| c.as_os_str() == "packages");

        let relative_path = if let Some(idx) = packages_idx {
            components
                .iter()
                .skip(idx + 1)
                .map(|c| c.as_os_str())
                .collect::<PathBuf>()
        } else {
            file_path
                .file_name()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("unknown"))
        };

        let output_path = Path::new(output).join(&relative_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(Error::IoError)?;
        }

        let formatted_content = fmt(output_path.as_ref(), &content);
        fs::write(&output_path, formatted_content).map_err(Error::IoError)?;
    }

    Ok(())
}
