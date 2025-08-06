use crate::dobot::dobot_trait::protocol::{
    Body, CommunicationProtocolIDs, Protocol, ProtocolError,
    bodies::general_response::GeneralResponse,
};

use super::dobot_error::DobotError;

pub trait CommandSender {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError>;

    fn send_command_with_params<'a, T: Body<'a> + 'a>(
        &'a mut self,
        id: CommunicationProtocolIDs,
        is_read: bool,
        params: T,
        response_buffer: &'a mut [u8],
    ) -> Result<GeneralResponse<'a>, DobotError> {
        let protocol = Protocol::new(id, false, is_read, params);
        let mut request_buffer = [0u8; 128]; // Max packet size
        let request_len = protocol
            .to_packet(&mut request_buffer)
            .map_err(|e| DobotError::Protocol(e))?;

        let mut response_temp_buffer = [0u8; 128];
        let response_len =
            self.send_raw_packet(&request_buffer[..request_len], &mut response_temp_buffer)?;

        let response_protocol =
            Protocol::<GeneralResponse>::from_packet(&response_temp_buffer[..response_len])
                .map_err(|e| DobotError::Protocol(e))?;

        if response_buffer.len() < response_protocol.body.params.len() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        response_buffer[..response_protocol.body.params.len()]
            .copy_from_slice(response_protocol.body.params);
        let response_body = GeneralResponse {
            params: &response_buffer[..response_protocol.body.params.len()],
        };
        Ok(response_body)
    }
}

#[cfg(feature = "std")]
pub mod mock_command_sender {
    use core::{cell::RefCell, ops::Deref};

    use crate::dobot::dobot_trait::{
        dobot_core::dobot_error::DobotError,
        protocol::{
            Body, CommunicationProtocolIDs, Protocol, ProtocolError,
            bodies::{general_request::GeneralRequest, general_response::GeneralResponse},
        },
    };

    use super::CommandSender;

    pub struct MockCommandSender {
        pub canned_response: RefCell<Vec<u8>>,
        pub expected_request: RefCell<Option<Vec<u8>>>,
        pub response_len: RefCell<Result<usize, DobotError>>,
    }

    impl MockCommandSender {
        pub fn new(response: Vec<u8>, len: Result<usize, DobotError>) -> Self {
            MockCommandSender {
                canned_response: RefCell::new(response),
                expected_request: RefCell::new(None),
                response_len: RefCell::new(len),
            }
        }
    }

    impl CommandSender for MockCommandSender {
        fn send_raw_packet(
            &mut self,
            request_packet: &[u8],
            response_buffer: &mut [u8],
        ) -> Result<usize, DobotError> {
            // Check if the received request matches the expected one, if set.
            if let Some(expected) = self.expected_request.borrow_mut().take() {
                assert_eq!(request_packet, expected.as_slice());
            }

            // Simulate writing the canned response to the buffer.
            let response_bytes = self.canned_response.borrow();
            let len = response_bytes.len();
            println!(
                "Response bytes: {:?}\nResponse buffer: {:?}",
                &response_bytes, &response_buffer
            );
            if response_buffer.len() < len {
                return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
            }
            response_buffer[..len].copy_from_slice(&response_bytes);

            // Return the pre-configured result.
            self.response_len.borrow_mut().deref().clone()
        }
    }

    pub fn create_response_packet(id: CommunicationProtocolIDs, params: &[u8]) -> Vec<u8> {
        let body = GeneralRequest { params };
        let protocol = Protocol::new(id, false, true, body);
        let mut buffer = [0u8; 128];
        let len = protocol.to_packet(&mut buffer).unwrap();
        buffer[..len].to_vec()
    }

    // Helper function to create a request packet for assertion purposes.
    pub fn create_request_packet<'a, T: Body<'a> + 'a>(
        id: CommunicationProtocolIDs,
        is_read: bool,
        params: T,
    ) -> Vec<u8> {
        let protocol = Protocol::new(id, false, is_read, params);
        let mut buffer = [0u8; 128];
        let len = protocol.to_packet(&mut buffer).unwrap();
        buffer[..len].to_vec()
    }
}
