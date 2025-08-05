#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, ProtocolError, bodies::tag_arc_params::TagARCParams,
    };

    /// Test case for successful serialization and deserialization of TagARCParams.
    #[test]
    fn test_tag_arc_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagARCParams {
            xyz_velocity: 100.0,
            r_velocity: 50.0,
            xyz_acceleration: 200.0,
            r_acceleration: 75.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 16];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 floats * 4 bytes/float = 16 bytes)
        assert_eq!(size, 16);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagARCParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_arc_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 15];
        let result = TagARCParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
