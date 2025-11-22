#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_end_effector_params::TagEndEffectorParams, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagEndEffectorParams.
    #[test]
    fn test_tag_end_effector_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagEndEffectorParams {
            x_bias: 10.5,
            y_bias: 20.25,
            z_bias: -5.75,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 24];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 f64 * 8 bytes/f64 = 24 bytes)
        assert_eq!(size, 12);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagEndEffectorParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_end_effector_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 11];
        let result = TagEndEffectorParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
