#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::arc_control::ArcSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{
                    mock_command_sender::{create_response_packet, MockCommandSender}, Dobot
                },
                dobot_error::DobotError,
                sub_command_dobot::arc_control::ArcControl,
            },
            protocol::{
                bodies::{
                    tag_arc_cmd::{Point, TagARCCmd},
                    tag_arc_params::TagARCParams,
                    tag_queue::received::TagQueue,
                }, command_id::ArcIDs, Body, CommunicationProtocolIDs, ProtocolError
            },
            rwlock::RwLock,
        },
    };

    #[test]
    fn test_set_arc_params_ok() {
        let params = TagARCParams {
            xyz_velocity: 0.0,
            r_velocity: 0.0,
            xyz_acceleration: 0.0,
            r_acceleration: 0.0,
        };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Arc(ArcIDs::ArcParams), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.set_arc_params(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_arc_params with queuing.
    #[test]
    fn test_set_arc_params_queued_ok() {
        let params = TagARCParams {
            xyz_velocity: 0.0,
            r_velocity: 0.0,
            xyz_acceleration: 0.0,
            r_acceleration: 0.0,
        };
        let expected_queue_id: u64 = 123;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        mock_queue_response.serialize(&mut response_buffer).unwrap();

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Arc(ArcIDs::ArcParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.set_arc_params(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }

    // --- Tests for get_arc_params ---

    // This test verifies a successful call to get_arc_params.
    #[test]
    fn test_get_arc_params_ok() {
        let expected_params = TagARCParams {
            xyz_velocity: 10.0,
            r_velocity: 20.0,
            xyz_acceleration: 30.0,
            r_acceleration: 40.0,
        };
        let mut response_buffer = [0u8; 16]; // 4 * f32 + 1 u8
        expected_params.serialize(&mut response_buffer).unwrap();
        
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Arc(ArcIDs::ArcParams),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.get_arc_params();
        assert!(result.is_ok());
        let result_params = result.unwrap();
        assert_eq!(result_params.xyz_velocity, expected_params.xyz_velocity);
        assert_eq!(result_params.xyz_acceleration, expected_params.xyz_acceleration);
        assert_eq!(result_params.r_velocity, expected_params.r_velocity);
        assert_eq!(result_params.r_acceleration, expected_params.r_acceleration);
    }

    // This test simulates an invalid response for get_arc_params.
    #[test]
    fn test_get_arc_params_invalid_response() {
        let invalid_params = [1, 2, 3];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Arc(ArcIDs::ArcParams),
            &invalid_params,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.get_arc_params();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    // --- Tests for set_arc_cmd ---

    // This test verifies a successful call to set_arc_cmd without queuing.
    #[test]
    fn test_set_arc_cmd_ok() {
        let to_point = Point {x: 0., y: 0., z: 0., r: 0.};
        let mid_point = Point {x: 10., y: 10., z: 10., r: 10.};
        let cmd = TagARCCmd { to_point, circ_point: mid_point };
        let mock_response =
            create_response_packet(CommunicationProtocolIDs::Arc(ArcIDs::ArcCmd), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.set_arc_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    // This test verifies a successful call to set_arc_cmd with queuing.
    #[test]
    fn test_set_arc_cmd_queued_ok() {
        let to_point = Point {x: 0., y: 0., z: 0., r: 0.};
        let mid_point = Point {x: 10., y: 10., z: 10., r: 10.};
        let cmd = TagARCCmd { to_point, circ_point: mid_point };
        let expected_queue_id: u64 = 456;
        let mock_queue_response = TagQueue {
            queue_idx: expected_queue_id,
        };
        let mut response_buffer = [0u8; 8];
        mock_queue_response.serialize(&mut response_buffer).unwrap();
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Arc(ArcIDs::ArcCmd),
            &response_buffer,
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);

        let result = arc_control.set_arc_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(expected_queue_id));
    }
    
    // This test checks for a general communication failure from the mock sender.
    #[test]
    fn test_arc_control_send_raw_packet_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mut mutex = create_mock_sender_lock!(mock_sender);
        let mut arc_control = ArcSerialControl::new(&mut mutex);
        let params = TagARCParams {
            xyz_velocity: 0.,
            xyz_acceleration: 9.,
            r_velocity: 0.,
            r_acceleration: 1.
        };

        let result = arc_control.set_arc_params(params, false);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
