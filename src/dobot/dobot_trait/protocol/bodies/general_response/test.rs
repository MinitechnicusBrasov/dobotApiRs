#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::bodies::general_response::GeneralResponse;
    use crate::dobot::dobot_trait::protocol::{Body, ProtocolError};

    /// Test case for serialization, which should always return InvalidOperation.
    #[test]
    fn test_general_response_serialize_invalid_operation() {
        let params_data = [0x01, 0x02, 0x03];
        let response = GeneralResponse {
            params: &params_data,
        };
        let mut buffer = [0u8; 10]; // Arbitrary buffer

        let result = response.serialize(&mut buffer);

        // Assert that serialization always returns InvalidOperation error
        assert_eq!(result, Err(ProtocolError::InvalidOperation));
    }

    /// Test case for successful deserialization of GeneralResponse.
    #[test]
    fn test_general_response_deserialize_success() {
        let buffer_data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

        let result = GeneralResponse::deserialize(&buffer_data);

        // Assert that deserialization was successful
        assert!(result.is_ok());

        let deserialized_response = result.unwrap();
        // Assert that the params slice in the deserialized struct points to the original buffer data
        assert_eq!(deserialized_response.params, &buffer_data);
        // Assert that the size is correct
        assert_eq!(deserialized_response.size(), buffer_data.len());
    }

    /// Test case for deserialization with an empty buffer.
    #[test]
    fn test_general_response_deserialize_empty_buffer() {
        let empty_buffer: &[u8] = &[];

        let result = GeneralResponse::deserialize(empty_buffer);

        // Assert that deserialization was successful even with an empty buffer
        assert!(result.is_ok());

        let deserialized_response = result.unwrap();
        // Assert that the params slice is empty
        assert_eq!(deserialized_response.params, empty_buffer);
        // Assert that the size is 0
        assert_eq!(deserialized_response.size(), 0);
    }

    /// Test case for the size method.
    #[test]
    fn test_general_response_size() {
        let params_data = [0x10, 0x20, 0x30];
        let response = GeneralResponse {
            params: &params_data,
        };

        // Assert that the size method returns the correct length of the params slice
        assert_eq!(response.size(), 3);

        let empty_params: &[u8] = &[];
        let empty_response = GeneralResponse {
            params: empty_params,
        };
        assert_eq!(empty_response.size(), 0);
    }
}
