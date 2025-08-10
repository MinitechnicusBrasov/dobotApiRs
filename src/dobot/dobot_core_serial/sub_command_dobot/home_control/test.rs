#[cfg(test)]
mod tests {
    use critical_section::Mutex;

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::home_control::HomeSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{
                    Dobot,
                    mock_command_sender::{MockCommandSender, create_response_packet},
                },
                dobot_error::DobotError,
                sub_command_dobot::home_control::HomeControl,
            },
            protocol::{
                Body, CommunicationProtocolIDs, ProtocolError,
                bodies::{
                    general_response::GeneralResponse,
                    tag_auto_leveling_params::TagAutoLevelingParams, tag_home_cmd::TagHomeCmd,
                    tag_home_params::TagHomeParams, tag_queue::received::TagQueue,
                },
                command_id::HomeIDs,
            },
            rwlock::RwLock,
        },
    };

    // Helper macro from the example to create a mutex for the mock sender.
    // Assuming this macro exists in the original project.
    // --- Tests for set_home_params ---

    // This test verifies a successful call to set_home_params without queuing.
    #[test]
    fn test_set_home_params_ok() {
        let params = TagHomeParams {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
        };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Home(HomeIDs::HomeParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_home_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_home_params with queuing.
    #[test]
    fn test_set_home_params_queued_ok() {
        let params = TagHomeParams {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
        };
        let expected_queue_id: u64 = 123;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        let _ = mock_queue_response.serialize(&mut response_buffer);

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::HomeParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_home_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for get_home_params ---

    // This test verifies a successful call to get_home_params.
    #[test]
    fn test_get_home_params_ok() {
        let expected_params = TagHomeParams {
            x: 10.0,
            y: 20.0,
            z: 30.0,
            r: 40.0,
        };
        let mut response_buffer = [0u8; 16];
        let serialized_params = expected_params.serialize(&mut response_buffer);
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::HomeParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.get_home_params();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_params);
    }

    // This test simulates an invalid response for get_home_params,
    // which should result in a ProtocolError.
    #[test]
    fn test_get_home_params_invalid_response() {
        // Create a response with a payload that's too small for TagHomeParams.
        let invalid_params = [1, 2, 3];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::HomeParams),
            &invalid_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.get_home_params();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // --- Tests for set_home_cmd ---

    // This test verifies a successful call to set_home_cmd without queuing.
    #[test]
    fn test_set_home_cmd_ok() {
        let params = TagHomeCmd { reserved: 10 };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Home(HomeIDs::HomeCmd), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_home_cmd(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_home_cmd with queuing.
    #[test]
    fn test_set_home_cmd_queued_ok() {
        let params = TagHomeCmd { reserved: 20 };
        let expected_queue_id: u64 = 456;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        match mock_queue_response.serialize(&mut response_buffer) {
            Ok(x) => x,
            Err(err) => panic!("Serialization failed: {err:?}"),
        };
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::HomeCmd),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_home_cmd(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for set_autoleveling ---

    // This test verifies a successful call to set_autoleveling without queuing.
    #[test]
    fn test_set_autoleveling_ok() {
        let params = TagAutoLevelingParams {
            is_auto_leveling: true,
            accuracy: 20.0,
        };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_autoleveling(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_autoleveling with queuing.
    #[test]
    fn test_set_autoleveling_queued_ok() {
        let params = TagAutoLevelingParams {
            is_auto_leveling: true,
            accuracy: 10.0,
        };
        let expected_queue_id: u64 = 789;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        &mock_queue_response.serialize(&mut response_buffer);
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.set_autoleveling(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for get_autoleveling ---

    // This test verifies a successful call to get_autoleveling.
    #[test]
    fn test_get_autoleveling_ok() {
        let expected_value: f32 = 1.234;
        let params = expected_value.to_le_bytes();
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling),
            &params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.get_autoleveling();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    // This test simulates an invalid response for get_autoleveling,
    // which should result in a conversion error.
    #[test]
    fn test_get_autoleveling_invalid_response() {
        // Response with a payload that's too small for an f32 (4 bytes).
        let invalid_params = [1, 2];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling),
            &invalid_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);

        let result = home_control.get_autoleveling();
        assert!(result.is_err());
        let err = result.unwrap_err();
        // The error type depends on how f32::from_le_bytes handles incorrect sizes.
        // It will likely be a DobotError::Protocol error.
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // --- Communication Error Test ---

    // This test checks for a general communication failure from the mock sender.
    #[test]
    fn test_home_control_send_raw_packet_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut home_control = HomeSerialControl::new(&mut mutex);
        let params = TagHomeParams {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
        };

        let result = home_control.set_home_params(params, false);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
