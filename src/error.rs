use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest Error")]
    Reqwest(#[from] reqwest::Error),
    #[error("I/O Error")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
