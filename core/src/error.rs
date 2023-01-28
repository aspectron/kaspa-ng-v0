use thiserror::Error;
use wasm_bindgen::prelude::*;

use workflow_ux::error::Error as UxError;
use workflow_terminal::error::Error as TerminalError;
use kaspa_wallet_cli::error::Error as CLIError;
use std::sync::PoisonError;

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => ( workflow_ux::error::Error::String(format_args!($($t)*).to_string()) )
}
pub use error;

// #[allow(non_camel_case_types)]
#[derive(Debug, Error)]
pub enum Error {
    #[cfg(not(target_os = "solana"))]
    #[error("{0}")]
    String(String),

    #[cfg(not(target_os = "solana"))]
    #[error("{0:#?}")]
    JsValue(JsValue),

    #[cfg(not(target_os = "solana"))]
    #[error("{0:#?}")]
    UxError(#[from] UxError),

    #[error("Channel send error: {0}")]
    ChannelSendError(String),

    #[error("Channel receive error: {0}")]
    ChannelReceiveError(String),

    #[error("Terminal error: {0}")]
    TerminalError(#[from] TerminalError),

    #[error("CLI error: {0}")]
    CLIError(#[from] CLIError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("PoisonError error: {0}")]
    PoisonError(String),
}

impl From<Error> for UxError {
    fn from(err: Error) -> UxError {
        UxError::String(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::String(err.to_string())
    }
}
impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::String(err)
    }
}

impl From<JsValue> for Error {
    fn from(val: JsValue) -> Self {
        Self::JsValue(val)
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> JsValue {
        JsValue::from(format!("{:?}", error))
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Error {
        Error::PoisonError(format!("{:?}", error))
    }
}

/*
impl<T> From<SendError<T>> for Error {
    fn from(error: SendError<T>) -> Error {
        Error::ChannelSendError(format!("{:?}",error))
    }
}

impl From<RecvError> for Error {
    fn from(error: RecvError) -> Error {
        Error::ChannelReceiveError(format!("{:?}",error))
    }
}

impl From<TerminalError> for Error {
    fn from(error: TerminalError) -> Error {
        Error::TerminalError(format!("{:?}", error))
    }
}

*/
