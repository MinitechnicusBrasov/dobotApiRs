#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            wait_control::WaitControl
        ,
    }, protocol::{bodies::{
            general_request::GeneralRequest, tag_empty_body::EmptyBody, tag_queue::received::TagQueue}, command_id::WaitIDs, CommunicationProtocolIDs}, rwlock::RwLock};

pub struct WaitSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> WaitSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> WaitControl for WaitSerialControl<'a, T> {
    fn set_wait_cmd(
        &mut self,
        timeout: u32,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut request_buffer = [0u8; 4];
        request_buffer.copy_from_slice(&timeout.to_le_bytes());
        let request_body = GeneralRequest { params: &request_buffer };

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd), request_body, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Wait(WaitIDs::WaitCmd), request_body, write=true)?;
        Ok(None)
    }
}
