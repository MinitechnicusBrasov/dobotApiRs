#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_wifi_gateway::TagWIFIGateway, protocol_error::ProtocolError,
    };
    // The Ipv4Addr import is no longer needed since we are using a fixed-size array directly.

    /// Test case for successful serialization and deserialization of TagWIFIGateway.
    #[test]
    fn test_tag_wifi_gateway_pack_unpack_success() {
        // Create an original struct instance with sample values
        // Now directly initialize with a fixed-size array
        let original_cmd = TagWIFIGateway {
            addr: [192, 168, 1, 1],
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 4];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 bytes for the array)
        assert_eq!(size, 4);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagWIFIGateway::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_wifi_gateway_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 3];
        let result = TagWIFIGateway::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
