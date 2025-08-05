use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::{Body, ProtocolError},
};

#[derive(PartialEq, Debug)]
pub struct GeneralResponse<'a> {
    pub params: &'a [u8],
}

impl<'a> Body<'a> for GeneralResponse<'a> {
    fn serialize(&self, _buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        // Not used for response body
        Err(ProtocolError::InvalidOperation)
    }

    fn deserialize(buffer: &'a [u8]) -> Result<Self, ProtocolError> {
        Ok(Self { params: buffer })
    }

    fn size(&self) -> usize {
        self.params.len()
    }
}
