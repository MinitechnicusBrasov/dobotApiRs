#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::bodies::general_request::GeneralRequest;
    use crate::dobot::dobot_trait::protocol::{Body, ProtocolError};

    /// Test case for successful serialization of GeneralRequest with a full buffer.
    #[test]
    fn test_general_request_serialize_success() {
        let params_data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let request = GeneralRequest {
            params: &params_data,
        };
        let mut buffer = [0u8; 5]; // Exactly enough space

        let result = request.serialize(&mut buffer);

        // Assert that serialization was successful and returned the correct size
        assert_eq!(result, Ok(5));
        // Assert that the buffer contains the serialized data
        assert_eq!(buffer, params_data);
    }

    /// Test case for successful serialization of GeneralRequest with a larger buffer.
    #[test]
    fn test_general_request_serialize_larger_buffer() {
        let params_data = [0x0A, 0x0B, 0x0C];
        let request = GeneralRequest {
            params: &params_data,
        };
        let mut buffer = [0xFF; 10]; // More space than needed

        let result = request.serialize(&mut buffer);

        // Assert that serialization was successful and returned the correct size
        assert_eq!(result, Ok(3));
        // Assert that the buffer contains the serialized data at the beginning
        assert_eq!(buffer[0..3], params_data);
        // Assert that the rest of the buffer remains unchanged
        assert_eq!(buffer[3..], [0xFF; 7]);
    }

    /// Test case for serialization when the buffer is too small.
    #[test]
    fn test_general_request_serialize_buffer_too_small() {
        let params_data = [0x01, 0x02, 0x03];
        let request = GeneralRequest {
            params: &params_data,
        };
        let mut buffer = [0u8; 2]; // Not enough space

        let result = request.serialize(&mut buffer);

        // Assert that serialization failed with BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization, which should always return InvalidOperation.
    #[test]
    fn test_general_request_deserialize_invalid_operation() {
        let buffer = [0x01, 0x02, 0x03]; // Some arbitrary data

        let result = GeneralRequest::deserialize(&buffer);

        // Assert that deserialization always returns InvalidOperation error
        assert_eq!(result, Err(ProtocolError::InvalidOperation));
    }

    /// Test case for the size method.
    #[test]
    fn test_general_request_size() {
        let params_data = [0xAA, 0xBB, 0xCC, 0xDD];
        let request = GeneralRequest {
            params: &params_data,
        };

        // Assert that the size method returns the correct length of the params slice
        assert_eq!(request.size(), 4);

        let empty_params: &[u8] = &[];
        let empty_request = GeneralRequest {
            params: empty_params,
        };
        assert_eq!(empty_request.size(), 0);
    }
}
