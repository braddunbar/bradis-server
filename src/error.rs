use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SetGlobalDefault(#[from] SetGlobalDefaultError),
}
