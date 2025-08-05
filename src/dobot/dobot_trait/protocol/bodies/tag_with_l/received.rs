use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
use crate::dobot::dobot_trait::protocol::Body;

/// Represents a return tag with rail information.
/// This struct corresponds to the Python `tagWithLReturn` dataclass.
#[derive(Debug, PartialEq, Eq)]
pub struct TagWithLReturn {
    pub is_with_rail: bool,
}

impl Body for TagWithLReturn {
    /// Returns the size of the serialized body in bytes.
    /// A boolean is serialized as a single byte.
    fn size(&self) -> usize {
        1
    }

    /// Packs the `TagWithLReturn` struct into a byte sequence.
    /// The boolean is serialized as a single byte (0 or 1).
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        // Ensure the buffer is large enough for the single byte
        if buffer.len() < self.size() {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the boolean as a single byte
        buffer[0] = self.is_with_rail as u8;

        Ok(self.size())
    }

    /// Unpacks a byte sequence into a `TagWithLReturn` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        // Ensure the buffer contains exactly one byte for the boolean
        if buffer.len() != 1 {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the boolean from the first byte
        let is_with_rail = buffer[0] == 1;

        Ok(Self { is_with_rail })
    }
}
