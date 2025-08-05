use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents PTP jump parameters with start/end jump heights and z limit.
/// This struct corresponds to the Python `tagPTPJump2Params` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPJump2Params {
    /// The starting jump height.
    pub start_jump_height: f32,
    /// The ending jump height.
    pub end_jump_height: f32,
    /// The z-axis jump limit.
    pub z_limit: f32,
}

impl Body for TagPTPJump2Params {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 3 floats (`f32`), each 4 bytes, totaling 12 bytes.
    fn size(&self) -> usize {
        3 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagPTPJump2Params` struct into a byte sequence.
    /// It serializes the three `f32` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the start jump height
        buffer[offset..offset + float_size].copy_from_slice(&self.start_jump_height.to_le_bytes());
        offset += float_size;

        // Serialize the end jump height
        buffer[offset..offset + float_size].copy_from_slice(&self.end_jump_height.to_le_bytes());
        offset += float_size;

        // Serialize the z limit
        buffer[offset..offset + float_size].copy_from_slice(&self.z_limit.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPJump2Params` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 3 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the start jump height
        let mut start_jump_height_bytes = [0u8; 4];
        start_jump_height_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let start_jump_height = f32::from_le_bytes(start_jump_height_bytes);
        offset += float_size;

        // Deserialize the end jump height
        let mut end_jump_height_bytes = [0u8; 4];
        end_jump_height_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let end_jump_height = f32::from_le_bytes(end_jump_height_bytes);
        offset += float_size;

        // Deserialize the z limit
        let mut z_limit_bytes = [0u8; 4];
        z_limit_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let z_limit = f32::from_le_bytes(z_limit_bytes);

        Ok(Self {
            start_jump_height,
            end_jump_height,
            z_limit,
        })
    }
}
