use crate::dobot::dobot_trait::protocol::{
    Body, bodies::level::Level, protocol_error::ProtocolError,
};

/// Represents a digital output command.
/// This struct corresponds to the Python `tagIODO` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagIODO {
    pub address: u8,
    pub level: Level,
}

impl Body for TagIODO {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of two `u8`s, each 1 byte, totaling 2 bytes.
    fn size(&self) -> usize {
        2 * core::mem::size_of::<u8>()
    }

    /// Packs the `TagIODO` struct into a byte sequence.
    /// It serializes the `address` and `level` enum value into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Serialize the address
        buffer[..u8_size].copy_from_slice(&self.address.to_le_bytes());

        // Serialize the level enum value as a u8
        buffer[u8_size..].copy_from_slice(&(self.level as u8).to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagIODO` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 2 * core::mem::size_of::<u8>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Deserialize the address
        let mut address_bytes = [0u8; 1];
        address_bytes.copy_from_slice(&buffer[..u8_size]);
        let address = u8::from_le_bytes(address_bytes);

        let level = Level::try_from(buffer[u8_size])?;

        Ok(Self { address, level })
    }
}
