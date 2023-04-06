// use thiserror::Error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("client error: {0}")]
    ClientBuilder(String),
    #[error("failed to read {0} from environment")]
    VarError(String),
    #[error("Missing Vault Token")]
    MissingToken,
    #[error(transparent)]
    Decode(#[from] serde_json::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("error")]
    Status,
}

/// A `Result` alias where the `Err` case is "vault::Error`.
pub type Result<T> = std::result::Result<T, Error>;
