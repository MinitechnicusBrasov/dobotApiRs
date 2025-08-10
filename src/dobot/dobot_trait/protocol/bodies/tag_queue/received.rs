use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

/// Represents PTP linear parameters with velocity and acceleration.
/// This struct corresponds to the Python `tagPTPLParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagQueue {
    pub queue_idx: u64,
}

impl<'a> Body<'a> for TagQueue {
    fn size(&self) -> usize {
        core::mem::size_of::<u64>()
    }

    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        buffer[..size].copy_from_slice(&self.queue_idx.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPLParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<i64>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut queue_idx_bytes = [0u8; 8];
        queue_idx_bytes.copy_from_slice(&buffer[..size]);
        let queue_idx = u64::from_le_bytes(queue_idx_bytes);

        Ok(Self { queue_idx })
    }
}
