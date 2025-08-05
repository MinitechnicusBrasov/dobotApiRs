use thiserror::Error;

use crate::dobot::dobot_trait::protocol::ProtocolError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DobotError {
    #[error("A Protocol packet error occured: {0}")]
    Protocol(#[from] ProtocolError),
    #[error("No response from device")]
    NoResponse,
    #[error("Timeout waiting for response")]
    Timeout,
    #[error("Serial port error")]
    Serial,
    #[error("IO error")]
    IO,
}
