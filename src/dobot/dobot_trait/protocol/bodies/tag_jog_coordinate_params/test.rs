#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_jog_coordinate_params::TagJOGCoordinateParams,
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagJOGCoordinateParams.
    #[test]
    fn test_tag_jog_coordinate_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagJOGCoordinateParams {
            velocity: [100.0, 50.0, 20.0, 90.0],
            acceleration: [15.0, 30.0, 45.0, 60.0],
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 32];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (8 floats * 4 bytes/float = 32 bytes)
        assert_eq!(size, 32);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagJOGCoordinateParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_jog_coordinate_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 31];
        let result = TagJOGCoordinateParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
