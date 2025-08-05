#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::ProtocolError;
    use crate::dobot::dobot_trait::protocol::{
        Body,
        bodies::tag_io_multiplexing::{IOFunction, TagIOMultiplexing},
    };

    /// Test case for successful serialization and deserialization of TagIOMultiplexing.
    #[test]
    fn test_tag_io_multiplexing_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagIOMultiplexing {
            address: 10,
            multiplex: IOFunction::Pwm,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 2];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 u8s = 2 bytes)
        assert_eq!(size, 2);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagIOMultiplexing::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_io_multiplexing_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 1];
        let result = TagIOMultiplexing::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid IOFunction value.
    #[test]
    fn test_tag_io_multiplexing_unpack_invalid_io_function() {
        // Create a buffer with an invalid value for IOFunction (e.g., 99)
        let mut buffer = [0u8; 2];
        buffer[1] = 99;
        let result = TagIOMultiplexing::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
