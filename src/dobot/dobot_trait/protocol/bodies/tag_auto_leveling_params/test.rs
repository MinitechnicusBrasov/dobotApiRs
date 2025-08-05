#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_auto_leveling_params::TagAutoLevelingParams,
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagAutoLevelingParams.
    #[test]
    fn test_tag_auto_leveling_params_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_params = TagAutoLevelingParams {
            is_auto_leveling: true,
            accuracy: 0.12345,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 5];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 u8 + 1 f32 = 5 bytes)
        assert_eq!(size, 5);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagAutoLevelingParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_auto_leveling_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 4];
        let result = TagAutoLevelingParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
