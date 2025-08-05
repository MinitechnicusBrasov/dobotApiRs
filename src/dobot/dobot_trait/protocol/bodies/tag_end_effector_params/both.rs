use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents end effector parameters with x, y, and z bias coordinates.
/// This struct corresponds to the Python `tagEndEffectorParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagEndEffectorParams {
    pub x_bias: f64,
    pub y_bias: f64,
    pub z_bias: f64,
}

impl<'a> Body<'a> for TagEndEffectorParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 3 double-precision floating-point numbers (`f64`),
    /// each 8 bytes.
    fn size(&self) -> usize {
        3 * core::mem::size_of::<f64>()
    }

    /// Packs the `TagEndEffectorParams` struct into a byte sequence.
    /// It serializes the three `f64` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let double_size = core::mem::size_of::<f64>();

        // Serialize the xBias, yBias, zBias coordinates
        buffer[offset..offset + double_size].copy_from_slice(&self.x_bias.to_le_bytes());
        offset += double_size;
        buffer[offset..offset + double_size].copy_from_slice(&self.y_bias.to_le_bytes());
        offset += double_size;
        buffer[offset..offset + double_size].copy_from_slice(&self.z_bias.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagEndEffectorParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 3 * core::mem::size_of::<f64>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let double_size = core::mem::size_of::<f64>();

        // Deserialize the xBias, yBias, zBias coordinates
        let mut x_bias_bytes = [0u8; 8];
        x_bias_bytes.copy_from_slice(&buffer[offset..offset + double_size]);
        let x_bias = f64::from_le_bytes(x_bias_bytes);
        offset += double_size;

        let mut y_bias_bytes = [0u8; 8];
        y_bias_bytes.copy_from_slice(&buffer[offset..offset + double_size]);
        let y_bias = f64::from_le_bytes(y_bias_bytes);
        offset += double_size;

        let mut z_bias_bytes = [0u8; 8];
        z_bias_bytes.copy_from_slice(&buffer[offset..offset + double_size]);
        let z_bias = f64::from_le_bytes(z_bias_bytes);

        Ok(Self {
            x_bias,
            y_bias,
            z_bias,
        })
    }
}
