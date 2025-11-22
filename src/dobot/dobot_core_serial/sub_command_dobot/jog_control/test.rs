#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::jog_control::JOGSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{
                    Dobot,
                    mock_command_sender::{MockCommandSender, create_response_packet},
                },
                dobot_error::DobotError,
                sub_command_dobot::jog_control::JOGControl,
            },
            protocol::{
                CommunicationProtocolIDs, ProtocolError,
                bodies::{
                    tag_jog_cmd::{JogCmd, JogMode, TagJOGCmd},
                    tag_jog_common_params::TagJOGCommonParams,
                    tag_jog_coordinate_params::TagJOGCoordinateParams,
                    tag_jog_joint_params::TagJOGJointParams,
                    tag_jog_l_params::TagJOGLParams,
                },
                command_id::JogIDs,
            },
            rwlock::RwLock,
        },
    };

    #[test]
    fn test_set_jog_joint_params_ok_not_queued() {
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::JointParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGJointParams {
            velocity: [100.0, 150.0, 200.0, 250.0],
            acceleration: [300.0, 350.0, 400.0, 450.0],
        };
        let result = control.set_jog_joint_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_jog_joint_params_ok_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::JointParams),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGJointParams {
            velocity: [10.0, 20.0, 30.0, 40.0],
            acceleration: [50.0, 60.0, 70.0, 80.0],
        };
        let result = control.set_jog_joint_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_jog_joint_params_ok() {
        let mut response_body = Vec::new();
        // 4 velocity values
        response_body.extend((100.0f32).to_le_bytes());
        response_body.extend((150.0f32).to_le_bytes());
        response_body.extend((200.0f32).to_le_bytes());
        response_body.extend((250.0f32).to_le_bytes());
        // 4 acceleration values
        response_body.extend((300.0f32).to_le_bytes());
        response_body.extend((350.0f32).to_le_bytes());
        response_body.extend((400.0f32).to_le_bytes());
        response_body.extend((450.0f32).to_le_bytes());

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::JointParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_joint_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, [100.0, 150.0, 200.0, 250.0]);
        assert_eq!(params.acceleration, [300.0, 350.0, 400.0, 450.0]);
    }

    #[test]
    fn test_set_jog_coordinate_params_ok_not_queued() {
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGCoordinateParams {
            velocity: [10.0, 20.0, 30.0, 40.0],
            acceleration: [50.0, 60.0, 70.0, 80.0],
        };
        let result = control.set_jog_coordinate_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_jog_coordinate_params_ok_queued() {
        let queue_idx: u64 = 456;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGCoordinateParams {
            velocity: [5.5, 10.5, 15.5, 20.5],
            acceleration: [25.5, 30.5, 35.5, 40.5],
        };
        let result = control.set_jog_coordinate_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_jog_coordinate_params_ok() {
        let mut response_body = Vec::new();
        // 4 velocity values
        response_body.extend((50.0f32).to_le_bytes());
        response_body.extend((100.0f32).to_le_bytes());
        response_body.extend((150.0f32).to_le_bytes());
        response_body.extend((200.0f32).to_le_bytes());
        // 4 acceleration values
        response_body.extend((250.0f32).to_le_bytes());
        response_body.extend((300.0f32).to_le_bytes());
        response_body.extend((350.0f32).to_le_bytes());
        response_body.extend((400.0f32).to_le_bytes());

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_coordinate_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, [50.0, 100.0, 150.0, 200.0]);
        assert_eq!(params.acceleration, [250.0, 300.0, 350.0, 400.0]);
    }

    #[test]
    fn test_set_jog_common_params_ok_not_queued() {
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::CommonParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGCommonParams {
            velocity_ratio: 0.5,
            acceleration_ratio: 0.75,
        };
        let result = control.set_jog_common_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_jog_common_params_ok_queued() {
        let queue_idx: u64 = 789;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CommonParams),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGCommonParams {
            velocity_ratio: 1.0,
            acceleration_ratio: 1.0,
        };
        let result = control.set_jog_common_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_jog_common_params_ok() {
        let mut response_body = Vec::new();
        response_body.extend((0.8f32).to_le_bytes()); // velocity_ratio
        response_body.extend((0.9f32).to_le_bytes()); // acceleration_ratio

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CommonParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_common_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity_ratio, 0.8);
        assert_eq!(params.acceleration_ratio, 0.9);
    }

    #[test]
    fn test_set_jog_cmd_idle_not_queued() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::Cmd), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let cmd = TagJOGCmd {
            is_joint: JogMode::Coordinate,
            cmd: JogCmd::Idle,
        };
        let result = control.set_jog_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_jog_cmd_ap_down_queued() {
        let queue_idx: u64 = 999;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::Cmd),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let cmd = TagJOGCmd {
            is_joint: JogMode::Joint,
            cmd: JogCmd::ApDown,
        };
        let result = control.set_jog_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_jog_cmd_all_commands() {
        // Test all JogCmd variants
        let commands = [
            JogCmd::Idle,
            JogCmd::ApDown,
            JogCmd::AnDown,
            JogCmd::BpDown,
            JogCmd::BnDown,
            JogCmd::CpDown,
            JogCmd::CnDown,
            JogCmd::DpDown,
            JogCmd::DnDown,
        ];

        for cmd_type in commands.iter() {
            let mock_response =
                create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::Cmd), b"");
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = JOGSerialControl::new(&mutex);

            let cmd = TagJOGCmd {
                is_joint: JogMode::Coordinate,
                cmd: *cmd_type,
            };
            let result = control.set_jog_cmd(cmd, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_set_jogl_params_ok_not_queued() {
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::LParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGLParams {
            velocity: 100.0,
            acceleration: 200.0,
        };
        let result = control.set_jogl_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_jogl_params_ok_queued() {
        let queue_idx: u64 = 111;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::LParams),
            &queue_idx.to_le_bytes(),
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let params = TagJOGLParams {
            velocity: 50.0,
            acceleration: 75.0,
        };
        let result = control.set_jogl_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_jogl_params_ok() {
        let mut response_body = Vec::new();
        response_body.extend((150.0f32).to_le_bytes()); // velocity
        response_body.extend((250.0f32).to_le_bytes()); // acceleration

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::LParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jogl_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, 150.0);
        assert_eq!(params.acceleration, 250.0);
    }

    #[test]
    fn test_get_jog_joint_params_invalid_response() {
        let response_body = [0u8; 4]; // Too small, should be 32 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::JointParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_joint_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_jog_coordinate_params_invalid_response() {
        let response_body = [0u8; 16]; // Too small, should be 32 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_coordinate_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_jog_common_params_invalid_response() {
        let response_body = [0u8; 4]; // Too small, should be 8 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CommonParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_common_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_jogl_params_invalid_response() {
        let response_body = [0u8; 4]; // Too small, should be 8 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::LParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jogl_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_joint_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_jog_cmd_both_modes() {
        // Test both Coordinate and Joint modes
        let modes = [JogMode::Coordinate, JogMode::Joint];

        for mode in modes.iter() {
            let mock_response =
                create_response_packet(CommunicationProtocolIDs::Jog(JogIDs::Cmd), b"");
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = JOGSerialControl::new(&mutex);

            let cmd = TagJOGCmd {
                is_joint: *mode,
                cmd: JogCmd::CpDown,
            };
            let result = control.set_jog_cmd(cmd, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_jog_params_with_zeros() {
        let mut response_body = Vec::new();
        // All zeros
        for _ in 0..8 {
            response_body.extend((0.0f32).to_le_bytes());
        }

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::JointParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_joint_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(params.acceleration, [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_jog_common_params_extreme_values() {
        let mut response_body = Vec::new();
        response_body.extend((0.0f32).to_le_bytes()); // min velocity_ratio
        response_body.extend((100.0f32).to_le_bytes()); // high acceleration_ratio

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Jog(JogIDs::CommonParams),
            &response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = JOGSerialControl::new(&mutex);

        let result = control.get_jog_common_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity_ratio, 0.0);
        assert_eq!(params.acceleration_ratio, 100.0);
    }
}
