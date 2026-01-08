use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Не удалось прочитать файл: {0}")]
    IoError(std::io::Error),
    #[error("Ошибка парсинга TOML: {0}")]
    ParseError(toml::de::Error),
    #[error("Отсутствует обязательное поле: {0}")]
    MissingField(String),
    #[error("Пустое значение поля: {0}")]
    EmptyField(String),
}
