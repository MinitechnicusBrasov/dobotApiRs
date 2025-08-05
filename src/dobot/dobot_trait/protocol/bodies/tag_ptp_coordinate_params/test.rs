#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, ProtocolError, bodies::tag_ptp_coordinate_params::TagPTPCoordinateParams,
    };

    /// Test case for successful serialization and deserialization of TagPTPCoordinateParams.
    #[test]
    fn test_tag_ptp_coordinate_params_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_params = TagPTPCoordinateParams {
            xyz_velocity: 100.0,
            r_velocity: 50.0,
            xyz_acceleration: 15.0,
            r_acceleration: 30.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 16];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 floats * 4 bytes/float = 16 bytes)
        assert_eq!(size, 16);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagPTPCoordinateParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_ptp_coordinate_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 15];
        let result = TagPTPCoordinateParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
