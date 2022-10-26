use thiserror::Error;


#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlvError {
    #[error("There is no `TlvTag` for the value: `{value:?}`")]
    TlvTagValueError { value: u8 }
}