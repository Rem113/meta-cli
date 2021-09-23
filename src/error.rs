#[derive(Debug)]
pub enum Error {
    DockerError(String),
    ImageError(String),
    UsageError(String)
}

impl Error {
    pub fn message(&self) -> &str {
        match self {
            Error::DockerError(message) => message,
            Error::ImageError(message) => message,
            Error::UsageError(message) => message,
        }
    }
}