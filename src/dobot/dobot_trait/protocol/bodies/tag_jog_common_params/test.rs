#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_jog_common_params::TagJOGCommonParams, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagJOGCommonParams.
    #[test]
    fn test_tag_jog_common_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagJOGCommonParams {
            velocity_ratio: 0.75,
            acceleration_ratio: 0.5,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 8];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 floats * 4 bytes/float = 8 bytes)
        assert_eq!(size, 8);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagJOGCommonParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_jog_common_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 7];
        let result = TagJOGCommonParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
