#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::{
        Body,
        bodies::tag_arc_cmd::{Point, TagARCCmd},
        protocol_error::ProtocolError,
    };

    /// Test case for successful serialization and deserialization of Point.
    #[test]
    fn test_point_pack_unpack_success() {
        // Create an original struct instance with sample float values
        let original_point = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            r: 4.0,
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 16];
        let size = original_point.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (4 floats * 4 bytes/float = 16 bytes)
        assert_eq!(size, 16);

        // Deserialize the bytes back into a new struct instance
        let deserialized_point = Point::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_point, deserialized_point);
    }

    /// Test case for successful serialization and deserialization of TagARCCmd.
    #[test]
    fn test_tag_arc_cmd_pack_unpack_success() {
        // Create an original struct instance with sample Point structs
        let original_cmd = TagARCCmd {
            circ_point: Point {
                x: 100.0,
                y: 50.0,
                z: 25.0,
                r: 10.0,
            },
            to_point: Point {
                x: 200.0,
                y: 150.0,
                z: 75.0,
                r: 50.0,
            },
        };

        // Create a buffer and serialize the struct into it
        let mut buffer = [0u8; 32];
        let size = original_cmd.serialize(&mut buffer).unwrap();

        // Assert that the size is correct (2 Points * 16 bytes/Point = 32 bytes)
        assert_eq!(size, 32);

        // Deserialize the bytes back into a new struct instance
        let deserialized_cmd = TagARCCmd::deserialize(&buffer[..size]).unwrap();

        // Assert that the original and deserialized structs are identical
        assert_eq!(original_cmd, deserialized_cmd);
    }

    /// Test case for deserialization of TagARCCmd with a buffer that is too small.
    #[test]
    fn test_tag_arc_cmd_unpack_buffer_too_small() {
        // Create a buffer that is intentionally too small
        let buffer = [0u8; 31];
        let result = TagARCCmd::deserialize(&buffer);

        // Assert that the deserialization failed with a BufferTooSmall error
        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }
}
