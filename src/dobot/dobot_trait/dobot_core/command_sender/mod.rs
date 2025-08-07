use crate::dobot::dobot_trait::{protocol::{
    bodies::general_response::GeneralResponse, Body, CommunicationProtocolIDs, Protocol, ProtocolError
}, rwlock::RwLock};

const MAX_PACKET_SIZE: usize = 32;

use core::fmt::Debug;

#[cfg(not(feature = "std"))]
struct FmtWriter<'a> {
    buffer: &'a mut [u8],
    cursor: &'a mut usize,
}

#[cfg(not(feature = "std"))]
impl<'a> FmtWriter<'a> {
    fn new(buffer: &'a mut [u8], cursor: &'a mut usize) -> Self {
        *cursor = 0;
        Self { buffer, cursor }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> Write for FmtWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let new_cursor = *self.cursor + s.len();
        if new_cursor > self.buffer.len() {
            return Err(core::fmt::Error);
        }
        self.buffer[*self.cursor..new_cursor].copy_from_slice(s.as_bytes());
        *self.cursor = new_cursor;
        Ok(())
    }
}

#[cfg(not(feature = "std"))]
pub use spin::{RwLockReadGuard, RwLockWriteGuard};

#[cfg(feature = "std")]
pub use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use super::dobot_error::DobotError;

pub trait CommandSender: Send + Sync {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError>;

    fn send_command<'a, Req: Body<'a> + 'a, Resp: Body<'a> + 'a>(
        &mut self,
        id: CommunicationProtocolIDs,
        is_read: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: &'a mut [u8],
    ) -> Result<Resp, DobotError> {
        let protocol = Protocol::new(id, is_queued, is_read, request_body);
        let mut request_buffer = [0u8; MAX_PACKET_SIZE]; // Max packet size
        let request_len = protocol
            .to_packet(&mut request_buffer)
            .map_err(|e| DobotError::Protocol(e))?;

        let mut response_temp_buffer = [0u8; MAX_PACKET_SIZE];
        let response_len =
            self.send_raw_packet(&request_buffer[..request_len], &mut response_temp_buffer)?;

        if response_buffer.len() < response_len {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        response_buffer[..response_len].copy_from_slice(&response_temp_buffer[..response_len]);

        let response_protocol =
            Protocol::<GeneralResponse>::from_packet(&response_buffer[..response_len])
                .map_err(|e| DobotError::Protocol(e))?;

        let response_body = Resp::deserialize(response_protocol.body.params)?;
        Ok(response_body)
    }

    fn get_status_str(&self, buffer: &mut [u8]) -> Result<usize, DobotError>
    where
        Self: Debug;
}

pub struct Dobot<T: CommandSender> {
    // The conditional RwLock protects the CommandSender.
    command_sender: RwLock<T>,
}

impl<T: CommandSender> Dobot<T> {
    /// Creates a new Dobot instance with a given CommandSender.
    pub fn new(sender: T) -> Self {
        Self {
            command_sender: RwLock::new(sender),
        }
    }

    /// Example of acquiring a read lock to perform a read-only operation.
    #[cfg(feature = "std")]
    pub fn get_status(&self) -> Result<String, DobotError>
    where
        T: Debug,
    {
        let sender = self.command_sender.read().map_err(|_| DobotError::SenderPoisoned)?;
        Ok(format!("Dobot status: {:?}", *sender))
    }

    #[cfg(not(feature = "std"))]
    pub fn get_status<'a, const N: usize>(&self, buffer: &'a mut [u8; N]) -> Result<&'a str, DobotError>
    where
        T: Debug,
    {
        let sender = self.command_sender.read();
        
        let mut cursor = 0;
        write!(
            FmtWriter::new(&mut buffer[..], &mut cursor),
            "Dobot status: {:?}",
            *sender
        )?;

        // Safety: We've just written a valid UTF-8 string, so this is safe.
        Ok(unsafe { core::str::from_utf8_unchecked(&buffer[..cursor]) })
    }

    /// Sends a command to the Dobot and returns a deserialized response body.
    #[cfg(feature = "std")]
    pub fn send_command<'a, Req: Body<'a> + Send + 'a, Resp: Body<'a> + 'a>(
        &self,
        id: CommunicationProtocolIDs,
        is_read: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: &'a mut [u8]
    ) -> Result<Resp, DobotError>
    where
        T: Debug,
    {
        let mut sender = self.command_sender.write().map_err(|_| DobotError::SenderPoisoned)?;
        sender.send_command(id, is_read, is_queued, request_body, response_buffer)
    }

    /// Sends a command to the Dobot and returns a deserialized response body in a no-std environment.
    #[cfg(not(feature = "std"))]
    pub fn send_command<'a, Req: Body<'a>, Resp: Body<'a>>(
        &'a self,
        id: CommunicationProtocolIDs,
        is_read: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: &'a mut [u8],
    ) -> Result<Resp, DobotError>
    where
        T: Debug,
    {
        let mut sender = self.command_sender.write();
        sender.send_command(id, is_read, is_queued, request_body, response_buffer)
    }
}


#[cfg(feature = "std")]
pub mod mock_command_sender;
