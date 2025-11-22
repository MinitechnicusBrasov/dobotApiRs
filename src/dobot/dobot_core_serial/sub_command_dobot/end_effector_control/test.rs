#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::end_effector_control::EndEffectorSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{
                    Dobot,
                    mock_command_sender::{MockCommandSender, create_response_packet},
                },
                dobot_error::DobotError,
                sub_command_dobot::end_effector_control::EndEffectorControl,
            },
            protocol::{
                CommunicationProtocolIDs, ProtocolError,
                bodies::tag_end_effector_params::TagEndEffectorParams, command_id::EndEffectorIDs,
            },
            rwlock::RwLock,
        },
    };

    #[test]
    fn test_set_gripper_state_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_gripper_state(true, true, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_gripper_state_ok_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_gripper_state(true, false, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_gripper_state_ok() {
        let response_params = [1, 0]; // enabled: true, gripped: false
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper),
            &response_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_gripper_state();
        assert!(result.is_ok());
        let (enabled, gripped) = result.unwrap();
        assert!(enabled);
        assert!(!gripped);
    }

    #[test]
    fn test_set_suction_cup_state_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_suction_cup_state(true, true, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_suction_cup_state_ok_queued() {
        let queue_idx: u64 = 456;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_suction_cup_state(false, true, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_suction_cup_state_ok() {
        let response_params = [0, 1]; // enabled: false, sucking: true
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup),
            &response_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_suction_cup_state();
        assert!(result.is_ok());
        let (enabled, sucking) = result.unwrap();
        assert!(!enabled);
        assert!(sucking);
    }

    #[test]
    fn test_set_laser_state_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_laser_state(true, true, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_laser_state_ok_queued() {
        let queue_idx: u64 = 789;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.set_laser_state(true, false, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_laser_state_ok() {
        let response_params = [1, 1]; // enabled: true, on: true
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser),
            &response_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_laser_state();
        assert!(result.is_ok());
        let (enabled, on) = result.unwrap();
        assert!(enabled);
        assert!(on);
    }

    #[test]
    fn test_set_end_effector_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let params = TagEndEffectorParams {
            x_bias: 10.0,
            y_bias: 20.0,
            z_bias: 30.0,
        };
        let result = control.set_end_effector_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_end_effector_params_ok_queued() {
        let queue_idx: u64 = 999;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let params = TagEndEffectorParams {
            x_bias: 5.5,
            y_bias: 10.5,
            z_bias: 15.5,
        };
        let result = control.set_end_effector_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_end_effector_params_ok() {
        let mut response_body = Vec::new();
        response_body.extend((100.0f32).to_le_bytes()); // x_bias
        response_body.extend((200.0f32).to_le_bytes()); // y_bias
        response_body.extend((300.0f32).to_le_bytes()); // z_bias

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_end_effector_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.x_bias, 100.0);
        assert_eq!(params.y_bias, 200.0);
        assert_eq!(params.z_bias, 300.0);
    }

    #[test]
    fn test_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_gripper_state();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_get_gripper_state_invalid_response() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper),
            &[1], // Invalid 1-byte response, expected 2 bytes
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_gripper_state();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_gripper_state_both_disabled() {
        let response_params = [0, 0]; // enabled: false, gripped: false
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper),
            &response_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_gripper_state();
        assert!(result.is_ok());
        let (enabled, gripped) = result.unwrap();
        assert!(!enabled);
        assert!(!gripped);
    }

    #[test]
    fn test_laser_state_enabled_off() {
        let response_params = [1, 0]; // enabled: true, on: false
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser),
            &response_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = EndEffectorSerialControl::new(&mutex);

        let result = control.get_laser_state();
        assert!(result.is_ok());
        let (enabled, on) = result.unwrap();
        assert!(enabled);
        assert!(!on);
    }
}
