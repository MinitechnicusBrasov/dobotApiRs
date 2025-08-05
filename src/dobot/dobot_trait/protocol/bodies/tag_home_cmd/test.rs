#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_home_cmd::TagHomeCmd, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagHomeCmd.
    #[test]
    fn test_tag_home_cmd_pack_unpack_success() {
        // Create an original struct instance with a sample u32 value
        let original_cmd = TagHomeCmd {
            reserved: 0xDEADBEEF,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 4];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 u32 * 4 bytes/u32 = 4 bytes)
        assert_eq!(size, 4);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagHomeCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_home_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 3];
        let result = TagHomeCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
