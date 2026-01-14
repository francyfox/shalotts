use crate::{consts::Error, get_workspace_root};
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
        .filter(|i| i.extension().map_or(false, |ext| ext == "jinja2"))
        .collect();
    let mut result: HashMap<String, String> = HashMap::new();

    for i in &tera_files {
        tera.add_template_file(i, i.to_str())
            .map_err(Error::TemplateParseError)?;
    }

    for file in files_clone {
        let is_template = file.extension().map_or(false, |ext| ext == "jinja2");
        let name = file.to_str().ok_or_else(|| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid path",
            ))
        })?;
        let output_name: String = if is_template {
            file.with_extension("").to_string_lossy().to_string()
        } else {
            file.to_string_lossy().to_string()
        };

        if is_template {
            let rendered = tera.render(name, &ctx).map_err(Error::TemplateParseError)?;
            result.insert(output_name, rendered);
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
    let root_path = get_workspace_root();

    for (path, content) in files {
        let relative = Path::new(&path).strip_prefix(&root_path);

        fs::write(format!("{}/{:?}", output, relative), content).map_err(Error::IoError)?;
    }

    Ok(())
}
