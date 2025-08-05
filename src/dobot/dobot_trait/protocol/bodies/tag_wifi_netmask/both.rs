use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a WiFi network mask configuration.
/// This struct corresponds to the Python `tagWIFINetmask` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagWIFINetmask {
    // Replaced Ipv4Addr with a fixed-size array of 4 bytes
    pub addr: [u8; 4],
}

impl Body for TagWIFINetmask {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one 4-byte array.
    fn size(&self) -> usize {
        // The size is simply the length of the array
        self.addr.len()
    }

    /// Packs the `TagWIFINetmask` struct into a byte sequence.
    /// It serializes the `addr` (4-byte array) into the buffer.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Serialize the 4-byte IP address directly from the array
        buffer[..size].copy_from_slice(&self.addr);

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagWIFINetmask` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        // The fixed size for an IPv4 address is 4 bytes
        let size = 4;
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        // Deserialize the 4-byte address into a new array
        let mut addr_bytes = [0u8; 4];
        addr_bytes.copy_from_slice(&buffer[..size]);

        Ok(Self { addr: addr_bytes })
    }
}
