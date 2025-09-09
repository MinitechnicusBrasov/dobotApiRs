use crate::dobot::dobot_trait::protocol::{protocol_error::ProtocolError, Body};

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

impl<'a> Body<'a> for Level {
    fn size(&self) -> usize {
        core::mem::size_of::<u8>()
    }

    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let level = *self as u8;
        buffer[0] = level;
        Ok(core::mem::size_of::<u8>())
    }

    fn deserialize(buffer: &'a [u8]) -> Result<Self, ProtocolError> {
        let level = Level::try_from(buffer[0])?;
        Ok(level)
    }
}
