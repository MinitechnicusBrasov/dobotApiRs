use core::fmt::Debug;
use std::{sync::{Arc, Mutex}, time};

use serialport::SerialPort;

use crate::dobot::dobot_trait::{
    dobot_core::{command_sender::CommandSender, dobot_error::DobotError},
    protocol::ProtocolError,
};

pub struct DobotCommandSender {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl DobotCommandSender {
    pub fn new(port_name: &str) -> Result<Self, DobotError> {
        let port = serialport::new(port_name, 115200)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(std::time::Duration::from_secs(2))
            .open()
            .map_err(|_e| DobotError::Serial)?;
        port.clear(serialport::ClearBuffer::All)?;
        Ok(Self {
            port: Arc::new(Mutex::new(port)),
        })
    }
}

impl Debug for DobotCommandSender {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DobotCommandSender")
            .field("port", &self.port)
            .finish()
    }
}

impl CommandSender for DobotCommandSender {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError> {
        println!("Sending packet: {:?}", request_packet);
        let mut serial_port = match self.port.lock() {
            Ok(x) => x,
            Err(_) => return Err(DobotError::SenderPoisoned),
        };
        println!("Test 2");
        serial_port
            .write_all(request_packet)
            .map_err(|_e| DobotError::IO)?;
        println!("Test3");

        // Read response. This is a simplified implementation. Real-world might need to read byte-by-byte
        // until a full packet is received (e.g., check for 0xAA 0xAA start bytes).
        println!("Test4");
        let bytes_to_read = serial_port.bytes_to_read();
        println!("Bytes to read: {}", bytes_to_read.unwrap());
        
        let bytes_read = serial_port
            .read(response_buffer);
        println!("Test5");
        if bytes_read.is_err() {
            println!("Error: {:?}", bytes_read.as_ref().err());
        }

        Ok(bytes_read.unwrap())
    }

    fn get_status_str(&self, buffer: &mut [u8]) -> Result<usize, DobotError>
    where
        Self: core::fmt::Debug,
    {
        let message = format!("Command serial sender: {:?}", self);
        if buffer.len() < message.len() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        buffer[..message.len()].copy_from_slice(message.as_str().as_bytes());

        Ok(message.len())
    }
}
