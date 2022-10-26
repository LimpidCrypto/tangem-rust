use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    #[error("Unknown command: {command:?}")]
    CommandNotFound { command: String }
}