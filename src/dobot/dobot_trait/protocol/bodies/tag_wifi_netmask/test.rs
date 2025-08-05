#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_wifi_netmask::TagWIFINetmask, protocol_error::ProtocolError,
    };
    // The Ipv4Addr import is no longer needed.

    /// Test case for successful serialization and deserialization of TagWIFINetmask.
    #[test]
    fn test_tag_wifi_netmask_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagWIFINetmask {
            addr: [255, 255, 255, 0],
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 4];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 Ipv4Addr = 4 bytes)
        assert_eq!(size, 4);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagWIFINetmask::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_wifi_netmask_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 3];
        let result = TagWIFINetmask::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
