use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};
use core::convert::TryFrom;

/// Represents the JOG command mode.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum JogMode {
    Coordinate = 0,
    Joint = 1,
}

impl TryFrom<u8> for JogMode {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `JogMode`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JogMode::Coordinate),
            1 => Ok(JogMode::Joint),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents the JOG command type with more descriptive names.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum JogCmd {
    Idle = 0,   // Void
    ApDown = 1, // X+/Joint1+
    AnDown = 2, // X-/Joint1-
    BpDown = 3, // Y+/Joint2+
    BnDown = 4, // Y-/Joint2-
    CpDown = 5, // Z+/Joint3+
    CnDown = 6, // Z-/Joint3-
    DpDown = 7, // R+/Joint4+
    DnDown = 8, // R-/Joint4-
}

impl TryFrom<u8> for JogCmd {
    type Error = ProtocolError;

    /// Attempts to convert a `u8` into a `JogCmd`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JogCmd::Idle),
            1 => Ok(JogCmd::ApDown),
            2 => Ok(JogCmd::AnDown),
            3 => Ok(JogCmd::BpDown),
            4 => Ok(JogCmd::BnDown),
            5 => Ok(JogCmd::CpDown),
            6 => Ok(JogCmd::CnDown),
            7 => Ok(JogCmd::DpDown),
            8 => Ok(JogCmd::DnDown),
            _ => Err(ProtocolError::InvalidEnumValue),
        }
    }
}

/// Represents a JOG command with mode and command type.
/// This struct corresponds to the Python `tagJOGCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagJOGCmd {
    pub is_joint: JogMode,
    pub cmd: JogCmd,
}

impl Body for TagJOGCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of two unsigned 8-bit integers (`u8`), each 1 byte.
    fn size(&self) -> usize {
        2 * core::mem::size_of::<u8>()
    }

    /// Packs the `TagJOGCmd` struct into a byte sequence.
    /// It serializes the `is_joint` and `cmd` enum values as `u8`s.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the enum values as u8s
        buffer[0] = self.is_joint as u8;
        buffer[1] = self.cmd as u8;

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagJOGCmd` struct.
    /// It deserializes the `u8`s and converts them into `JogMode` and `JogCmd`
    /// enums, handling any invalid values.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 2;
        if buffer.len() != size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the u8s and convert to enums
        let is_joint = JogMode::try_from(buffer[0])?;
        let cmd = JogCmd::try_from(buffer[1])?;

        Ok(Self { is_joint, cmd })
    }
}
