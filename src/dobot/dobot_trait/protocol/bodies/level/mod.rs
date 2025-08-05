use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Level {
    Low = 0x00,
    High = 0x01,
}

impl TryFrom<u8> for Level {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `Level` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Level::Low),
            0x01 => Ok(Level::High),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}
