use thiserror::Error;

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum Error {
    Stderr(String),
    CommandNotFound(
        #[from]
        #[source]
        which::Error,
    ),
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    VerError(
        #[from]
        #[source]
        std::env::VarError,
    ),
    Generic {
        message: String,
        source: Option<Box<dyn std::error::Error>>,
    },
}

impl Error {
    pub fn message<S: Into<String>>(message: S) -> Self {
        Self::Generic {
            message: message.into(),
            source: None,
        }
    }

    pub fn generic<E: std::error::Error + 'static>(source: E) -> Self {
        Self::Generic {
            message: source.to_string(),
            source: Some(Box::new(source)),
        }
    }
}
