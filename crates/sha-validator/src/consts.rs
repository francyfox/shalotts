use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Cant read file: {0}")]
    IoError(std::io::Error),
    #[error("Parser TOML error: {0}")]
    ParseError(toml::de::Error),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Missing field value: {0}")]
    EmptyField(String),
}
