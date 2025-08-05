use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};
use core::convert::TryFrom;

/// Represents the PTP command mode.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PTPMode {
    JumpXyz = 0x00,
    MovjXyz = 0x01,
    MovlXyz = 0x02,
    JumpAngle = 0x03,
    MovjAngle = 0x04,
    MovlAngle = 0x05,
    MovjInc = 0x06,
    MovlInc = 0x07,
    MovjXyzInc = 0x08,
    JumpMovlXyz = 0x09,
}

impl TryFrom<u8> for PTPMode {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `PTPMode`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PTPMode::JumpXyz),
            0x01 => Ok(PTPMode::MovjXyz),
            0x02 => Ok(PTPMode::MovlXyz),
            0x03 => Ok(PTPMode::JumpAngle),
            0x04 => Ok(PTPMode::MovjAngle),
            0x05 => Ok(PTPMode::MovlAngle),
            0x06 => Ok(PTPMode::MovjInc),
            0x07 => Ok(PTPMode::MovlInc),
            0x08 => Ok(PTPMode::MovjXyzInc),
            0x09 => Ok(PTPMode::JumpMovlXyz),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents a PTP command with mode and coordinate data.
/// This struct corresponds to the Python `tagPTPCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPTPCmd {
    pub ptp_mode: PTPMode,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

impl<'a> Body<'a> for TagPTPCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and four `f32`s (4 bytes each),
    /// totaling 1 + (4 * 4) = 17 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + (4 * core::mem::size_of::<f32>())
    }

    /// Packs the `TagPTPCmd` struct into a byte sequence.
    /// It serializes the `ptp_mode` enum value and the four `f32` coordinates
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

        // Serialize the four `f32` coordinates
        buffer[offset..offset + float_size].copy_from_slice(&self.x.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.y.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.z.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.r.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPTPCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + (4 * core::mem::size_of::<f32>());
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the `u8` and convert to `PTPMode` enum
        let ptp_mode = PTPMode::try_from(buffer[0])?;
        offset += core::mem::size_of::<u8>();

        // Deserialize the four `f32` coordinates
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

        Ok(Self {
            ptp_mode,
            x,
            y,
            z,
            r,
        })
    }
}
