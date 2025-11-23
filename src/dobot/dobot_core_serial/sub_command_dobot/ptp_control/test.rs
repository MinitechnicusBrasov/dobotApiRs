#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::ptp_control::PTPSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::ptp_control::PTPControl,
            },
            protocol::{
                bodies::{
                    tag_po_cmd::TagPOCmd,
                    tag_ptp_cmd::{PTPMode, TagPTPCmd},
                    tag_ptp_common_params::TagPTPCommonParams,
                    tag_ptp_coordinate_params::TagPTPCoordinateParams,
                    tag_ptp_joint_params::TagPTPJointParams,
                    tag_ptp_jump2_params::TagPTPJump2Params,
                    tag_ptp_jump_params::TagPTPJumpParams,
                    tag_ptp_with_l_cmd::TagPTPWithLCmd,
                    tag_ptpl_params::TagPTPLParams,
                },
                command_id::PtpIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    // Helper function to create float array response
    fn create_float_array_response(values: &[f32]) -> Vec<u8> {
        let mut response = Vec::new();
        for &value in values {
            response.extend(value.to_le_bytes());
        }
        response
    }

    #[test]
    fn test_get_ptp_joint_params_ok() {
        let values = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JointParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_joint_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, [10.0, 20.0, 30.0, 40.0]);
        assert_eq!(params.acceleration, [50.0, 60.0, 70.0, 80.0]);
    }

    #[test]
    fn test_set_ptp_joint_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JointParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJointParams {
            velocity: [100.0, 150.0, 200.0, 250.0],
            acceleration: [300.0, 350.0, 400.0, 450.0],
        };
        let result = control.set_ptp_joint_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_joint_params_ok_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JointParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJointParams {
            velocity: [10.0, 20.0, 30.0, 40.0],
            acceleration: [50.0, 60.0, 70.0, 80.0],
        };
        let result = control.set_ptp_joint_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_ptp_coordinate_params_ok() {
        let values = [100.0, 200.0, 300.0, 400.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_coordinate_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.xyz_velocity, 100.0);
        assert_eq!(params.r_velocity, 200.0);
        assert_eq!(params.xyz_acceleration, 300.0);
        assert_eq!(params.r_acceleration, 400.0);
    }

    #[test]
    fn test_set_ptp_coordinate_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPCoordinateParams {
            xyz_velocity: 50.0,
            r_velocity: 75.0,
            xyz_acceleration: 100.0,
            r_acceleration: 125.0,
        };
        let result = control.set_ptp_coordinate_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_coordinate_params_ok_queued() {
        let queue_idx: u64 = 456;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPCoordinateParams {
            xyz_velocity: 25.5,
            r_velocity: 30.5,
            xyz_acceleration: 35.5,
            r_acceleration: 40.5,
        };
        let result = control.set_ptp_coordinate_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_ptp_jump_params_ok() {
        let values = [50.0, 150.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_jump_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.jump_height, 50.0);
        assert_eq!(params.z_limit, 150.0);
    }

    #[test]
    fn test_set_ptp_jump_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJumpParams {
            jump_height: 100.0,
            z_limit: 200.0,
        };
        let result = control.set_ptp_jump_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_jump_params_ok_queued() {
        let queue_idx: u64 = 789;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJumpParams {
            jump_height: 75.0,
            z_limit: 125.0,
        };
        let result = control.set_ptp_jump_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_ptp_common_params_ok() {
        let values = [0.5, 0.75];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_common_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity_ratio, 0.5);
        assert_eq!(params.acceleration_ratio, 0.75);
    }

    #[test]
    fn test_set_ptp_common_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CommonParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPCommonParams {
            velocity_ratio: 0.8,
            acceleration_ratio: 0.9,
        };
        let result = control.set_ptp_common_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_common_params_ok_queued() {
        let queue_idx: u64 = 111;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CommonParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPCommonParams {
            velocity_ratio: 1.0,
            acceleration_ratio: 1.0,
        };
        let result = control.set_ptp_common_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_ptp_cmd_jump_xyz_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::Cmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let cmd = TagPTPCmd {
            ptp_mode: PTPMode::JumpXyz,
            x: 100.0,
            y: 200.0,
            z: 300.0,
            r: 45.0,
        };
        let result = control.set_ptp_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_cmd_movl_xyz_queued() {
        let queue_idx: u64 = 999;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::Cmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let cmd = TagPTPCmd {
            ptp_mode: PTPMode::MovlXyz,
            x: 50.0,
            y: 100.0,
            z: 150.0,
            r: 90.0,
        };
        let result = control.set_ptp_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_ptp_cmd_all_modes() {
        let modes = [
            PTPMode::JumpXyz,
            PTPMode::MovjXyz,
            PTPMode::MovlXyz,
            PTPMode::JumpAngle,
            PTPMode::MovjAngle,
            PTPMode::MovlAngle,
            PTPMode::MovjInc,
            PTPMode::MovlInc,
            PTPMode::MovjXyzInc,
            PTPMode::JumpMovlXyz,
        ];

        for mode in modes.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Ptp(PtpIDs::Cmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = PTPSerialControl::new(&mutex);

            let cmd = TagPTPCmd {
                ptp_mode: *mode,
                x: 10.0,
                y: 20.0,
                z: 30.0,
                r: 40.0,
            };
            let result = control.set_ptp_cmd(cmd, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_get_ptpl_params_ok() {
        let values = [100.0, 200.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::LParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptpl_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, 100.0);
        assert_eq!(params.acceleration, 200.0);
    }

    #[test]
    fn test_set_ptpl_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::LParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPLParams {
            velocity: 150.0,
            acceleration: 250.0,
        };
        let result = control.set_ptpl_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptpl_params_ok_queued() {
        let queue_idx: u64 = 222;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::LParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPLParams {
            velocity: 75.0,
            acceleration: 125.0,
        };
        let result = control.set_ptpl_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_ptp_with_rail_cmd_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::WithLCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let cmd = TagPTPWithLCmd {
            ptp_mode: PTPMode::JumpXyz,
            x: 100.0,
            y: 200.0,
            z: 300.0,
            r: 45.0,
            l: 500.0,
        };
        let result = control.set_ptp_with_rail_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_with_rail_cmd_ok_queued() {
        let queue_idx: u64 = 333;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::WithLCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let cmd = TagPTPWithLCmd {
            ptp_mode: PTPMode::MovlXyz,
            x: 50.0,
            y: 100.0,
            z: 150.0,
            r: 90.0,
            l: 250.0,
        };
        let result = control.set_ptp_with_rail_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_ptp_jump2_params_ok() {
        let values = [10.0, 20.0, 30.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_jump2_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.start_jump_height, 10.0);
        assert_eq!(params.end_jump_height, 20.0);
        assert_eq!(params.z_limit, 30.0);
    }

    #[test]
    fn test_set_ptp_jump2_params_ok_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJump2Params {
            start_jump_height: 50.0,
            end_jump_height: 75.0,
            z_limit: 100.0,
        };
        let result = control.set_ptp_jump2_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_jump2_params_ok_queued() {
        let queue_idx: u64 = 444;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let params = TagPTPJump2Params {
            start_jump_height: 25.0,
            end_jump_height: 35.0,
            z_limit: 45.0,
        };
        let result = control.set_ptp_jump2_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_ptp_po_cmd_single_po_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::PoCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let ptp_cmd = TagPTPCmd {
            ptp_mode: PTPMode::JumpXyz,
            x: 100.0,
            y: 200.0,
            z: 300.0,
            r: 45.0,
        };
        let po_cmds = vec![
            TagPOCmd {
                ratio: 50,
                address: 1,
                level: 1,
            }
        ];
        let result = control.set_ptp_po_cmd(ptp_cmd, &po_cmds, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_po_cmd_multiple_po_queued() {
        let queue_idx: u64 = 555;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::PoCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let ptp_cmd = TagPTPCmd {
            ptp_mode: PTPMode::MovlXyz,
            x: 50.0,
            y: 100.0,
            z: 150.0,
            r: 90.0,
        };
        let po_cmds = vec![
            TagPOCmd {
                ratio: 25,
                address: 1,
                level: 0,
            },
            TagPOCmd {
                ratio: 75,
                address: 2,
                level: 1,
            }
        ];
        let result = control.set_ptp_po_cmd(ptp_cmd, &po_cmds, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_ptp_po_with_rail_cmd_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::PoWithLCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let ptp_cmd = TagPTPWithLCmd {
            ptp_mode: PTPMode::JumpXyz,
            x: 100.0,
            y: 200.0,
            z: 300.0,
            r: 45.0,
            l: 500.0,
        };
        let po_cmds = vec![
            TagPOCmd {
                ratio: 50,
                address: 1,
                level: 1,
            }
        ];
        let result = control.set_ptp_po_with_rail_cmd(ptp_cmd, &po_cmds, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_ptp_po_with_rail_cmd_queued() {
        let queue_idx: u64 = 666;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::PoWithLCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let ptp_cmd = TagPTPWithLCmd {
            ptp_mode: PTPMode::MovlXyz,
            x: 50.0,
            y: 100.0,
            z: 150.0,
            r: 90.0,
            l: 250.0,
        };
        let po_cmds = vec![
            TagPOCmd {
                ratio: 30,
                address: 3,
                level: 0,
            },
            TagPOCmd {
                ratio: 70,
                address: 4,
                level: 1,
            }
        ];
        let result = control.set_ptp_po_with_rail_cmd(ptp_cmd, &po_cmds, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_joint_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_get_ptp_joint_params_invalid_response() {
        let response_body = [0u8; 16]; // Too small, should be 32 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JointParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_joint_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_ptp_coordinate_params_invalid_response() {
        let response_body = [0u8; 8]; // Too small, should be 16 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_coordinate_params();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_ptp_cmd_negative_coordinates() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::Cmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let cmd = TagPTPCmd {
            ptp_mode: PTPMode::JumpXyz,
            x: -100.0,
            y: -200.0,
            z: -50.0,
            r: -45.0,
        };
        let result = control.set_ptp_cmd(cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ptp_params_with_zeros() {
        let values = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let response_body = create_float_array_response(&values);
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Ptp(PtpIDs::JointParams),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = PTPSerialControl::new(&mutex);

        let result = control.get_ptp_joint_params();
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.velocity, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(params.acceleration, [0.0, 0.0, 0.0, 0.0]);
    }
}
