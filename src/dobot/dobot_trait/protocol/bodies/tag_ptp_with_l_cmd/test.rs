#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::bodies::tag_ptp_cmd::PTPMode;
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_ptp_with_l_cmd::TagPTPWithLCmd, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagPTPWithLCmd.
    #[test]
    fn test_tag_ptp_with_l_cmd_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagPTPWithLCmd {
            ptp_mode: PTPMode::MovlXyz,
            x: 100.0,
            y: 50.5,
            z: 20.0,
            r: -15.25,
            l: 30.75,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 21];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 u8 + 5 floats = 21 bytes)
        assert_eq!(size, 21);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagPTPWithLCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_ptp_with_l_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 20];
        let result = TagPTPWithLCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid PTPMode value.
    #[test]
    fn test_tag_ptp_with_l_cmd_unpack_invalid_ptp_mode() {
        // Create a buffer with an invalid value for PTPMode (e.g., 99)
        let mut buffer = [0u8; 21];
        buffer[0] = 99;
        let result = TagPTPWithLCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
