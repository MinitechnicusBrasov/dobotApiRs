use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a WiFi IP address configuration.
/// This struct corresponds to the Python `tagWIFIIPAddress` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagWIFIIPAddress {
    pub dhcp: bool,
    // Replaced Ipv4Addr with a fixed-size array of 4 bytes
    pub addr: [u8; 4],
}

impl<'a> Body<'a> for TagWIFIIPAddress {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of one `u8` (1 byte) and a 4-byte array,
    /// totaling 1 + 4 = 5 bytes.
    fn size(&self) -> usize {
        core::mem::size_of::<u8>() + self.addr.len()
    }

    /// Packs the `TagWIFIIPAddress` struct into a byte sequence.
    /// It serializes the `dhcp` (`bool` as `u8`) and the `addr` (4-byte array)
    /// into the buffer.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let u8_size = core::mem::size_of::<u8>();

        // Serialize the boolean `dhcp` as a u8 (1 or 0)
        let dhcp_byte = if self.dhcp { 1u8 } else { 0u8 };
        buffer[offset..offset + u8_size].copy_from_slice(&dhcp_byte.to_le_bytes());
        offset += u8_size;

        // Serialize the 4-byte IP address directly from the array
        let addr_size = self.addr.len();
        buffer[offset..offset + addr_size].copy_from_slice(&self.addr);

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagWIFIIPAddress` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = core::mem::size_of::<u8>() + 4;
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let u8_size = core::mem::size_of::<u8>();

        // Deserialize the `dhcp` u8 and convert to bool
        let dhcp = buffer[0] == 1;

        // Deserialize the 4-byte address into a new array
        let mut addr_bytes = [0u8; 4];
        addr_bytes.copy_from_slice(&buffer[u8_size..u8_size + 4]);

        Ok(Self {
            dhcp,
            addr: addr_bytes,
        })
    }
}
