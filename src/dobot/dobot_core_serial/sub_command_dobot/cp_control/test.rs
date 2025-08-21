#[cfg(test)]
mod tests {

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::cp_control::CPSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{
                    mock_command_sender::{create_response_packet, MockCommandSender}, Dobot
                },
                dobot_error::DobotError,
                sub_command_dobot::cp_control::CPControl,
            },
            protocol::{
                bodies::{
                    general_request::GeneralRequest,
                    general_response::GeneralResponse,
                    tag_cp_cmd::{CPMode, TagCPCmd},
                    tag_cp_params::{RealTimeTrack, TagCPParams},
                    tag_queue::received::TagQueue,
                }, command_id::CpIDs, Body, CommunicationProtocolIDs, ProtocolError
            },
            rwlock::RwLock,
        },
    };

        // Mock implementation for TagCPCmd.

    // --- Tests for set_cp_cmd ---

    // This test verifies a successful call to set_cp_cmd without queuing.
    #[test]
    fn test_set_cp_cmd_ok() {
        let cmd = TagCPCmd { x: 10.0, y: 20.0, z: 30.0, cp_mode: CPMode::Relative, velocity_or_power: 10.0 };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Cp(CpIDs::CpCmd), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_cp_cmd with queuing.
    #[test]
    fn test_set_cp_cmd_queued_ok() {
        let cmd = TagCPCmd { x: 10.0, y: 20.0, z: 30.0, cp_mode: CPMode::Relative, velocity_or_power: 10.0 };
        let expected_queue_id: u64 = 123;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        mock_queue_response.serialize(&mut response_buffer).unwrap();

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cp(CpIDs::CpCmd),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for set_cp_params ---

    // This test verifies a successful call to set_cp_params without queuing.
    #[test]
    fn test_set_cp_params_ok() {
        let params = TagCPParams { plan_acc: 10.0, junction_acc: 5.0, acceleratio_or_period: 5.0, real_time_track: RealTimeTrack::NonRealTime };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Cp(CpIDs::CpParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_cp_params with queuing.
    #[test]
    fn test_set_cp_params_queued_ok() {
        let params = TagCPParams { plan_acc: 10.0, junction_acc: 5.0, acceleratio_or_period: 5.0, real_time_track: RealTimeTrack::NonRealTime };
        let expected_queue_id: u64 = 456;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        mock_queue_response.serialize(&mut response_buffer).unwrap();

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cp(CpIDs::CpParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for get_cp_params ---

    // This test verifies a successful call to get_cp_params.
    #[test]
    fn test_get_cp_params_ok() {
        let expected_params = TagCPParams { plan_acc: 10.0, junction_acc: 5.0, acceleratio_or_period: 5.0, real_time_track: RealTimeTrack::NonRealTime };
        let mut response_buffer = [0u8; 13];
        expected_params.serialize(&mut response_buffer).unwrap();

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cp(CpIDs::CpParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mutex);

        let result = cp_control.get_cp_params();
        assert!(result.is_ok());
        assert_eq!(result.clone().unwrap().plan_acc, expected_params.plan_acc);
        assert_eq!(result.clone().unwrap().junction_acc, expected_params.junction_acc);
        assert_eq!(result.clone().unwrap().acceleratio_or_period, expected_params.acceleratio_or_period);
        assert_eq!(result.clone().unwrap().real_time_track, expected_params.real_time_track);
    }

    // This test simulates an invalid response for get_cp_params.
    #[test]
    fn test_get_cp_params_invalid_response() {
        let invalid_params = [1, 2, 3];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cp(CpIDs::CpParams),
            &invalid_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.get_cp_params();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // --- Tests for set_cp_le_cmd ---

    // This test verifies a successful call to set_cp_le_cmd without queuing.
    #[test]
    fn test_set_cp_le_cmd_ok() {
        let cmd = TagCPCmd { x: 10.0, y: 20.0, z: 30.0, cp_mode: CPMode::Relative, velocity_or_power: 10.0 };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Cp(CpIDs::CpleCmd), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_le_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_cp_le_cmd with queuing.
    #[test]
    fn test_set_cp_le_cmd_queued_ok() {
        let cmd = TagCPCmd { x: 10.0, y: 20.0, z: 30.0, cp_mode: CPMode::Relative, velocity_or_power: 10.0 };
        let expected_queue_id: u64 = 789;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        mock_queue_response.serialize(&mut response_buffer).unwrap();

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cp(CpIDs::CpleCmd),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut cp_control = CPSerialControl::new(&mut mutex);

        let result = cp_control.set_cp_le_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }
}
