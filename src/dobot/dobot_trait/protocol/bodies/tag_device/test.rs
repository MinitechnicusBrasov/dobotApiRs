#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body,
        bodies::tag_device::{TagDevice, TagVersionColorSensorAndIR},
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagDevice.
    #[test]
    fn test_tag_device_pack_unpack_success() {
        // Create an original struct instance with sample values
        let original_cmd = TagDevice {
            is_enabled: true,
            port: 5,
            version: TagVersionColorSensorAndIR::Version2,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 3];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (3 u8s = 3 bytes)
        assert_eq!(size, 3);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagDevice::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_device_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 2];
        let result = TagDevice::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    /// Test case for deserialization with an invalid TagVersionColorSensorAndIR value.
    #[test]
    fn test_tag_device_unpack_invalid_version() {
        // Create a buffer with an invalid value for TagVersionColorSensorAndIR (e.g., 99)
        let mut buffer = [0u8; 3];
        buffer[2] = 99;
        let result = TagDevice::deserialize(&buffer);

        // Assert that the deserialization failed with an InvalidEnumValue error
        assert_eq!(result, Err(ProtocolError::InvalidEnumValue));
    }
}
