#[cfg(feature = "std")]
mod test;

use core::mem;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            queue_control::QueueControl
        ,
    },
    protocol::{
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse, tag_empty_body::EmptyBody
        }, command_id::QueuedCmdIDs, CommunicationProtocolIDs, ProtocolError
    },
    rwlock::RwLock,
};

pub struct QueueSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> QueueSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> QueueControl for QueueSerialControl<'a, T> {
    fn set_queued_cmd_force_stop_exec(&mut self) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopExec), EmptyBody {  }, write=true)?;
        Ok(())
    }

    fn set_queued_cmd_start_download(
        &mut self,
        total_loop: u32,
        line_per_loop: u32,
    ) -> Result<(), DobotError> {
        let mut request_buffer = [0u8; 2 * mem::size_of::<u32>()];
        request_buffer[0..mem::size_of::<u32>()].copy_from_slice(&total_loop.to_le_bytes());
        request_buffer[mem::size_of::<u32>()..2*mem::size_of::<u32>()].copy_from_slice(&line_per_loop.to_le_bytes());
        
        let params = GeneralRequest { params: &request_buffer};

let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartDownload), params, write=true)?;
        Ok(())
    }

    fn set_queued_cmd_stop_download(&mut self) -> Result<(), DobotError> {
let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopDownload), EmptyBody {  }, write=true)?;
        Ok(())
    }

    fn set_queued_cmd_start_exec(&mut self) -> Result<(), DobotError> {
let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StartExec), EmptyBody {  }, write=true)?;
        Ok(())
    }

    fn set_queued_cmd_stop_exec(&mut self) -> Result<(), DobotError> {
let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::StopExec), EmptyBody {  }, write=true)?;
        Ok(())
    }

    fn set_queued_cmd_clear(&mut self) -> Result<(), DobotError> {
let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::Clear), EmptyBody {  }, write=true)?;
        Ok(())
    }

    fn get_queued_cmd_current_index(&mut self) -> Result<u64, DobotError> {
let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 8];
        let response_body = send_cmd!(get sender, EmptyBody, GeneralResponse, CommunicationProtocolIDs::QueuedCmd(QueuedCmdIDs::CurrentIndex), EmptyBody {}, &mut response_buffer)?;
        if response_body.params.len() < 8 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        let current_idx = u64::from_le_bytes(response_buffer);

        Ok(current_idx)
    }
}
