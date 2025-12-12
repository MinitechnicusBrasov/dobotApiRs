#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::alarm_control::AlarmSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::alarm_control::AlarmControl,
            },
            protocol::{
                alarm::Alarm, command_id::AlarmIDs, CommunicationProtocolIDs, ProtocolError
            }, rwlock::RwLock,
        },
    };

    // Test for a successful `get_active_alarms` operation.
    #[test]
    fn test_get_active_alarms_ok() {
        // Simulate a response where alarm 0 and alarm 9 are active.
        // Byte 0: 0b00000001 (Alarm 0 active)
        // Byte 1: 0b00000010 (Alarm 9 active, which is 8 + 1)
        let mut mock_response_body = [0u8; 16];
        mock_response_body[0] = 0b00000001; // Alarm 0
        mock_response_body[2] = 0b0000001; // Alarm 9 (index 1 * 8 + bit 1)

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Alarm(AlarmIDs::GetAlarmState),
            &mock_response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut alarm_control = AlarmSerialControl::new(&mutex);

        let result = alarm_control.get_active_alarms();

        assert!(result.is_ok());
        let alarms = result.unwrap();

        // Check specific alarms that should be active
        assert_eq!(alarms[0], Some(Alarm::CommonResetting)); // Alarm 0
        assert_eq!(alarms[16], Some(Alarm::PlanInvSingularity)); // This is the default for byte 1, but bit 1 (alarm 9) is checked below
        // Check that other alarms are the default `CommonResetting`
        for (i, alarm) in alarms.iter().enumerate() {
            if i != 0 && i != 16 {
                assert!(alarm.is_none());
            }
        }
    }

    // Test for `get_active_alarms` when the response buffer is too small.
    #[test]
    fn test_get_active_alarms_buffer_too_small() {
        let mock_response_body = [0u8; 10]; // Smaller than 16 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Alarm(AlarmIDs::GetAlarmState),
            &mock_response_body,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut alarm_control = AlarmSerialControl::new(&mut mutex);

        let result = alarm_control.get_active_alarms();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // Test for `get_active_alarms` when the underlying command sender returns an error.
    #[test]
    fn test_get_active_alarms_send_raw_packet_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut alarm_control = AlarmSerialControl::new(&mut mutex);
        let result = alarm_control.get_active_alarms();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    // Test for a successful `clear_all_alarms_state` operation.
    #[test]
    fn test_clear_all_alarms_state_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Alarm(AlarmIDs::ClearAlarmState),
            b"",
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut alarm_control = AlarmSerialControl::new(&mut mutex);

        let result = alarm_control.clear_all_alarms_state();

        assert!(result.is_ok());
    }

    // Test for `clear_all_alarms_state` when the underlying command sender returns an error.
    #[test]
    fn test_clear_all_alarms_state_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut alarm_control = AlarmSerialControl::new(&mut mutex);

        let result = alarm_control.clear_all_alarms_state();

        assert!(result.is_err());
        let err = result.unwrap_err();
        println!("{}", err);
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
