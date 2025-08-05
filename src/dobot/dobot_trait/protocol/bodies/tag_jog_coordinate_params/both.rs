use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents JOG coordinate parameters with velocity and acceleration for a 4-axis system (x, y, z, r).
/// This struct corresponds to the Python `tagJOGCoordinateParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagJOGCoordinateParams {
    /// Coordinate velocity for 4 axes.
    pub velocity: [f32; 4],
    /// Coordinate acceleration for 4 axes.
    pub acceleration: [f32; 4],
}

impl Body for TagJOGCoordinateParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 8 floats (`f32`), each 4 bytes, totaling 32 bytes.
    fn size(&self) -> usize {
        8 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagJOGCoordinateParams` struct into a byte sequence.
    /// It serializes the 4 velocity floats and the 4 acceleration floats
    /// into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the 4 velocity values
        for &value in &self.velocity {
            buffer[offset..offset + float_size].copy_from_slice(&value.to_le_bytes());
            offset += float_size;
        }

        // Serialize the 4 acceleration values
        for &value in &self.acceleration {
            buffer[offset..offset + float_size].copy_from_slice(&value.to_le_bytes());
            offset += float_size;
        }

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagJOGCoordinateParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 8 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the 4 velocity values
        let mut velocity = [0.0; 4];
        for i in 0..4 {
            let mut value_bytes = [0u8; 4];
            value_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
            velocity[i] = f32::from_le_bytes(value_bytes);
            offset += float_size;
        }

        // Deserialize the 4 acceleration values
        let mut acceleration = [0.0; 4];
        for i in 0..4 {
            let mut value_bytes = [0u8; 4];
            value_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
            acceleration[i] = f32::from_le_bytes(value_bytes);
            offset += float_size;
        }

        Ok(Self {
            velocity,
            acceleration,
        })
    }
}
