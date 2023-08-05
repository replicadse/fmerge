use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("std {0}")]
    Std(#[from] Box<dyn std::error::Error+Sync+Send>),
    #[error("io {0}")]
    IO(#[from] std::io::Error),
    #[error("generic {0}")]
    Generic(String),
    #[error("argument {0}")]
    Argument(String),
    // #[error("experimental command")]
    // ExperimentalCommand,
    #[error("unknown command")]
    UnknownCommand,
}
