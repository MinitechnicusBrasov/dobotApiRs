#[cfg(test)]
mod tests {

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::wait_control::WaitSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::wait_control::WaitControl,
            },
            protocol::{
                command_id::WaitIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_wait_cmd_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(1000, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(2000, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_wait_cmd_zero_timeout_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(0, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_zero_timeout_queued() {
        let queue_idx: u64 = 456;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(0, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_wait_cmd_max_timeout_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(u32::MAX, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_max_timeout_queued() {
        let queue_idx: u64 = 789;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(u32::MAX, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_wait_cmd_short_timeout() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(100, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_medium_timeout() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(5000, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_long_timeout() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(60000, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_one_second() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        // Assuming timeout is in milliseconds
        let result = control.set_wait_cmd(1000, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_wait_cmd_queued_various_queue_indices() {
        let queue_indices = [0u64, 1, 100, 1000, 10000, u64::MAX];

        for queue_idx in queue_indices.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
                &queue_idx.to_le_bytes()
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WaitSerialControl::new(&mutex);

            let result = control.set_wait_cmd(1000, true);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(*queue_idx));
        }
    }

    #[test]
    fn test_set_wait_cmd_communication_error_not_queued() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WaitSerialControl::new(&mutex);

        let result = control.set_wait_cmd(1000, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_set_wait_cmd_various_timeouts() {
        let timeouts = [1, 10, 50, 100, 250, 500, 750, 1000, 2500, 5000, 10000];

        for timeout in timeouts.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WaitSerialControl::new(&mutex);

            let result = control.set_wait_cmd(*timeout, false);
            assert!(result.is_ok(), "Failed for timeout: {}", timeout);
            assert_eq!(result.unwrap(), None);
        }
    }

    #[test]
    fn test_set_wait_cmd_typical_workflow() {
        // Test a typical workflow of multiple wait commands
        
        // Wait 1: Short delay, not queued
        let mock_response1 = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            b""
        );
        let length1 = mock_response1.len();
        let mock_sender1 = MockCommandSender::new(mock_response1, Ok(length1));
        let mutex1 = create_mock_sender_lock!(mock_sender1);
        let mut control1 = WaitSerialControl::new(&mutex1);
        let result1 = control1.set_wait_cmd(500, false);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), None);

        // Wait 2: Medium delay, queued
        let queue_idx2: u64 = 1;
        let mock_response2 = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            &queue_idx2.to_le_bytes()
        );
        let length2 = mock_response2.len();
        let mock_sender2 = MockCommandSender::new(mock_response2, Ok(length2));
        let mutex2 = create_mock_sender_lock!(mock_sender2);
        let mut control2 = WaitSerialControl::new(&mutex2);
        let result2 = control2.set_wait_cmd(2000, true);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Some(queue_idx2));

        // Wait 3: Long delay, queued
        let queue_idx3: u64 = 2;
        let mock_response3 = create_response_packet(
            CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
            &queue_idx3.to_le_bytes()
        );
        let length3 = mock_response3.len();
        let mock_sender3 = MockCommandSender::new(mock_response3, Ok(length3));
        let mutex3 = create_mock_sender_lock!(mock_sender3);
        let mut control3 = WaitSerialControl::new(&mutex3);
        let result3 = control3.set_wait_cmd(5000, true);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), Some(queue_idx3));
    }

    #[test]
    fn test_set_wait_cmd_boundary_values() {
        // Test boundary values around common timeout ranges
        let boundary_timeouts = [
            0,           // Minimum
            1,           // Minimum + 1
            255,         // u8 max
            256,         // u8 max + 1
            65535,       // u16 max
            65536,       // u16 max + 1
            16777215,    // u24 max
            16777216,    // u24 max + 1
            u32::MAX - 1,
            u32::MAX,    // Maximum
        ];

        for timeout in boundary_timeouts.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WaitSerialControl::new(&mutex);

            let result = control.set_wait_cmd(*timeout, false);
            assert!(result.is_ok(), "Failed for timeout: {}", timeout);
        }
    }
}
