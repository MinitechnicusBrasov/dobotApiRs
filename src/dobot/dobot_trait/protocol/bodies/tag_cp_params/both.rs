use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};
use core::convert::TryFrom;

/// Represents the RealTimeTrack parameter as a boolean-like enum.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum RealTimeTrack {
    NonRealTime = 0x00,
    RealTime = 0x01,
}

impl TryFrom<u8> for RealTimeTrack {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `RealTimeTrack` enum.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RealTimeTrack::NonRealTime),
            0x01 => Ok(RealTimeTrack::RealTime),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents CP (Continuous Path) parameters.
/// This struct corresponds to the Python `tagCPParams` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagCPParams {
    /// The planned acceleration.
    pub plan_acc: f32,
    /// The acceleration at the junction.
    pub junction_acc: f32,
    /// The acceleration or period.
    pub acceleratio_or_period: f32,
    /// The real-time track parameter.
    pub real_time_track: RealTimeTrack,
}

impl Body for TagCPParams {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of three `f32`s (4 bytes each) and one `u8` (1 byte),
    /// totaling (3 * 4) + 1 = 13 bytes.
    fn size(&self) -> usize {
        (3 * core::mem::size_of::<f32>()) + core::mem::size_of::<u8>()
    }

    /// Packs the `TagCPParams` struct into a byte sequence.
    /// It serializes the three `f32` values and the `u8` enum value into the buffer
    /// using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();
        let u8_size = core::mem::size_of::<u8>();

        // Serialize the three `f32` values
        buffer[offset..offset + float_size].copy_from_slice(&self.plan_acc.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.junction_acc.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size]
            .copy_from_slice(&self.acceleratio_or_period.to_le_bytes());
        offset += float_size;

        // Serialize the enum value as a u8
        buffer[offset..offset + u8_size]
            .copy_from_slice(&(self.real_time_track as u8).to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagCPParams` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = (3 * core::mem::size_of::<f32>()) + core::mem::size_of::<u8>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();
        let u8_size = core::mem::size_of::<u8>();

        // Deserialize the three `f32` values
        let mut plan_acc_bytes = [0u8; 4];
        plan_acc_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let plan_acc = f32::from_le_bytes(plan_acc_bytes);
        offset += float_size;

        let mut junction_acc_bytes = [0u8; 4];
        junction_acc_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let junction_acc = f32::from_le_bytes(junction_acc_bytes);
        offset += float_size;

        let mut acceleratio_or_period_bytes = [0u8; 4];
        acceleratio_or_period_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let acceleratio_or_period = f32::from_le_bytes(acceleratio_or_period_bytes);
        offset += float_size;

        // Deserialize the `u8` and convert to `RealTimeTrack` enum
        let mut real_time_track_bytes = [0u8; 1];
        real_time_track_bytes.copy_from_slice(&buffer[offset..offset + u8_size]);
        let real_time_track = RealTimeTrack::try_from(u8::from_le_bytes(real_time_track_bytes))?;

        Ok(Self {
            plan_acc,
            junction_acc,
            acceleratio_or_period,
            real_time_track,
        })
    }
}
