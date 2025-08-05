#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body, bodies::tag_pose::TagPose, protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of TagPose.
    #[test]
    fn test_tag_pose_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_pose = TagPose {
            x: 100.5,
            y: 50.25,
            z: 20.0,
            r: 90.0,
            joint_angle: [15.0, 30.5, 45.0, 60.75],
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 32];
        let size = original_pose.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (8 floats * 4 bytes/float = 32 bytes)
        assert_eq!(size, 32);

        // Deserialize the bytes back into a new struct instance
        let deserialized_pose = TagPose::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_pose, deserialized_pose);
    }

    /// Test case for deserialization with a buffer that is too small.
    #[test]
    fn test_tag_pose_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 31];
        let result = TagPose::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
