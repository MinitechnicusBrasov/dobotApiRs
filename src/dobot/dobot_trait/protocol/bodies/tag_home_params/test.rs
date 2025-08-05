#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::{
        protocol::bodies::tag_home_params::TagHomeParams,
        protocol::{Body, protocol_error::ProtocolError},
    };

    /// Test case for successful serialization and deserialization of TagHomeParams.
    #[test]
    fn test_tag_home_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagHomeParams {
            x: 100.5,
            y: 50.25,
            z: 20.0,
            r: 90.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 16];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 floats * 4 bytes/float = 16 bytes)
        assert_eq!(size, 16);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagHomeParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_home_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 15];
        let result = TagHomeParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
