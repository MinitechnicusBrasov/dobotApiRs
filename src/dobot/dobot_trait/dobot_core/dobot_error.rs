use std::sync::PoisonError;

use thiserror::Error;

use crate::dobot::dobot_trait::protocol::ProtocolError;

use super::command_sender::CommandSender;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DobotError {
    #[error("A Protocol packet error occured: {0}")]
    Protocol(#[from] ProtocolError),

    #[cfg(feature = "std")]
    #[error("Command sender has been poisoned by a write error")]
    SenderPoisoned,
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
pub fn parse_poison_err<T, U>(result: Result<T, PoisonError<U>>) -> Result<T, DobotError> {
    match result {
        Ok(x) => Ok(x),
        Err(_) => Err(DobotError::SenderPoisoned),
    }
}

#[cfg(not(feature = "std"))]
pub fn parse_poison_err<T, U>(result: Result<T, DobotError>) -> Result<T, DobotError> {
    return result;
}
