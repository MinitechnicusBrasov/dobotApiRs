#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
    use crate::dobot::dobot_trait::protocol::Body;
    use crate::dobot::dobot_trait::protocol::bodies::tag_with_l::{TagWithLReturn};

    /// Test case for successful serialization and deserialization of TagWithLReturn.
    #[test]
    fn test_tag_with_l_return_pack_unpack_success() {
        // Create an original struct instance
        let original_tag = TagWithLReturn {
            is_with_rail: true,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 1];
        let size = original_tag.serialize(&mut buffer).unwrap();
        
        // Assert that the size is correct (1 byte)
        assert_eq!(size, 1);

        // Deserialize the bytes back into a new struct instance
        let deserialized_tag = TagWithLReturn::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_tag, deserialized_tag);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_with_l_return_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small (0 bytes)
        let buffer = [0u8; 0];
        let result = TagWithLReturn::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
