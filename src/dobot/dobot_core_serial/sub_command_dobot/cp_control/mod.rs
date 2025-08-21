#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            cp_control::CPControl, home_control::HomeControl, jog_control::JOGControl,
            ptp_control::PTPControl,
        },
    },
    protocol::{
        bodies::{
            general_response::GeneralResponse, tag_auto_leveling_params::TagAutoLevelingParams, tag_cp_cmd::TagCPCmd, tag_cp_params::TagCPParams, tag_empty_body::EmptyBody, tag_home_cmd::TagHomeCmd, tag_home_params::TagHomeParams, tag_queue::received::TagQueue
        }, command_id::{CpIDs, HomeIDs}, CommunicationProtocolIDs, ProtocolError
    },
    rwlock::RwLock,
};

pub struct CPSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> CPSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> CPControl for CPSerialControl<'a, T> {
    fn set_cp_cmd(
        &mut self,
        cmd: TagCPCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagCPCmd, CommunicationProtocolIDs::Cp(CpIDs::CpCmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagCPCmd, CommunicationProtocolIDs::Cp(CpIDs::CpCmd), cmd, write=true)?;
        Ok(None)
    }

    fn set_cp_params(
        &mut self,
        params: TagCPParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagCPParams, CommunicationProtocolIDs::Cp(CpIDs::CpParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagCPParams, CommunicationProtocolIDs::Cp(CpIDs::CpParams), params, write=true)?;
        Ok(None)
    }

    fn get_cp_params(
        &mut self,
    ) -> Result<TagCPParams, DobotError>
    {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 13];

        let result = send_cmd!(get sender, TagCPParams, CommunicationProtocolIDs::Cp(CpIDs::CpParams), &mut response_buffer)?;
        Ok(result)
    }

    fn set_cp_le_cmd(
        &mut self,
        cmd: TagCPCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagCPCmd, CommunicationProtocolIDs::Cp(CpIDs::CpleCmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagCPCmd, CommunicationProtocolIDs::Cp(CpIDs::CpleCmd), cmd, write=true)?;
        Ok(None)
    }
}
