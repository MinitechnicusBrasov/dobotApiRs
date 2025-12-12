use thiserror::Error;

use crate::dobot::dobot_trait::protocol::ProtocolError;

use super::command_sender::CommandSender;

#[derive(Debug, Error, Clone)]
pub enum DobotError {
    #[error("A Protocol packet error occured: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("Couldn't parse byte array to string")]
    StrConversion(#[from] core::str::Utf8Error),

    #[cfg(feature = "std")]
    #[error("Command sender has been poisoned by a write error")]
    SenderPoisoned,

    #[cfg(feature = "std")]
    #[error("A serial error occured: {0}")]
    SerialError(#[from] serialport::Error),

    #[error("No response from device")]
    NoResponse,
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("Serial port error")]
    Serial,
    #[error("IO error")]
    IO,
}

#[cfg(feature = "std")]
pub fn parse_poison_err<T, U>(result: Result<T, std::sync::PoisonError<U>>) -> Result<T, DobotError> {
    match result {
        Ok(x) => Ok(x),
        Err(_) => Err(DobotError::SenderPoisoned),
    }
}

#[cfg(not(feature = "std"))]
pub fn parse_poison_err<T, U>(result: Result<T, DobotError>) -> Result<T, DobotError> {
    return result;
}
