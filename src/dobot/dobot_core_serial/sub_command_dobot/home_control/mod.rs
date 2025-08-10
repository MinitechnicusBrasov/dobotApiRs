#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::home_control::HomeControl,
    },
    protocol::{
        CommunicationProtocolIDs, ProtocolError,
        bodies::{
            general_response::GeneralResponse, tag_auto_leveling_params::TagAutoLevelingParams,
            tag_empty_body::EmptyBody, tag_home_cmd::TagHomeCmd, tag_home_params::TagHomeParams,
            tag_queue::received::TagQueue,
        },
        command_id::HomeIDs,
    },
    rwlock::RwLock,
};

pub struct HomeSerialControl<'a, T: CommandSender> {
    command_sender: &'a mut RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> HomeSerialControl<'a, T> {
    pub fn new(command_sender: &'a mut RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> HomeControl for HomeSerialControl<'a, T> {
    fn set_home_params(
        &mut self,
        params: TagHomeParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagHomeParams, CommunicationProtocolIDs::Home(HomeIDs::HomeParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagHomeParams, CommunicationProtocolIDs::Home(HomeIDs::HomeParams), params, write=true)?;
        Ok(None)
    }

    fn get_home_params(&mut self) -> Result<TagHomeParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 16];
        let response = send_cmd!(get sender, TagHomeParams, CommunicationProtocolIDs::Home(HomeIDs::HomeParams), &mut response_buffer)?;
        Ok(response)
    }

    fn set_home_cmd(
        &mut self,
        params: TagHomeCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagHomeCmd, CommunicationProtocolIDs::Home(HomeIDs::HomeCmd), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagHomeCmd, CommunicationProtocolIDs::Home(HomeIDs::HomeCmd), params, write=true)?;
        Ok(None)
    }

    fn set_autoleveling(
        &mut self,
        params: TagAutoLevelingParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagAutoLevelingParams, CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagAutoLevelingParams, CommunicationProtocolIDs::Home(HomeIDs::AutoLeveling), params, write=true)?;
        Ok(None)
    }

    fn get_autoleveling(&mut self) -> Result<f32, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 4];
        let response = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::Home(HomeIDs::HomeParams), &mut response_buffer)?;
        if response.params.len() < 4 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        let result = f32::from_le_bytes(response_buffer);
        Ok(result)
    }
}
