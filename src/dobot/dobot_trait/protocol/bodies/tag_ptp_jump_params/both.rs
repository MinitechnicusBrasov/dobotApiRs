use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

/// Represents PTP jump parameters with jump height and z limit.
/// This struct corresponds to the Python `tagPTPJumpParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPJumpParams {
    /// Jump height.
    pub jump_height: f32,
    /// Z-axis jump limit.
    pub z_limit: f32,
}

impl Body for TagPTPJumpParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 2 floats (`f32`), each 4 bytes, totaling 8 bytes.
    fn size(&self) -> usize {
        2 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagPTPJumpParams` struct into a byte sequence.
    /// It serializes the two `f32` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the jump height
        buffer[offset..offset + float_size].copy_from_slice(&self.jump_height.to_le_bytes());
        offset += float_size;

        // Serialize the z limit
        buffer[offset..offset + float_size].copy_from_slice(&self.z_limit.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPJumpParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 2 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the jump height
        let mut jump_height_bytes = [0u8; 4];
        jump_height_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let jump_height = f32::from_le_bytes(jump_height_bytes);
        offset += float_size;

        // Deserialize the z limit
        let mut z_limit_bytes = [0u8; 4];
        z_limit_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let z_limit = f32::from_le_bytes(z_limit_bytes);

        Ok(Self {
            jump_height,
            z_limit,
        })
    }
}
