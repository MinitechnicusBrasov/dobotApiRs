#[cfg(test)]
mod tests {

use crate::dobot::{
        dobot_core_serial::sub_command_dobot::losing_step_detection_control::LosingStepDetectionSerialControl, dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::losing_step_control::LosingStepControl,
            },
            protocol::{
                command_id::LostStepIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_lost_step_params_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(10.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_params_zero() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(0.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_params_negative() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(-5.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_params_large_value() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(1000.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_params_fractional() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(0.123456);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_cmd_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_cmd(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_lost_step_cmd_queued() {
        let queue_idx: u64 = 42;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_cmd(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_lost_step_cmd_queued_large_index() {
        let queue_idx: u64 = 999999;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_cmd(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_lost_step_params_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(10.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_lost_step_cmd_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_cmd(false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_lost_step_params_max_float() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(f32::MAX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_lost_step_params_min_float() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_params(f32::MIN);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sequential_operations() {
        // Test setting params followed by setting command
        let mock_response1 = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
            b""
        );
        let length1 = mock_response1.len();
        let mock_sender1 = MockCommandSender::new(mock_response1, Ok(length1));
        let mutex1 = create_mock_sender_lock!(mock_sender1);
        let mut control1 = LosingStepDetectionSerialControl::new(&mutex1);

        let result1 = control1.set_lost_step_params(25.0);
        assert!(result1.is_ok());

        let queue_idx: u64 = 100;
        let mock_response2 = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd),
            &queue_idx.to_le_bytes()
        );
        let length2 = mock_response2.len();
        let mock_sender2 = MockCommandSender::new(mock_response2, Ok(length2));
        let mutex2 = create_mock_sender_lock!(mock_sender2);
        let mut control2 = LosingStepDetectionSerialControl::new(&mutex2);

        let result2 = control2.set_lost_step_cmd(true);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_lost_step_cmd_queued_zero_index() {
        let queue_idx: u64 = 0;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = LosingStepDetectionSerialControl::new(&mutex);

        let result = control.set_lost_step_cmd(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(0));
    }

    #[test]
    fn test_set_lost_step_params_special_float_values() {
        // Test with various special float values
        let test_values = [
            1.0,
            -1.0,
            0.1,
            -0.1,
            100.5,
            -100.5,
        ];

        for value in test_values.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = LosingStepDetectionSerialControl::new(&mutex);

            let result = control.set_lost_step_params(*value);
            assert!(result.is_ok(), "Failed for value: {}", value);
        }
    }
}
