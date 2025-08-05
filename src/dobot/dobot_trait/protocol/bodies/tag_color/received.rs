use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents an RGB color using 8-bit integer values.
/// This struct corresponds to the Python `tagColor` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl<'a> Body<'a> for TagColor {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of three `u8`s, each 1 byte, totaling 3 bytes.
    fn size(&self) -> usize {
        3 * core::mem::size_of::<u8>()
    }

    /// Packs the `TagColor` struct into a byte sequence.
    /// It serializes the `red`, `green`, and `blue` (`u8`) values into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();

        // Serialize the red component
        buffer[offset..offset + u8_size].copy_from_slice(&self.red.to_le_bytes());
        offset += u8_size;

        // Serialize the green component
        buffer[offset..offset + u8_size].copy_from_slice(&self.green.to_le_bytes());
        offset += u8_size;

        // Serialize the blue component
        buffer[offset..offset + u8_size].copy_from_slice(&self.blue.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagColor` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 3 * core::mem::size_of::<u8>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Deserialize the red component
        let mut red_bytes = [0u8; 1];
        red_bytes.copy_from_slice(&buffer[..u8_size]);
        let red = u8::from_le_bytes(red_bytes);

        // Deserialize the green component
        let mut green_bytes = [0u8; 1];
        green_bytes.copy_from_slice(&buffer[u8_size..u8_size + u8_size]);
        let green = u8::from_le_bytes(green_bytes);

        // Deserialize the blue component
        let mut blue_bytes = [0u8; 1];
        blue_bytes.copy_from_slice(&buffer[2 * u8_size..2 * u8_size + u8_size]);
        let blue = u8::from_le_bytes(blue_bytes);

        Ok(Self { red, green, blue })
    }
}
