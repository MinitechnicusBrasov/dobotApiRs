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
