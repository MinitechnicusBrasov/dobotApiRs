use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents auto-leveling parameters with a boolean flag and an accuracy value.
/// This struct corresponds to the Python `tagAutoLevelingParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagAutoLevelingParams {
    pub is_auto_leveling: bool,
    pub accuracy: f32,
}

impl<'a> Body<'a> for TagAutoLevelingParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one unsigned 8-bit integer (`u8`) and one float (`f32`),
    /// totaling 1 + 4 = 5 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + core::mem::size_of::<f32>()
    }

    /// Packs the `TagAutoLevelingParams` struct into a byte sequence.
    /// It serializes the boolean as a `u8` and the accuracy as an `f32`
    /// into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;

        // Serialize the boolean as a u8 (0 or 1)
        buffer[offset] = self.is_auto_leveling as u8;
        offset += 1;

        // Serialize the f32 accuracy value
        let float_size = core::mem::size_of::<f32>();
        buffer[offset..offset + float_size].copy_from_slice(&self.accuracy.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagAutoLevelingParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &'a [u8]) -> Result<Self, ProtocolError> {
        let size = 5;
        if buffer.len() != size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;

        // Deserialize the boolean from the first byte
        let is_auto_leveling = buffer[offset] == 1;
        offset += 1;

        // Deserialize the f32 from the remaining bytes
        let float_size = core::mem::size_of::<f32>();
        let mut accuracy_bytes = [0u8; 4];
        accuracy_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let accuracy = f32::from_le_bytes(accuracy_bytes);

        Ok(Self {
            is_auto_leveling,
            accuracy,
        })
    }
}
