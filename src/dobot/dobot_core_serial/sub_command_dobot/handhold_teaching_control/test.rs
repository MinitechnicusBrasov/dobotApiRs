#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::handhold_teaching_control::HandholdTeachingSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::handhold_teaching_control::HandholdTeachingControl,
            },
            protocol::{
                bodies::hht_trig_mode::HHTTrigMode,
                command_id::HHTIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_hht_trig_mode_triggered_on_key_release() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.set_hht_trig_mode(HHTTrigMode::TriggeredOnKeyRelease);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_hht_trig_mode_triggered_on_periodic_interval() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.set_hht_trig_mode(HHTTrigMode::TriggeredOnPeriodicInterval);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_hht_trig_mode_triggered_on_key_release() {
        let response_params = [0u8]; // HHTTrigMode::TriggeredOnKeyRelease
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_mode();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), HHTTrigMode::TriggeredOnKeyRelease);
    }

    #[test]
    fn test_get_hht_trig_mode_triggered_on_periodic_interval() {
        let response_params = [1u8]; // HHTTrigMode::TriggeredOnPeriodicInterval
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_mode();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), HHTTrigMode::TriggeredOnPeriodicInterval);
    }

    #[test]
    fn test_get_hht_trig_mode_invalid_value() {
        let response_params = [255u8]; // Invalid mode value
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_mode();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_hht_trig_mode_empty_response() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_mode();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_set_hht_trig_output_enabled_true() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.set_hht_trig_output_enabled(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_hht_trig_output_enabled_false() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.set_hht_trig_output_enabled(false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_hht_trig_output_enabled_true() {
        let response_params = [1u8]; // enabled
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output_enabled();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_hht_trig_output_enabled_false() {
        let response_params = [0u8]; // disabled
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output_enabled();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_get_hht_trig_output_enabled_empty_response() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output_enabled();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_hht_trig_output_true() {
        let response_params = [1u8]; // output active
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutput),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_hht_trig_output_false() {
        let response_params = [0u8]; // output inactive
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutput),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_get_hht_trig_output_empty_response() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutput),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output();
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
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_mode();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_get_hht_trig_output_nonzero_as_true() {
        let response_params = [42u8]; // any nonzero value should be true
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutput),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_hht_trig_output_enabled_nonzero_as_true() {
        let response_params = [255u8]; // any nonzero value should be true
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled),
            &response_params
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = HandholdTeachingSerialControl::new(&mutex);

        let result = control.get_hht_trig_output_enabled();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
