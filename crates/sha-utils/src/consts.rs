use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse template")]
    TemplateParseError(#[from] tera::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
