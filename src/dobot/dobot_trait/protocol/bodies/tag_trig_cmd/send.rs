use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
use crate::dobot::dobot_trait::protocol::Body;
use core::convert::TryFrom;

/// Represents the Trigger Mode.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerMode {
    Level = 0x00,
    Ad = 0x01,
}

impl TryFrom<u8> for TriggerMode {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `TriggerMode` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TriggerMode::Level),
            0x01 => Ok(TriggerMode::Ad),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents the Trigger Condition.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerCondition {
    LevelEqualOrAdLess = 0x00,
    LevelUnequalOrAdLessEqual = 0x01,
    AdGreaterEqual = 0x02,
    AdGreater = 0x03,
}

impl TryFrom<u8> for TriggerCondition {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `TriggerCondition` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TriggerCondition::LevelEqualOrAdLess),
            0x01 => Ok(TriggerCondition::LevelUnequalOrAdLessEqual),
            0x02 => Ok(TriggerCondition::AdGreaterEqual),
            0x03 => Ok(TriggerCondition::AdGreater),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents a TRIG (Trigger) command.
/// This struct corresponds to the Python `tagTRIGCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagTRIGCmd {
    pub address: u8,
    pub mode: TriggerMode,
    pub condition: TriggerCondition,
    pub threshold: u16,
}

impl Body for TagTRIGCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of three `u8`s (1 byte each) and one `u16` (2 bytes),
    /// totaling 1 + 1 + 1 + 2 = 5 bytes.
    fn size(&self) -> usize {
        (3 * core::mem::size_of::<u8>()) + core::mem::size_of::<u16>()
    }

    /// Packs the `TagTRIGCmd` struct into a byte sequence.
    /// It serializes the `address`, `mode`, `condition`, and `threshold` values
    /// into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Serialize the address
        buffer[offset..offset + u8_size].copy_from_slice(&self.address.to_le_bytes());
        offset += u8_size;

        // Serialize the mode enum value as a u8
        buffer[offset..offset + u8_size].copy_from_slice(&(self.mode as u8).to_le_bytes());
        offset += u8_size;

        // Serialize the condition enum value as a u8
        buffer[offset..offset + u8_size].copy_from_slice(&(self.condition as u8).to_le_bytes());
        offset += u8_size;
        
        // Serialize the threshold
        buffer[offset..offset + u16_size].copy_from_slice(&self.threshold.to_le_bytes());
        
        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagTRIGCmd` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = (3 * core::mem::size_of::<u8>()) + core::mem::size_of::<u16>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }
        
        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();
        let u16_size = core::mem::size_of::<u16>();

        // Deserialize the address
        let mut address_bytes = [0u8; 1];
        address_bytes.copy_from_slice(&buffer[offset..offset + u8_size]);
        let address = u8::from_le_bytes(address_bytes);
        offset += u8_size;

        // Deserialize the mode `u8` and convert to `TriggerMode` enum
        let mode = TriggerMode::try_from(buffer[offset])?;
        offset += u8_size;

        // Deserialize the condition `u8` and convert to `TriggerCondition` enum
        let condition = TriggerCondition::try_from(buffer[offset])?;
        offset += u8_size;
        
        // Deserialize the threshold
        let mut threshold_bytes = [0u8; 2];
        threshold_bytes.copy_from_slice(&buffer[offset..offset + u16_size]);
        let threshold = u16::from_le_bytes(threshold_bytes);

        Ok(Self { address, mode, condition, threshold })
    }
}
