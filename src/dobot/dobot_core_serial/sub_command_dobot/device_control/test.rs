#[cfg(test)]
mod tests {
    use critical_section::Mutex;

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::device_control::DeviceSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::mock_command_sender::{MockCommandSender, create_response_packet},
                dobot_error::DobotError,
                sub_command_dobot::device_control::DeviceControl,
            },
            protocol::{
                CommunicationProtocolIDs, ProtocolError,
                bodies::tag_with_l::{TagVersionRail, TagWithL},
                command_id::DeviceInfoIDs,
            },
        },
    };

    #[test]
    fn test_set_device_sn_ok() {
        let sn = b"1234567890ABCDEF";
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.set_device_sn(sn);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_device_sn_ok() {
        let expected_sn = b"TESTSN1234";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn),
            expected_sn,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);
        let mut buffer = [0u8; 32];

        let result = device_control.get_device_sn(&mut buffer);

        assert!(result.is_ok());
        let len = result.unwrap();
        assert_eq!(len, expected_sn.len());
        assert_eq!(&buffer[..len], expected_sn);
    }

    // This test covers the case where the provided buffer is too small for the response.
    #[test]
    fn test_get_device_sn_buffer_too_small() {
        let expected_sn = b"TESTSN1234";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn),
            expected_sn,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);
        let mut buffer = [0u8; 5]; // Buffer is smaller than the expected SN

        let result = device_control.get_device_sn(&mut buffer);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_set_device_name_ok() {
        let name = b"MyDobot";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.set_device_name(name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_device_name_ok() {
        let expected_name = b"TestName";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            expected_name,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);
        let mut buffer = [0u8; 32];

        let result = device_control.get_device_name(&mut buffer);

        assert!(result.is_ok());
        let len = result.unwrap();
        assert_eq!(len, expected_name.len());
        assert_eq!(&buffer[..len], expected_name);
    }

    #[test]
    fn test_get_device_version_ok() {
        let version_params = [1, 2, 3];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Version),
            &version_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_version();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (1, 2, 3));
    }

    // This test simulates a malformed response where the version is missing bytes.
    #[test]
    fn test_get_device_version_invalid_response() {
        let version_params = [1, 2]; // Missing the third byte
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Version),
            &version_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_version();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::InvalidEnumValue)
        ));
    }

    #[test]
    fn test_set_device_rail_capability_ok() {
        let request_body = TagWithL {
            is_with_rail: true,
            version: TagVersionRail::VerV2,
        };
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.set_device_rail_capability(request_body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_device_rail_capability_true() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail),
            &[1],
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_rail_capability();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_get_device_rail_capability_false() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail),
            &[0],
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_rail_capability();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_get_device_time_ok() {
        let time_val: u32 = 1672531200; // Example timestamp
        let params = time_val.to_le_bytes();
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Time),
            &params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_time();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), time_val);
    }

    #[test]
    fn test_get_device_id_ok() {
        let id1: u32 = 123;
        let id2: u32 = 456;
        let id3: u32 = 789;
        let mut params = Vec::new();
        params.extend_from_slice(&id1.to_le_bytes());
        params.extend_from_slice(&id2.to_le_bytes());
        params.extend_from_slice(&id3.to_le_bytes());

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Id),
            &params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);

        let result = device_control.get_device_id();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (id1, id2, id3));
    }

    // This test checks for the scenario where the mock sender returns a protocol error
    // simulating a communication failure.
    #[test]
    fn test_get_device_sn_send_raw_packet_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = DeviceSerialControl::new(&mut mutex);
        let mut buffer = [0u8; 32];

        let result = device_control.get_device_sn(&mut buffer);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
