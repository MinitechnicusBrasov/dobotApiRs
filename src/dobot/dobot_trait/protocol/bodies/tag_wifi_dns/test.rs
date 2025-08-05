#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_wifi_dns::TagWIFIDNS, protocol_error::ProtocolError,
    };
    // No longer need to import Ipv4Addr
    // use std::net::Ipv4Addr;

    /// Test case for successful serialization and deserialization of TagWIFIDNS.
    #[test]
    fn test_tag_wifi_dns_pack_unpack_success() {
        // Create an original struct instance with sample values
        // Now directly use a [u8; 4] array for the address
        let original_cmd = TagWIFIDNS { addr: [8, 8, 8, 8] };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 4];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 bytes for the array)
        assert_eq!(size, 4);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagWIFIDNS::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_wifi_dns_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 3];
        let result = TagWIFIDNS::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
