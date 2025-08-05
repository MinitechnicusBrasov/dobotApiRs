#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::Body;
    use crate::dobot::dobot_trait::protocol::bodies::tag_trig_cmd::{
        TagTRIGCmd, TriggerCondition, TriggerMode,
    };
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

    /// Test case for successful serialization and deserialization of TagTRIGCmd.
    #[test]
    fn test_tag_trig_cmd_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagTRIGCmd {
            address: 15,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::LevelUnequalOrAdLessEqual,
            threshold: 1024,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 5];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 u8s + 1 u16 = 5 bytes)
        assert_eq!(size, 5);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagTRIGCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_trig_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 4];
        let result = TagTRIGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid TriggerMode value.
    #[test]
    fn test_tag_trig_cmd_unpack_invalid_trigger_mode() {
        // Create a buffer with an invalid value for TriggerMode (e.g., 99)
        let mut buffer = [0u8; 5];
        buffer[1] = 99;
        let result = TagTRIGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }

    /// Test case for deserialization with an invalid TriggerCondition value.
    #[test]
    fn test_tag_trig_cmd_unpack_invalid_trigger_condition() {
        // Create a buffer with an invalid value for TriggerCondition (e.g., 99)
        let mut buffer = [0u8; 5];
        buffer[2] = 99;
        let result = TagTRIGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
