use crate::dobot::dobot_trait::protocol::{
    Body, CommunicationProtocolIDs, Protocol, bodies::general_response::GeneralResponse,
};

use super::dobot_error::DobotError;

pub trait CommandSender {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError>;

    fn send_command_with_params<'a, T: Body<'a>>(
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

        // let mut response_buffer = [0u8; 128];
        let response_len = self.send_raw_packet(&request_buffer[..request_len], response_buffer)?;

        let response_protocol =
            Protocol::<GeneralResponse>::from_packet(&response_buffer[..response_len])
                .map_err(|e| DobotError::Protocol(e))?;

        Ok(response_protocol.body)
    }
}
