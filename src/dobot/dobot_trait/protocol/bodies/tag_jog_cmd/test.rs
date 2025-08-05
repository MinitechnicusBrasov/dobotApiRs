#[cfg(test)]
mod tests {
    use super::super::{JogCmd, JogMode, TagJOGCmd};
    use crate::dobot::dobot_trait::protocol::Body;
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

    /// Test case for successful serialization and deserialization of TagJOGCmd.
    #[test]
    fn test_tag_jog_cmd_pack_unpack_success() {
        // Create an original struct instance with sample enum values
        let original_cmd = TagJOGCmd {
            is_joint: JogMode::Joint,
            cmd: JogCmd::ApDown,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 2];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 u8s = 2 bytes)
        assert_eq!(size, 2);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagJOGCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_jog_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 1];
        let result = TagJOGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid JogMode value.
    #[test]
    fn test_tag_jog_cmd_unpack_invalid_jog_mode() {
        // Create a buffer with an invalid value for JogMode
        let buffer = [99u8, 1];
        let result = TagJOGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }

    /// Test case for deserialization with an invalid JogCmd value.
    #[test]
    fn test_tag_jog_cmd_unpack_invalid_jog_cmd() {
        // Create a buffer with an invalid value for JogCmd
        let buffer = [0u8, 99];
        let result = TagJOGCmd::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
