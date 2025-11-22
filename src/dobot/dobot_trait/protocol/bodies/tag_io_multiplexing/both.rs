use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};
use core::convert::TryFrom;

/// Represents the I/O Function mode.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum IOFunction {
    Dummy = 0x00,
    Do = 0x01,
    Pwm = 0x02,
    Di = 0x03,
    Adc = 0x04,
    Dipu = 0x05,
    Dipd = 0x06,
}

impl TryFrom<u8> for IOFunction {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into an `IOFunction`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(IOFunction::Dummy),
            0x01 => Ok(IOFunction::Do),
            0x02 => Ok(IOFunction::Pwm),
            0x03 => Ok(IOFunction::Di),
            0x04 => Ok(IOFunction::Adc),
            0x05 => Ok(IOFunction::Dipu),
            0x06 => Ok(IOFunction::Dipd),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents an I/O multiplexing command.
/// This struct corresponds to the Python `tagIOMultiplexing` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagIOMultiplexing {
    pub address: u8,
    pub multiplex: IOFunction,
}

impl<'a> Body<'a> for TagIOMultiplexing {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of two `u8`s, each 1 byte, totaling 2 bytes.
    fn size(&self) -> usize {
        2 * core::mem::size_of::<u8>()
    }

    /// Packs the `TagIOMultiplexing` struct into a byte sequence.
    /// It serializes the `address` and `multiplex` enum value into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Serialize the address
        buffer[..u8_size].copy_from_slice(&self.address.to_le_bytes());

        // Serialize the multiplex enum value as a u8
        buffer[u8_size..u8_size + 1].copy_from_slice(&(self.multiplex as u8).to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagIOMultiplexing` struct.
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

        // Deserialize the multiplex `u8` and convert to `IOFunction` enum
        let multiplex = IOFunction::try_from(buffer[u8_size])?;

        Ok(Self { address, multiplex })
    }
}
