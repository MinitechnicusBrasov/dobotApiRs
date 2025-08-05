use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum EMotorIndex {
    Stepper1 = 0x00,
    Stepper2 = 0x01,
}

impl TryFrom<u8> for EMotorIndex {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into an `EMotorIndex` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(EMotorIndex::Stepper1),
            0x01 => Ok(EMotorIndex::Stepper2),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TagEMotor {
    pub address: EMotorIndex,
    pub ins_enabled: bool,
    pub speed: f64,
}

impl Body for TagEMotor {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of two `u8`s (1 byte each) and one `f64` (8 bytes),
    /// totaling 1 + 1 + 8 = 10 bytes.
    fn size(&self) -> usize {
        (2 * core::mem::size_of::<u8>()) + core::mem::size_of::<f64>()
    }

    /// Packs the `TagEMotor` struct into a byte sequence.
    /// It serializes the `address` (`EMotorIndex` as `u8`), `ins_enabled` (`bool` as `u8`),
    /// and `speed` (`f64`) into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let f64_size = core::mem::size_of::<f64>();

        // Serialize the address enum value as a u8
        buffer[offset..offset + u8_size].copy_from_slice(&(self.address as u8).to_le_bytes());
        offset += u8_size;

        // Serialize the boolean `ins_enabled` as a u8 (1 or 0)
        let ins_enabled_byte = if self.ins_enabled { 1u8 } else { 0u8 };
        buffer[offset..offset + u8_size].copy_from_slice(&ins_enabled_byte.to_le_bytes());
        offset += u8_size;

        // Serialize the speed
        buffer[offset..offset + f64_size].copy_from_slice(&self.speed.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagEMotor` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = (2 * core::mem::size_of::<u8>()) + core::mem::size_of::<f64>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let f64_size = core::mem::size_of::<f64>();

        // Deserialize the address `u8` and convert to `EMotorIndex` enum
        let address = EMotorIndex::try_from(buffer[offset])?;
        offset += u8_size;

        // Deserialize the `ins_enabled` u8 and convert to bool
        let ins_enabled = buffer[offset] == 1;
        offset += u8_size;

        // Deserialize the speed
        let mut speed_bytes = [0u8; 8];
        speed_bytes.copy_from_slice(&buffer[offset..offset + f64_size]);
        let speed = f64::from_le_bytes(speed_bytes);

        Ok(Self {
            address,
            ins_enabled,
            speed,
        })
    }
}
