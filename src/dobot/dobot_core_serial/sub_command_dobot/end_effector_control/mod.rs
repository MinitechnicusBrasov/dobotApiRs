#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{end_effector_control::EndEffectorControl, home_control::HomeControl},
    },
    protocol::{
        CommunicationProtocolIDs, ProtocolError,
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse,
            tag_auto_leveling_params::TagAutoLevelingParams, tag_empty_body::EmptyBody,
            tag_end_effector_params::TagEndEffectorParams, tag_home_cmd::TagHomeCmd,
            tag_home_params::TagHomeParams, tag_queue::received::TagQueue,
        },
        command_id::{EndEffectorIDs, HomeIDs},
    },
    rwlock::RwLock,
};

pub struct EndEffectorSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> EndEffectorSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> EndEffectorControl for EndEffectorSerialControl<'a, T> {
    fn set_gripper_state(
        &mut self,
        enable: bool,
        grip: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let params_body = [enable as u8, grip as u8];
        let request_body = GeneralRequest {
            params: &params_body,
        };

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper), request_body, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper), request_body, write=true)?;
        Ok(None)
    }

    fn set_suction_cup_state(
        &mut self,
        enable: bool,
        suck: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let params_body = [enable as u8, suck as u8];
        let request_body = GeneralRequest {
            params: &params_body,
        };

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup), request_body, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup), request_body, write=true)?;
        Ok(None)
    }

    fn set_laser_state(
        &mut self,
        enable_ctrl: bool,
        on: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let params_body = [enable_ctrl as u8, on as u8];
        let request_body = GeneralRequest {
            params: &params_body,
        };

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser), request_body, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser), request_body, write=true)?;
        Ok(None)
    }

    fn get_gripper_state(&mut self) -> Result<(bool, bool), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 2];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Gripper), &mut response_buffer)?;

        if response_body.params.len() != 2 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok((response_body.params[0] != 0, response_body.params[1] != 0))
    }

    fn get_suction_cup_state(&mut self) -> Result<(bool, bool), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 2];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::SuctionCup), &mut response_buffer)?;

        if response_body.params.len() != 2 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok((response_body.params[0] != 0, response_body.params[1] != 0))
    }

    fn get_laser_state(&mut self) -> Result<(bool, bool), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 2];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Laser), &mut response_buffer)?;

        if response_body.params.len() != 2 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok((response_body.params[0] != 0, response_body.params[1] != 0))
    }

    fn set_end_effector_params(
        &mut self,
        params: TagEndEffectorParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagEndEffectorParams, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagEndEffectorParams, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params), params, write=true)?;
        Ok(None)
    }

    fn get_end_effector_params(&mut self) -> Result<TagEndEffectorParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 12];
        let response_body = send_cmd!(get sender, TagEndEffectorParams, CommunicationProtocolIDs::EndEffector(EndEffectorIDs::Params), &mut response_buffer)?;

        Ok(response_body)
    }
}
