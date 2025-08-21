#[cfg(test)]
mod tests {

    use crate::dobot::dobot_core_serial::Dobot;
    use crate::dobot::dobot_core_serial::RwLock;
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::calibration_control::CalibrationSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::mock_command_sender::{MockCommandSender, create_response_packet},
                dobot_error::DobotError,
                sub_command_dobot::calibration_control::CalibrationControl,
            },
            protocol::{CommunicationProtocolIDs, ProtocolError, command_id::CalIDs},
        },
    };

    // This test verifies a successful call to set_angle_sensor_static_error.
    #[test]
    fn test_set_angle_sensor_static_error_ok() {
        let rear_arm_angle_error = 1.234;
        let front_arm_angle_error = 5.678;

        // The set command returns an empty response body.
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cal(CalIDs::AngleSensorStaticError),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut calibration_control = CalibrationSerialControl::new(&mutex);

        let result = calibration_control
            .set_angle_sensor_static_error(rear_arm_angle_error, front_arm_angle_error);

        // We expect the operation to be successful.
        assert!(result.is_ok());
    }

    // --- Tests for get_angle_sensor_static_error ---

    // This test verifies a successful call to get_angle_sensor_static_error.
    #[test]
    fn test_get_angle_sensor_static_error_ok() {
        let expected_rear_angle: f32 = 9.876;
        let expected_front_angle: f32 = 54.321;

        // Combine the two f32 values into a byte array.
        let mut response_buffer = [0u8; 8];
        response_buffer[..4].copy_from_slice(&expected_rear_angle.to_le_bytes());
        response_buffer[4..8].copy_from_slice(&expected_front_angle.to_le_bytes());

        // Create a mock response packet with the serialized data.
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cal(CalIDs::AngleSensorStaticError),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut calibration_control = CalibrationSerialControl::new(&mutex);

        let result = calibration_control.get_angle_sensor_static_error();

        // We expect the operation to be successful and return the correct tuple.
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (expected_rear_angle, expected_front_angle));
    }

    // This test simulates an invalid response for get_angle_sensor_static_error,
    // which should result in a ProtocolError.
    #[test]
    fn test_get_angle_sensor_static_error_invalid_response() {
        // Create a response with a payload that's too small (e.g., 4 bytes instead of 8).
        let invalid_params = [1, 2, 3, 4];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Cal(CalIDs::AngleSensorStaticError),
            &invalid_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut calibration_control = CalibrationSerialControl::new(&mutex);

        let result = calibration_control.get_angle_sensor_static_error();

        // We expect the result to be an error indicating the buffer was too small.
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // This test checks for a general communication failure from the mock sender.
    #[test]
    fn test_calibration_control_send_raw_packet_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut calibration_control = CalibrationSerialControl::new(&mutex);
        let rear_arm_angle_error = 1.0;
        let front_arm_angle_error = 2.0;

        let result = calibration_control
            .set_angle_sensor_static_error(rear_arm_angle_error, front_arm_angle_error);

        // We expect the result to be an error from the mock sender.
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
