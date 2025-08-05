#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, ProtocolError,
        bodies::tag_cp_params::{RealTimeTrack, TagCPParams},
    };

    /// Test case for successful serialization and deserialization of TagCPParams.
    #[test]
    fn test_tag_cp_params_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_params = TagCPParams {
            plan_acc: 100.0,
            junction_acc: 50.5,
            acceleratio_or_period: 20.0,
            real_time_track: RealTimeTrack::RealTime,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 13];
        let size = original_params.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 floats + 1 u8 = 13 bytes)
        assert_eq!(size, 13);

        // Deserialize the bytes back into a new struct instance
        let deserialized_params = TagCPParams::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_params, deserialized_params);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_cp_params_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 12];
        let result = TagCPParams::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid RealTimeTrack value.
    #[test]
    fn test_tag_cp_params_unpack_invalid_real_time_track() {
        // Create a buffer with an invalid value for RealTimeTrack (e.g., 99)
        let mut buffer = [0u8; 13];
        buffer[12] = 99;
        let result = TagCPParams::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
