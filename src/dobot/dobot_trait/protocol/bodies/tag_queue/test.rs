#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::Body;
    use crate::dobot::dobot_trait::protocol::bodies::tag_queue::received::TagQueue;
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

    /// Test case for successful serialization and deserialization of a TagQueue struct.
    #[test]
    fn test_tag_queue_pack_unpack_success() {
        // Create an original struct instance with a sample value
        let original_cmd = TagQueue {
            queue_idx: 1234567890123456789,
        };

        // The size of i64 is 8 bytes
        let size = core::mem::size_of::<i64>();
        let mut buffer = [0u8; 8];

        // Serialize the struct into the buffer
        let result = original_cmd.serialize(&mut buffer);

        // Assert that the serialization was successful and the size is correct
        assert_eq!(result, Ok(size));

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagQueue::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for serialization with a buffer that is too small.
    #[test]
    fn test_tag_queue_serialize_buffer_too_small() {
        let original_cmd = TagQueue { queue_idx: 1 };
        // Create a buffer that is intentionally too small (7 bytes instead of 8)
        let mut buffer = [0u8; 7];
        let result = original_cmd.serialize(&mut buffer);

        // Assert that the serialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_queue_deserialize_buffer_too_small() {
        // Create a buffer that is intentionally too small (7 bytes instead of 8)
        let buffer = [0u8; 7];
        let result = TagQueue::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
