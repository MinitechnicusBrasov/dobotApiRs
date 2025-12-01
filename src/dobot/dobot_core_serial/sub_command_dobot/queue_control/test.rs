#[cfg(test)]
mod tests {

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::queue_control::QueueSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::queue_control::QueueControl,
            },
            protocol::{
                command_id::QueuedCmdIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_queued_cmd_force_stop_exec_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopExec),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_force_stop_exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(10, 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_zero_values() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(0, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_large_values() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(u32::MAX, u32::MAX);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_typical_values() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(5, 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_stop_download_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_stop_download();
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_exec_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartExec),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_stop_exec_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopExec),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_stop_exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_clear_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::Clear),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_clear();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_queued_cmd_current_index_ok() {
        let current_idx: u64 = 42;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            &current_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current_idx);
    }

    #[test]
    fn test_get_queued_cmd_current_index_zero() {
        let current_idx: u64 = 0;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            &current_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_get_queued_cmd_current_index_large_value() {
        let current_idx: u64 = 999999;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            &current_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current_idx);
    }

    #[test]
    fn test_get_queued_cmd_current_index_max_value() {
        let current_idx: u64 = u64::MAX;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            &current_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), u64::MAX);
    }

    #[test]
    fn test_get_queued_cmd_current_index_buffer_too_small() {
        let response_body = [0u8; 4]; // Too small, should be 8 bytes
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_get_queued_cmd_current_index_empty_response() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_set_queued_cmd_force_stop_exec_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_force_stop_exec();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_queued_cmd_stop_download_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_stop_download();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_queued_cmd_start_exec_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_exec();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_queued_cmd_clear_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_clear();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_get_queued_cmd_current_index_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.get_queued_cmd_current_index();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_queue_workflow_sequence() {
        // Test typical workflow: start download -> start exec -> stop exec -> clear
        
        // 1. Start download
        let mock_response1 = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length1 = mock_response1.len();
        let mock_sender1 = MockCommandSender::new(mock_response1, Ok(length1));
        let mutex1 = create_mock_sender_lock!(mock_sender1);
        let mut control1 = QueueSerialControl::new(&mutex1);
        let result1 = control1.set_queued_cmd_start_download(3, 500);
        assert!(result1.is_ok());

        // 2. Stop download
        let mock_response2 = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopDownload),
            b""
        );
        let length2 = mock_response2.len();
        let mock_sender2 = MockCommandSender::new(mock_response2, Ok(length2));
        let mutex2 = create_mock_sender_lock!(mock_sender2);
        let mut control2 = QueueSerialControl::new(&mutex2);
        let result2 = control2.set_queued_cmd_stop_download();
        assert!(result2.is_ok());

        // 3. Start exec
        let mock_response3 = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartExec),
            b""
        );
        let length3 = mock_response3.len();
        let mock_sender3 = MockCommandSender::new(mock_response3, Ok(length3));
        let mutex3 = create_mock_sender_lock!(mock_sender3);
        let mut control3 = QueueSerialControl::new(&mutex3);
        let result3 = control3.set_queued_cmd_start_exec();
        assert!(result3.is_ok());

        // 4. Stop exec
        let mock_response4 = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopExec),
            b""
        );
        let length4 = mock_response4.len();
        let mock_sender4 = MockCommandSender::new(mock_response4, Ok(length4));
        let mutex4 = create_mock_sender_lock!(mock_sender4);
        let mut control4 = QueueSerialControl::new(&mutex4);
        let result4 = control4.set_queued_cmd_stop_exec();
        assert!(result4.is_ok());

        // 5. Clear
        let mock_response5 = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::Clear),
            b""
        );
        let length5 = mock_response5.len();
        let mock_sender5 = MockCommandSender::new(mock_response5, Ok(length5));
        let mutex5 = create_mock_sender_lock!(mock_sender5);
        let mut control5 = QueueSerialControl::new(&mutex5);
        let result5 = control5.set_queued_cmd_clear();
        assert!(result5.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_single_loop() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(1, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_queued_cmd_start_download_many_lines() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = QueueSerialControl::new(&mutex);

        let result = control.set_queued_cmd_start_download(100, 10000);
        assert!(result.is_ok());
    }
}
