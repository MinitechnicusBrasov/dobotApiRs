use crate::dobot::dobot_trait::protocol::Body;
use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
use core::convert::TryFrom;

/// Represents the CP (Continuous Path) command mode.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CPMode {
    Relative = 0x00,
    Absolute = 0x01,
}

impl TryFrom<u8> for CPMode {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `CPMode`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CPMode::Relative),
            0x01 => Ok(CPMode::Absolute),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents a CP command with mode and coordinate data, and an additional
/// parameter that can represent velocity or power.
/// This struct corresponds to the Python `tagCPCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagCPCmd {
    pub cp_mode: CPMode,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub velocity_or_power: f32,
}

impl<'a> Body<'a> for TagCPCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and four `f32`s (4 bytes each),
    /// totaling 1 + (4 * 4) = 17 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + (4 * core::mem::size_of::<f32>())
    }

    /// Packs the `TagCPCmd` struct into a byte sequence.
    /// It serializes the `cp_mode` enum value and the four `f32` values
    /// into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Serialize the enum value as a u8
        buffer[0] = self.cp_mode as u8;
        offset += core::mem::size_of::<u8>();

        // Serialize the four `f32` values
        buffer[offset..offset + float_size].copy_from_slice(&self.x.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.y.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.z.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.velocity_or_power.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagCPCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + (4 * core::mem::size_of::<f32>());
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        // Deserialize the `u8` and convert to `CPMode` enum
        let cp_mode = CPMode::try_from(buffer[0])?;
        offset += core::mem::size_of::<u8>();

        // Deserialize the four `f32` values
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

        let mut velocity_or_power_bytes = [0u8; 4];
        velocity_or_power_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let velocity_or_power = f32::from_le_bytes(velocity_or_power_bytes);

        Ok(Self {
            cp_mode,
            x,
            y,
            z,
            velocity_or_power,
        })
    }
}
