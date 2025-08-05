#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body,
        bodies::tag_emotor::{TagEMotor, send::EMotorIndex},
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagEMotor.
    #[test]
    fn test_tag_e_motor_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagEMotor {
            address: EMotorIndex::Stepper1,
            ins_enabled: true,
            speed: 500.5,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 10];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 u8s + 1 f64 = 10 bytes)
        assert_eq!(size, 10);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagEMotor::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_e_motor_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 9];
        let result = TagEMotor::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid EMotorIndex value.
    #[test]
    fn test_tag_e_motor_unpack_invalid_index() {
        // Create a buffer with an invalid value for EMotorIndex (e.g., 99)
        let mut buffer = [0u8; 10];
        buffer[0] = 99;
        let result = TagEMotor::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
