#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;
    use crate::dobot::dobot_trait::protocol::{Body, bodies::tag_io_pwm::TagIOPWM};

    /// Test case for successful serialization and deserialization of TagIOPWM.
    #[test]
    fn test_tag_io_pwm_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagIOPWM {
            address: 12,
            frequency: 50000.0,
            duty_cycle: 75.5,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 9];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 u8 + 2 f32s = 9 bytes)
        assert_eq!(size, 9);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagIOPWM::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_io_pwm_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 8];
        let result = TagIOPWM::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
