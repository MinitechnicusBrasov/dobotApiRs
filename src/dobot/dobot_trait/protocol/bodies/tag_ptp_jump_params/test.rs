#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, ProtocolError, bodies::tag_ptp_jump_params::TagPTPJumpParams,
    };

    /// Test case for successful serialization and deserialization of TagPTPJumpParams.
    #[test]
    fn test_tag_ptp_jump_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagPTPJumpParams {
            jump_height: 50.0,
            z_limit: 100.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 8];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 floats * 4 bytes/float = 8 bytes)
        assert_eq!(size, 8);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagPTPJumpParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_ptp_jump_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 7];
        let result = TagPTPJumpParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
