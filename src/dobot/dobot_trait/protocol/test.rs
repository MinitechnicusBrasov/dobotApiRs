use super::{
    Body, CommunicationProtocolIDs, Protocol, define_protocol_group,
    implement_try_from_for_protocol_group, protocol_error::ProtocolError,
};
use core::convert::TryFrom;
use paste::paste;

// This macro simplifies the creation of structs that implement the `Body` trait.
// It automatically generates the `size()`, `serialize()`, and `deserialize()`
// methods for a given struct and its fields, using little-endian byte order.
//
// Example usage:
// implement_body! {
//     pub struct MyBody {
//         pub my_u8: u8,
//         pub my_u32: u32,
//     }
// }
macro_rules! implement_body {
    ($($visibility:vis struct $name:ident {
        $(
            $field_visibility:vis $field_name:ident: $field_type:ty,
        )*
    })*) => {
        $(
            #[derive(Debug, PartialEq, Eq)]
            $visibility struct $name {
                $(
                    $field_visibility $field_name: $field_type,
                )*
            }

            impl Body for $name {
                fn size(&self) -> usize {
                    let mut size = 0;
                    $(
                        size += core::mem::size_of::<$field_type>();
                    )*
                    size
                }

                fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
                    let mut index = 0;
                    $(
                        let field_size = core::mem::size_of::<$field_type>();
                        if index + field_size > buffer.len() {
                            return Err(ProtocolError::BufferTooSmall);
                        }
                        // Use little-endian byte order for serialization
                        buffer[index..index + field_size].copy_from_slice(&self.$field_name.to_le_bytes());
                        index += field_size;
                    )*
                    Ok(index)
                }

                fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
                    let mut index = 0;
                    $(
                        let field_size = core::mem::size_of::<$field_type>();
                        if index + field_size > buffer.len() {
                            return Err(ProtocolError::BufferTooSmall); // Can be a more specific error
                        }
                        let mut bytes = [0u8; core::mem::size_of::<$field_type>()];
                        bytes.copy_from_slice(&buffer[index..index + field_size]);
                        let $field_name = <$field_type>::from_le_bytes(bytes);
                        index += field_size;
                    )*
                    Ok(Self {
                        $(
                            $field_name,
                        )*
                    })
                }
            }
        )*
    };
}

// Define a simple Body for testing purposes using the new macro.
implement_body! {
    pub struct TestBody {
        pub data: u32,
        pub status: u8,
    }
}

// Define a specific command group for testing.
define_protocol_group! {
    TestIDs, 250, {
        TestCmd = 0,
        MacroTest = 1,
    }
}

// Implement TryFrom for the test command group.
implement_try_from_for_protocol_group!(TestIDs);

// The `tests` module contains all the unit tests for the protocol.
#[cfg(test)]
mod tests {

    // A test to ensure that a packet can be created and then deserialized correctly.
    #[test]
    fn test_to_packet_from_packet_success() {
        let command_id = CommunicationProtocolIDs::Test(TestIDs::TestCmd);
        let is_queued = false;
        let is_read = false;
        let body = TestBody {
            data: 0x12345678,
            status: 0xAB,
        };

        let protocol = Protocol::new(command_id, is_queued, is_read, body);
        let mut buffer = [0u8; 100];
        let size = protocol.to_packet(&mut buffer).unwrap();

        let deserialized_protocol = Protocol::<TestBody>::from_packet(&buffer[..size]).unwrap();

        assert_eq!(deserialized_protocol.command_id, command_id);
        assert_eq!(deserialized_protocol.is_queued, is_queued);
        assert_eq!(deserialized_protocol.is_read, is_read);
        assert_eq!(deserialized_protocol.body.data, 0x12345678);
        assert_eq!(deserialized_protocol.body.status, 0xAB);
    }

    // A test for the `to_packet` method when the buffer is too small.
    #[test]
    fn test_to_packet_buffer_too_small() {
        let command_id = CommunicationProtocolIDs::Test(TestIDs::TestCmd);
        let body = TestBody {
            data: 0x12345678,
            status: 0xAB,
        };

        let protocol = Protocol::new(command_id, false, false, body);
        let mut buffer = [0u8; 1]; // Buffer is intentionally too small

        let result = protocol.to_packet(&mut buffer);

        assert_eq!(result, Err(ProtocolError::BufferTooSmall));
    }

    // A test for the `from_packet` method when the start bytes are missing.
    #[test]
    fn test_from_packet_invalid_start_bytes() {
        let packet = [
            0xAB,
            0xAA,
            0x07, // Invalid start byte
            CommunicationProtocolIDs::Test(TestIDs::TestCmd).into(),
            0x00,
            0x78,
            0x56,
            0x34,
            0x12,
            0xAB,
            0xC5, // Checksum
        ];

        let result = Protocol::<TestBody>::from_packet(&packet);
        assert_eq!(result, Err(ProtocolError::MissingStartBytes));
    }

    // A test for the `from_packet` method when the checksum is incorrect.
    #[test]
    fn test_from_packet_checksum_error() {
        let packet = [
            0xAA,
            0xAA,
            0x07,
            CommunicationProtocolIDs::Test(TestIDs::TestCmd).into(),
            0x00,
            0x78,
            0x56,
            0x34,
            0x12,
            0xAB,
            0x00, // Invalid checksum
        ];

        let result = Protocol::<TestBody>::from_packet(&packet);
        assert_eq!(result, Err(ProtocolError::ChecksumError));
    }

    // A test for the `from_packet` method when the buffer is too small to read the entire packet.
    #[test]
    fn test_from_packet_buffer_too_small_on_read() {
        let packet = [
            0xAA,
            0xAA,
            0x07, // Length field says 7 bytes of content
            CommunicationProtocolIDs::Test(TestIDs::TestCmd).into(),
            0x00,
            0x78,
            0x56,
            0x34,
            0x12,
            0xAB,
            // The rest of the packet is missing
        ];

        let result = Protocol::<TestBody>::from_packet(&packet);
        assert_eq!(result, Err(ProtocolError::LengthMismatch));
    }

    // Test the conversion from a u8 to a CommunicationProtocolIDs enum.
    #[test]
    fn test_command_id_conversion() {
        let raw_id = 10; // Corresponds to `DevicePoseIDs::GetPose`
        let expected_id = CommunicationProtocolIDs::DevicePose(super::DevicePoseIDs::GetPose);

        let converted_id = CommunicationProtocolIDs::try_from(raw_id);

        assert_eq!(converted_id, Ok(expected_id));
    }

    // Test the macro for creating a Body implementation.
    #[test]
    fn test_macro_generated_body() {
        let original_body = TestBody {
            data: 0xDEADBEEF,
            status: 0xFF,
        };
        let mut buffer = [0u8; 100];
        let size = original_body.serialize(&mut buffer).unwrap();

        let deserialized_body = TestBody::deserialize(&buffer[..size]).unwrap();

        assert_eq!(original_body, deserialized_body);
        assert_eq!(size, 5); // u32 (4 bytes) + u8 (1 byte)
    }
}
