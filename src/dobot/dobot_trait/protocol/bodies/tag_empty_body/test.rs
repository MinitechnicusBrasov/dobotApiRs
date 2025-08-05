#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::Body;
    use crate::dobot::dobot_trait::protocol::bodies::tag_empty_body::EmptyBody;

    /// Test case for successful serialization of EmptyBody.
    #[test]
    fn test_empty_body_serialize_success() {
        // Create an empty body instance
        let empty_body = EmptyBody {};

        // Create a buffer (its size doesn't matter for an empty body)
        let mut buffer = [0u8; 10];

        // Serialize the struct into the buffer
        let result = empty_body.serialize(&mut buffer);

        // Assert that the serialization was successful and returned a size of 0
        assert_eq!(result, Ok(0));
    }

    /// Test case for successful deserialization of EmptyBody from an empty buffer.
    #[test]
    fn test_empty_body_deserialize_success() {
        // Create an empty buffer
        let buffer = [0u8; 0];

        // Deserialize the bytes into a new struct instance
        let result = EmptyBody::deserialize(&buffer);

        // Assert that the deserialization was successful and returned an EmptyBody
        assert_eq!(result, Ok(EmptyBody {}));
    }
}
