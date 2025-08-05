use thiserror::Error; // Import Infallible

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("The provided buffer was too small to hold the entire packet")]
    BufferTooSmall,
    #[error("Body data was passed for a queued command index packet")]
    PassedBodyAndQueuedIndex,
    #[error("The packet is missing the start bytes (0xAA 0xAA)")]
    MissingStartBytes,
    #[error("The packet size does not match the length field")]
    LengthMismatch,
    #[error("The packet checksum is incorrect")]
    ChecksumError,
    #[error("Invalid command ID received: {0}")]
    InvalidCommandID(u8),
    #[error("Invalid Value in Body")]
    InvalidEnumValue,
    #[error("Invalid tag version: {0}")]
    InvalidTagVersion(u8),
    #[error("Invalid alarm code: {0}")]
    InvalidAlarmCode(u16),
    #[error("Invalid Handhold Teaching Trigger Mode {0}")]
    InvalidHHTTrigMode(u8),
}
