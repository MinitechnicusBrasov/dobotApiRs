use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents an Analog-to-Digital Converter (ADC) input status.
/// This struct corresponds to the Python `IOADC` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagIOADC {
    pub address: u8,
    pub value: u16,
}

impl Body for TagIOADC {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and one `u16` (2 bytes),
    /// totaling 1 + 2 = 3 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + core::mem::size_of::<u16>()
    }

    /// Packs the `TagIOADC` struct into a byte sequence.
    /// It serializes the `address` (`u8`) and `value` (`u16`) into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Serialize the address
        buffer[..u8_size].copy_from_slice(&self.address.to_le_bytes());

        // Serialize the value
        buffer[u8_size..u8_size + u16_size].copy_from_slice(&self.value.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagIOADC` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + core::mem::size_of::<u16>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Deserialize the address
        let mut address_bytes = [0u8; 1];
        address_bytes.copy_from_slice(&buffer[..u8_size]);
        let address = u8::from_le_bytes(address_bytes);

        // Deserialize the value
        let mut value_bytes = [0u8; 2];
        value_bytes.copy_from_slice(&buffer[u8_size..u8_size + u16_size]);
        let value = u16::from_le_bytes(value_bytes);

        Ok(Self { address, value })
    }
}
