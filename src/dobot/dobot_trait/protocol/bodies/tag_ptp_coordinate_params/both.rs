use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents PTP coordinate parameters with velocity and acceleration.
/// This struct corresponds to the Python `tagPTPCoordinateParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPCoordinateParams {
    /// Coordinate velocity for xyz.
    pub xyz_velocity: f32,
    /// Coordinate velocity for r.
    pub r_velocity: f32,
    /// Coordinate acceleration for xyz.
    pub xyz_acceleration: f32,
    /// Coordinate acceleration for r.
    pub r_acceleration: f32,
}

impl<'a> Body<'a> for TagPTPCoordinateParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 4 floats (`f32`), each 4 bytes, totaling 16 bytes.
    fn size(&self) -> usize {
        4 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagPTPCoordinateParams` struct into a byte sequence.
    /// It serializes the four `f32` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the velocity and acceleration values
        buffer[offset..offset + float_size].copy_from_slice(&self.xyz_velocity.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.r_velocity.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.xyz_acceleration.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.r_acceleration.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPCoordinateParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 4 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the velocity and acceleration values
        let mut xyz_velocity_bytes = [0u8; 4];
        xyz_velocity_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let xyz_velocity = f32::from_le_bytes(xyz_velocity_bytes);
        offset += float_size;

        let mut r_velocity_bytes = [0u8; 4];
        r_velocity_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let r_velocity = f32::from_le_bytes(r_velocity_bytes);
        offset += float_size;

        let mut xyz_acceleration_bytes = [0u8; 4];
        xyz_acceleration_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let xyz_acceleration = f32::from_le_bytes(xyz_acceleration_bytes);
        offset += float_size;

        let mut r_acceleration_bytes = [0u8; 4];
        r_acceleration_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let r_acceleration = f32::from_le_bytes(r_acceleration_bytes);

        Ok(Self {
            xyz_velocity,
            r_velocity,
            xyz_acceleration,
            r_acceleration,
        })
    }
}
