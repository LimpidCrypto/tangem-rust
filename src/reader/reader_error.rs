use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReaderError {
    #[error("No reader found for index {index:?}.")]
    ReaderNotFound { index: usize },
    #[error("There is no card connected. Use `connect`.")]
    NoCardConnected,
}