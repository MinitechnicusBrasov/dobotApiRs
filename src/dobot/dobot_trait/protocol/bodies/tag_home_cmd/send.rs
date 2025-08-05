use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

/// Represents a home command with a reserved integer field.
/// This struct corresponds to the Python `tagHomeCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagHomeCmd {
    pub reserved: u32,
}

impl<'a> Body<'a> for TagHomeCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one unsigned 32-bit integer (`u32`), which is 4 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u32>()
    }

    /// Packs the `TagHomeCmd` struct into a byte sequence.
    /// It serializes the `reserved` field into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the u32 as little-endian bytes
        buffer[..size].copy_from_slice(&self.reserved.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagHomeCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u32>();
        if buffer.len() != size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the u32 from the buffer
        let mut reserved_bytes = [0u8; 4];
        reserved_bytes.copy_from_slice(&buffer[..size]);
        let reserved = u32::from_le_bytes(reserved_bytes);

        Ok(Self { reserved })
    }
}
