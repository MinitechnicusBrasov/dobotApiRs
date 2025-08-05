#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, ProtocolError,
        bodies::tag_ptp_cmd::{PTPMode, TagPTPCmd},
    };

    /// Test case for successful serialization and deserialization of TagPTPCmd.
    #[test]
    fn test_tag_ptp_cmd_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagPTPCmd {
            ptp_mode: PTPMode::MovjXyz,
            x: 100.0,
            y: 50.5,
            z: 20.0,
            r: -15.25,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 17];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (1 u8 + 4 floats = 17 bytes)
        assert_eq!(size, 17);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagPTPCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_ptp_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 16];
        let result = TagPTPCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid PTPMode value.
    #[test]
    fn test_tag_ptp_cmd_unpack_invalid_ptp_mode() {
        // Create a buffer with an invalid value for PTPMode (e.g., 99)
        let mut buffer = [0u8; 17];
        buffer[0] = 99;
        let result = TagPTPCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
