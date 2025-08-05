#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body,
        bodies::{level::Level, tag_io_di::TagIODI},
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagIODI.
    #[test]
    fn test_tag_io_di_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagIODI {
            address: 10,
            level: Level::High,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 2];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 u8s = 2 bytes)
        assert_eq!(size, 2);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagIODI::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_io_di_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 1];
        let result = TagIODI::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid Level value.
    #[test]
    fn test_tag_io_di_unpack_invalid_level() {
        // Create a buffer with an invalid value for Level (e.g., 99)
        let mut buffer = [0u8; 2];
        buffer[1] = 99;
        let result = TagIODI::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
