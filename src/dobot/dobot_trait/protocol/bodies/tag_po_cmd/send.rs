use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a PO (Pulse Output) command.
/// This struct corresponds to the Python `tagPOCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagPOCmd {
    /// The ratio, an 8-bit unsigned integer.
    pub ratio: u8,
    /// The address, a 16-bit unsigned integer.
    pub address: u16,
    /// The level, an 8-bit unsigned integer.
    pub level: u8,
}

impl<'a> Body<'a> for TagPOCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte), one `u16` (2 bytes), and one `u8` (1 byte),
    /// totaling 1 + 2 + 1 = 4 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + core::mem::size_of::<u16>() + core::mem::size_of::<u8>()
    }

    /// Packs the `TagPOCmd` struct into a byte sequence.
    /// It serializes the `ratio`, `address`, and `level` values into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Serialize the ratio
        buffer[offset..offset + u8_size].copy_from_slice(&self.ratio.to_le_bytes());
        offset += u8_size;

        // Serialize the address
        buffer[offset..offset + u16_size].copy_from_slice(&self.address.to_le_bytes());
        offset += u16_size;

        // Serialize the level
        buffer[offset..offset + u8_size].copy_from_slice(&self.level.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagPOCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size =
            core::mem::size_of::<u8>() + core::mem::size_of::<u16>() + core::mem::size_of::<u8>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Deserialize the ratio
        let mut ratio_bytes = [0u8; 1];
        ratio_bytes.copy_from_slice(&buffer[offset..offset + u8_size]);
        let ratio = u8::from_le_bytes(ratio_bytes);
        offset += u8_size;

        // Deserialize the address
        let mut address_bytes = [0u8; 2];
        address_bytes.copy_from_slice(&buffer[offset..offset + u16_size]);
        let address = u16::from_le_bytes(address_bytes);
        offset += u16_size;

        // Deserialize the level
        let mut level_bytes = [0u8; 1];
        level_bytes.copy_from_slice(&buffer[offset..offset + u8_size]);
        let level = u8::from_le_bytes(level_bytes);

        Ok(Self {
            ratio,
            address,
            level,
        })
    }
}
