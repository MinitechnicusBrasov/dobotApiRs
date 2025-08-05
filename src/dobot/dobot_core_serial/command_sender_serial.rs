use serialport::SerialPort;

use crate::dobot::dobot_trait::{
    dobot_core::{command_sender::CommandSender, dobot_error::DobotError},
    protocol::{
        Body, CommunicationProtocolIDs, Protocol, bodies::general_response::GeneralResponse,
    },
};

pub struct DobotCommandSender {
    port: Box<dyn SerialPort>,
}

impl DobotCommandSender {
    pub fn new(port_name: &str) -> Result<Self, DobotError> {
        let port = serialport::new(port_name, 115200)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .map_err(|_e| DobotError::Serial)?;
        Ok(Self { port })
    }

    pub fn send_command_with_params<'a, T: Body<'a>>(
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

impl CommandSender for DobotCommandSender {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError> {
        self.port
            .write_all(request_packet)
            .map_err(|_e| DobotError::IO)?;

        // Read response. This is a simplified implementation. Real-world might need to read byte-by-byte
        // until a full packet is received (e.g., check for 0xAA 0xAA start bytes).
        let bytes_read = self
            .port
            .read(response_buffer)
            .map_err(|_e| DobotError::IO)?;

        Ok(bytes_read)
    }
}
