use crate::dobot::dobot_trait::protocol::{Body, ProtocolError};

#[derive(PartialEq, Debug)]
pub struct GeneralRequest<'a> {
    pub params: &'a [u8],
}

impl<'a> Body<'a> for GeneralRequest<'a> {
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        if buffer.len() < self.params.len() {
            return Err(ProtocolError::BufferTooSmall);
        }
        buffer[..self.params.len()].copy_from_slice(self.params);
        Ok(self.params.len())
    }

    fn deserialize(_buffer: &'a [u8]) -> Result<Self, ProtocolError> {
        // Not used for request body
        Err(ProtocolError::InvalidOperation)
    }

    fn size(&self) -> usize {
        self.params.len()
    }
}
