use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::ProtocolError;

/// Represents a WAIT command with a timeout.
/// This struct corresponds to the Python `tagWAITCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagWAITCmd {
    /// The timeout value, a 32-bit unsigned integer.
    pub timeout: u32,
}

impl Body for TagWAITCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u32`, which is 4 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u32>()
    }

    /// Packs the `TagWAITCmd` struct into a byte sequence.
    /// It serializes the `timeout` value into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the timeout value
        buffer[..size].copy_from_slice(&self.timeout.to_le_bytes());
        
        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagWAITCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the timeout value
        let mut timeout_bytes = [0u8; 4];
        timeout_bytes.copy_from_slice(&buffer[..size]);
        let timeout = u32::from_le_bytes(timeout_bytes);
        
        Ok(Self { timeout })
    }
}
