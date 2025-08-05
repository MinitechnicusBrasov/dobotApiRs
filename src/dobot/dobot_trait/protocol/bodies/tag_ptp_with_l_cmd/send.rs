use crate::dobot::dobot_trait::protocol::{
    Body, bodies::tag_ptp_cmd::PTPMode, protocol_error::ProtocolError,
};
use core::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPWithLCmd {
    pub ptp_mode: PTPMode,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub l: f32,
}

impl<'a> Body<'a> for TagPTPWithLCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and five `f32`s (4 bytes each),
    /// totaling 1 + (5 * 4) = 21 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + (5 * core::mem::size_of::<f32>())
    }

    /// Packs the `TagPTPWithLCmd` struct into a byte sequence.
    /// It serializes the `ptp_mode` enum value and the five `f32` coordinates
    /// into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the enum value as a u8
        buffer[0] = self.ptp_mode as u8;
        offset += core::mem::size_of::<u8>();

        // Serialize the five `f32` coordinates
        buffer[offset..offset + float_size].copy_from_slice(&self.x.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.y.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.z.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.r.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.l.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPWithLCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + (5 * core::mem::size_of::<f32>());
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the `u8` and convert to `PTPMode` enum
        let ptp_mode = PTPMode::try_from(buffer[0])?;
        offset += core::mem::size_of::<u8>();

        // Deserialize the five `f32` coordinates
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
        offset += float_size;

        let mut l_bytes = [0u8; 4];
        l_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let l = f32::from_le_bytes(l_bytes);

        Ok(Self {
            ptp_mode,
            x,
            y,
            z,
            r,
            l,
        })
    }
}
