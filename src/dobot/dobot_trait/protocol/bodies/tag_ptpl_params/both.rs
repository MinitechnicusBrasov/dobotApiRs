use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

/// Represents PTP linear parameters with velocity and acceleration.
/// This struct corresponds to the Python `tagPTPLParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPLParams {
    /// The linear velocity.
    pub velocity: f32,
    /// The linear acceleration.
    pub acceleration: f32,
}

impl Body for TagPTPLParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 2 floats (`f32`), each 4 bytes, totaling 8 bytes.
    fn size(&self) -> usize {
        2 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagPTPLParams` struct into a byte sequence.
    /// It serializes the two `f32` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the velocity
        buffer[offset..offset + float_size].copy_from_slice(&self.velocity.to_le_bytes());
        offset += float_size;

        // Serialize the acceleration
        buffer[offset..offset + float_size].copy_from_slice(&self.acceleration.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPLParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 2 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the velocity
        let mut velocity_bytes = [0u8; 4];
        velocity_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let velocity = f32::from_le_bytes(velocity_bytes);
        offset += float_size;

        // Deserialize the acceleration
        let mut acceleration_bytes = [0u8; 4];
        acceleration_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let acceleration = f32::from_le_bytes(acceleration_bytes);

        Ok(Self {
            velocity,
            acceleration,
        })
    }
}
