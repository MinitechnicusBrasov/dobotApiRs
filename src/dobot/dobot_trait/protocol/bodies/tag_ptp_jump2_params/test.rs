#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::ProtocolError;
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_ptp_jump2_params::TagPTPJump2Params,
    };

    /// Test case for successful serialization and deserialization of TagPTPJump2Params.
    #[test]
    fn test_tag_ptp_jump2_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagPTPJump2Params {
            start_jump_height: 50.0,
            end_jump_height: 75.0,
            z_limit: 100.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 12];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 floats * 4 bytes/float = 12 bytes)
        assert_eq!(size, 12);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagPTPJump2Params::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_ptp_jump2_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 11];
        let result = TagPTPJump2Params::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
