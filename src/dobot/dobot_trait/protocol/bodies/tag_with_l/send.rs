use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
use core::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum TagVersionRail {
    VerV1 = 0,
    VerV2 = 1,
}

/// Implements `TryFrom<u8>` to safely convert a byte to a `TagVersionRail`.
/// This is used during deserialization to handle invalid version values.
impl TryFrom<u8> for TagVersionRail {
    type Error = ProtocolError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(TagVersionRail::VerV1),
            1 => Ok(TagVersionRail::VerV2),
            _ => Err(ProtocolError::InvalidTagVersion(v)),
        }
    }
}

/// Represents a tag with rail information and version.
/// This struct corresponds to the Python `tagWithL` dataclass.
#[derive(Debug, PartialEq, Eq)]
pub struct TagWithL {
    pub is_with_rail: bool,
    pub version: TagVersionRail,
}

impl<'a> Body<'a> for TagWithL {
    /// Returns the size of the serialized body in bytes.
    /// A boolean is 1 byte, and the enum (as u8) is 1 byte.
    fn size(&self) -> usize {
        1 + core::mem::size_of::<TagVersionRail>()
    }

    /// Packs the `TagWithL` struct into a byte sequence.
    /// The boolean is serialized as a single byte (0 or 1), followed by the enum's u8 value.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        if buffer.len() < self.size() {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the boolean as a single byte
        buffer[0] = self.is_with_rail as u8;

        // Serialize the enum version as its u8 value
        buffer[1] = self.version as u8;

        Ok(self.size())
    }

    /// Unpacks a byte sequence into a `TagWithL` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        if buffer.len() < 2 {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the boolean from the first byte
        let is_with_rail = buffer[0] == 1;

        // Deserialize the version from the second byte
        let version_u8 = buffer[1];
        let version = TagVersionRail::try_from(version_u8)?;

        Ok(Self {
            is_with_rail,
            version,
        })
    }
}
