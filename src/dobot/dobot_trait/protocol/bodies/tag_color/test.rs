#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_color::TagColor, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagColor.
    #[test]
    fn test_tag_color_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagColor {
            red: 255,
            green: 128,
            blue: 64,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 3];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 u8s = 3 bytes)
        assert_eq!(size, 3);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagColor::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_color_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 2];
        let result = TagColor::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
