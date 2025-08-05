mod test;
use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a protocol message body with no content.
/// It is a zero-sized type and serves as a placeholder for commands
/// that do not require a payload.
#[derive(Debug, PartialEq, Clone)]
pub struct EmptyBody {}

impl Body for EmptyBody {
    /// Returns the size of the serialized body in bytes.
    /// Since the body is empty, the size is always 0.
    fn size(&self) -> usize {
        0
    }

    /// Packs the `EmptyBody` struct into a byte sequence.
    /// This method performs no actions and simply returns `Ok(0)`.
    fn serialize(&self, _buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        Ok(0)
    }

    /// Unpacks a byte sequence into an `EmptyBody` struct.
    /// This method performs no actions and simply returns an `EmptyBody`.
    fn deserialize(_buffer: &[u8]) -> Result<Self, ProtocolError> {
        Ok(EmptyBody {})
    }
}
