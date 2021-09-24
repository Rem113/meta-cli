#[derive(Debug)]
pub enum Error {
    DockerError(String),
    ImageError(String)
}

impl Error {
    pub fn message(&self) -> &str {
        match self {
            Error::DockerError(message) |
            Error::ImageError(message) => message,
        }
    }
}