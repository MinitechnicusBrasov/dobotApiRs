use super::protocol_error::ProtocolError;

pub trait Body<'a>: Sized {
    /// Returns the number of bytes this body will serialize into.
    fn size(&self) -> usize;

    /// Serializes the struct's data into the provided buffer.
    /// Returns the number of bytes written, or an error if the buffer is too small.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError>;

    /// Deserializes the struct's data from a byte slice.
    fn deserialize(buffer: &'a [u8]) -> Result<Self, ProtocolError>;
}
