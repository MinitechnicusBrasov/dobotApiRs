use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

#[derive(Debug, PartialEq, Clone)]
pub struct TagHomeParams {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

impl<'a> Body<'a> for TagHomeParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of 4 floats (`f32`), each 4 bytes.
    fn size(&self) -> usize {
        4 * core::mem::size_of::<f32>()
    }

    /// Packs the `TagHomeParams` struct into a byte sequence.
    /// It serializes the 4 float values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the x, y, z, r coordinates
        buffer[offset..offset + float_size].copy_from_slice(&self.x.to_le_bytes());
        offset += float_size;
        buffer[offset..offset + float_size].copy_from_slice(&self.y.to_le_bytes());
        offset += float_size;
        buffer[offset..offset + float_size].copy_from_slice(&self.z.to_le_bytes());
        offset += float_size;
        buffer[offset..offset + float_size].copy_from_slice(&self.r.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagHomeParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 4 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the x, y, z, r coordinates
        let mut x_bytes = [0u8; 4];
        x_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let x = f32::from_le_bytes(x_bytes);
        offset += float_size;

        let mut y_bytes = [0u8; 4];
        y_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let y = f32::from_le_bytes(y_bytes);
        offset += float_size;

        let mut z_bytes = [0u8; 4];
        z_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let z = f32::from_le_bytes(z_bytes);
        offset += float_size;

        let mut r_bytes = [0u8; 4];
        r_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let r = f32::from_le_bytes(r_bytes);

        Ok(Self { x, y, z, r })
    }
}
