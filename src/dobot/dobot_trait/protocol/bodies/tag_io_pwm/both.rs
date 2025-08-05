use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a PWM output command.
/// This struct corresponds to the Python `tagIOPWM` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagIOPWM {
    pub address: u8,
    pub frequency: f32,
    pub duty_cycle: f32,
}

impl<'a> Body<'a> for TagIOPWM {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and two `f32`s (4 bytes each),
    /// totaling 1 + 4 + 4 = 9 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + (2 * core::mem::size_of::<f32>())
    }

    /// Packs the `TagIOPWM` struct into a byte sequence.
    /// It serializes the `address` (`u8`), `frequency` (`f32`), and
    /// `duty_cycle` (`f32`) into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let f32_size = core::mem::size_of::<f32>();

        // Serialize the address
        buffer[offset..offset + u8_size].copy_from_slice(&self.address.to_le_bytes());
        offset += u8_size;

        // Serialize the frequency
        buffer[offset..offset + f32_size].copy_from_slice(&self.frequency.to_le_bytes());
        offset += f32_size;

        // Serialize the duty cycle
        buffer[offset..offset + f32_size].copy_from_slice(&self.duty_cycle.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagIOPWM` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + (2 * core::mem::size_of::<f32>());
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let f32_size = core::mem::size_of::<f32>();

        // Deserialize the address
        let mut address_bytes = [0u8; 1];
        address_bytes.copy_from_slice(&buffer[offset..offset + u8_size]);
        let address = u8::from_le_bytes(address_bytes);
        offset += u8_size;

        // Deserialize the frequency
        let mut frequency_bytes = [0u8; 4];
        frequency_bytes.copy_from_slice(&buffer[offset..offset + f32_size]);
        let frequency = f32::from_le_bytes(frequency_bytes);
        offset += f32_size;

        // Deserialize the duty cycle
        let mut duty_cycle_bytes = [0u8; 4];
        duty_cycle_bytes.copy_from_slice(&buffer[offset..offset + f32_size]);
        let duty_cycle = f32::from_le_bytes(duty_cycle_bytes);

        Ok(Self {
            address,
            frequency,
            duty_cycle,
        })
    }
}
