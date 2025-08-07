use core::{cell::RefCell, fmt::Debug, ops::Deref};
use std::sync::RwLock;

use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::{
        Body, CommunicationProtocolIDs, Protocol, ProtocolError,
        bodies::{general_request::GeneralRequest, general_response::GeneralResponse},
    },
};

use super::CommandSender;

pub struct MockCommandSender {
    pub canned_response: RwLock<Vec<u8>>,
    pub expected_request: RwLock<Option<Vec<u8>>>,
    pub response_len: RwLock<Result<usize, DobotError>>,
}

impl MockCommandSender {
    pub fn new(response: Vec<u8>, len: Result<usize, DobotError>) -> Self {
        MockCommandSender {
            canned_response: RwLock::new(response),
            expected_request: RwLock::new(None),
            response_len: RwLock::new(len),
        }
    }
}

impl Debug for MockCommandSender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MockCommandSender")
            .field("canned_response", &self.canned_response)
            .field("expected_request", &self.expected_request)
            .field("response_len", &self.response_len)
            .finish()
    }
}

impl CommandSender for MockCommandSender {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError> {
        // Check if the received request matches the expected one, if set.
        if let Some(expected) = self.expected_request.get_mut().unwrap().take() {
            assert_eq!(request_packet, expected.as_slice());
        }

        // Simulate writing the canned response to the buffer.
        let response_bytes = self.canned_response.get_mut().unwrap();
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
        let result = match self.response_len.get_mut() {
            Ok(x) => x,
            Err(_) => return Err(DobotError::SenderPoisoned),
        };
        result.clone()
    }

    fn get_status_str(&self, buffer: &mut [u8]) -> Result<usize, DobotError>
    where
        Self: core::fmt::Debug,
    {
        let message = format!("Mock Sender status: {:?}", self);
        if buffer.len() < message.len() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        buffer[..message.len()].copy_from_slice(message.as_str().as_bytes());
        Ok(message.len())
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
