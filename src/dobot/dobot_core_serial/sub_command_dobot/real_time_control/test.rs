#[cfg(test)]
mod tests {
    use critical_section::Mutex;

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::real_time_control::RealTimePoseSerialControl, dobot_trait::{
            dobot_core::{
                command_sender::mock_command_sender::{create_response_packet, MockCommandSender},
                dobot_error::DobotError,
                sub_command_dobot::real_time_control::RealTimeControl,
            },
            protocol::{
                bodies::tag_pose::TagPose,
                command_id::DeviceInfoIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
        }
    };

    // Test for a successful `reset_pose` operation.
    #[test]
    fn test_reset_pose_ok() {
        let manual: u8 = 1;
        let rear_arm_angle: f32 = 1.23;
        let front_arm_angle: f32 = 4.56;

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.reset_pose(manual, rear_arm_angle, front_arm_angle);

        assert!(result.is_ok());
    }

    // Test for `reset_pose` when the underlying command sender returns an error.
    #[test]
    fn test_reset_pose_error() {
        let manual: u8 = 1;
        let rear_arm_angle: f32 = 1.23;
        let front_arm_angle: f32 = 4.56;

        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.reset_pose(manual, rear_arm_angle, front_arm_angle);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    // Test for a successful `get_pose` operation.
    #[test]
    fn test_get_pose_ok() {
        let expected_pose = TagPose {
            x: 100.0,
            y: 200.0,
            z: 300.0,
            r: 400.0,
            joint_angle: [10.0, 20.0, 30.0, 40.0],
        };
        let mut serialized_pose = Vec::new();
        serialized_pose.extend_from_slice(&expected_pose.x.to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.y.to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.z.to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.r.to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.joint_angle[0].to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.joint_angle[1].to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.joint_angle[2].to_le_bytes());
        serialized_pose.extend_from_slice(&expected_pose.joint_angle[3].to_le_bytes());

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            &serialized_pose,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.get_pose();
        println!("{:?}", result.clone().err());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_pose);
    }

    // Test for a `get_pose` operation with a malformed response.
    #[test]
    fn test_get_pose_invalid_response() {
        let malformed_response_body = [0u8; 4]; // Incorrect size for a TagPose
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            &malformed_response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.get_pose();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // Test for a successful `get_pose_rail` operation.
    #[test]
    fn test_get_pose_rail_ok() {
        let expected_rail_pose: f32 = 50.5;
        let params = expected_rail_pose.to_le_bytes();
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            &params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.get_pose_rail();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_rail_pose);
    }

    // Test for `get_pose_rail` with a response that is not the correct size.
    #[test]
    fn test_get_pose_rail_invalid_response() {
        let malformed_response_body = [0u8; 3]; // Incorrect size for an f32
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            &malformed_response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = Mutex::new(mock_sender);
        let mut device_control = RealTimePoseSerialControl::new(&mut mutex);

        let result = device_control.get_pose_rail();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }
}
