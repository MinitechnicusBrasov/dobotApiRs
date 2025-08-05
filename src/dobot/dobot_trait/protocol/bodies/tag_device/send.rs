use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
use core::convert::TryFrom;

/// Represents the version for the color sensor and IR tag.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TagVersionColorSensorAndIR {
    Version1 = 0x00,
    Version2 = 0x01,
}

impl TryFrom<u8> for TagVersionColorSensorAndIR {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `TagVersionColorSensorAndIR` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TagVersionColorSensorAndIR::Version1),
            0x01 => Ok(TagVersionColorSensorAndIR::Version2),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents a generic device tag command.
/// This struct corresponds to the Python `tagDevice` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagDevice {
    pub is_enabled: bool,
    pub port: u8,
    pub version: TagVersionColorSensorAndIR,
}

impl<'a> Body<'a> for TagDevice {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of three `u8`s, each 1 byte, totaling 3 bytes.
    fn size(&self) -> usize {
        3 * core::mem::size_of::<u8>()
    }

    /// Packs the `TagDevice` struct into a byte sequence.
    /// It serializes the `is_enabled` (`bool` as `u8`), `port` (`u8`), and
    /// `version` (`TagVersionColorSensorAndIR` as `u8`) into the buffer using
    /// little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();

        // Serialize the boolean `is_enabled` as a u8 (1 or 0)
        let is_enabled_byte = if self.is_enabled { 1u8 } else { 0u8 };
        buffer[offset..offset + u8_size].copy_from_slice(&is_enabled_byte.to_le_bytes());
        offset += u8_size;

        // Serialize the port
        buffer[offset..offset + u8_size].copy_from_slice(&self.port.to_le_bytes());
        offset += u8_size;

        // Serialize the version enum value as a u8
        buffer[offset..offset + u8_size].copy_from_slice(&(self.version as u8).to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagDevice` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 3 * core::mem::size_of::<u8>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Deserialize the `is_enabled` u8 and convert to bool
        let is_enabled = buffer[0] == 1;

        // Deserialize the port
        let mut port_bytes = [0u8; 1];
        port_bytes.copy_from_slice(&buffer[u8_size..u8_size + u8_size]);
        let port = u8::from_le_bytes(port_bytes);

        // Deserialize the version `u8` and convert to `TagVersionColorSensorAndIR` enum
        let version = TagVersionColorSensorAndIR::try_from(buffer[2 * u8_size])?;

        Ok(Self {
            is_enabled,
            port,
            version,
        })
    }
}
