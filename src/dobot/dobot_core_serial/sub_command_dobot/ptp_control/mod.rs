#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            ptp_control::PTPControl
        ,
    },
    protocol::{
        bodies::{
            general_request::GeneralRequest, tag_empty_body::EmptyBody, tag_po_cmd::TagPOCmd, tag_ptp_cmd::TagPTPCmd, tag_ptp_common_params::TagPTPCommonParams, tag_ptp_coordinate_params::TagPTPCoordinateParams, tag_ptp_joint_params::TagPTPJointParams, tag_ptp_jump2_params::TagPTPJump2Params, tag_ptp_jump_params::TagPTPJumpParams, tag_ptp_with_l_cmd::TagPTPWithLCmd, tag_ptpl_params::TagPTPLParams, tag_queue::received::TagQueue
        }, command_id::PtpIDs, Body, CommunicationProtocolIDs, ProtocolError
    },
    rwlock::RwLock,
};

pub struct PTPSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> PTPSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> PTPControl for PTPSerialControl<'a, T> {
    fn get_ptp_joint_params(
        &mut self,
    ) -> Result<
        TagPTPJointParams,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 32];
        let response_body = send_cmd!(get sender, TagPTPJointParams, CommunicationProtocolIDs::Ptp(PtpIDs::JointParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_ptp_coordinate_params(&mut self) -> Result<TagPTPCoordinateParams, DobotError>{
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 16];
        let response_body = send_cmd!(get sender, TagPTPCoordinateParams, CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_ptp_jump_params(
        &mut self,
    ) -> Result<
        TagPTPJumpParams,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 16];
        let response_body = send_cmd!(get sender, TagPTPJumpParams, CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_ptp_common_params(
        &mut self,
    ) -> Result<
        TagPTPCommonParams,
        DobotError,
    > {
let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 8];
        let response_body = send_cmd!(get sender, TagPTPCommonParams, CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_ptpl_params(
        &mut self,
        params: TagPTPLParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPLParams, CommunicationProtocolIDs::Ptp(PtpIDs::LParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPLParams, CommunicationProtocolIDs::Ptp(PtpIDs::LParams), params, write=true)?;
        Ok(None)
    }

    fn get_ptpl_params(
        &mut self,
    ) -> Result<
        TagPTPLParams,
        DobotError,
    > {
let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 8];
        let response_body = send_cmd!(get sender, TagPTPLParams, CommunicationProtocolIDs::Ptp(PtpIDs::LParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_ptp_with_rail_cmd(
        &mut self,
        cmd: TagPTPWithLCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPWithLCmd, CommunicationProtocolIDs::Ptp(PtpIDs::WithLCmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPWithLCmd, CommunicationProtocolIDs::Ptp(PtpIDs::WithLCmd), cmd, write=true)?;
        Ok(None)
    }

    fn set_ptp_jump2_params(
        &mut self,
        params: TagPTPJump2Params,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPJump2Params, CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPJump2Params, CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams), params, write=true)?;
        Ok(None)
    }

    fn get_ptp_jump2_params(
        &mut self,
    ) -> Result<
        TagPTPJump2Params,
        DobotError,
    > {
let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 12];
        let response_body = send_cmd!(get sender, TagPTPJump2Params, CommunicationProtocolIDs::Ptp(PtpIDs::JumpToParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_ptp_po_cmd(
        &mut self,
        ptp_cmd: TagPTPCmd,
        po_cmds: &[TagPOCmd],
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        let mut data = [0u8; 250];
        if ptp_cmd.size() + po_cmds.len() * 4 > data.len() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        ptp_cmd.serialize(&mut data[0..ptp_cmd.size()])?;
        let mut pointer: usize = ptp_cmd.size();
        for po_cmd in po_cmds {
            po_cmd.serialize(&mut data[pointer..pointer + po_cmd.size()])?;
            pointer += po_cmd.size();
        }

        let wrapper = GeneralRequest { params: &data };


        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::Ptp(PtpIDs::PoCmd), wrapper, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Ptp(PtpIDs::PoCmd), wrapper, write=true)?;
        Ok(None)
    }

    fn set_ptp_po_with_rail_cmd(
        &mut self,
        ptp_cmd: TagPTPWithLCmd,
        po_cmds: &[TagPOCmd],
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        let mut data = [0u8; 250];
        if ptp_cmd.size() + po_cmds.len() * 4 > data.len() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        ptp_cmd.serialize(&mut data[0..ptp_cmd.size()])?;
        let mut pointer: usize = ptp_cmd.size();
        for po_cmd in po_cmds {
            po_cmd.serialize(&mut data[pointer..pointer + po_cmd.size()])?;
            pointer += po_cmd.size();
        }

        let wrapper = GeneralRequest { params: &data };


        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, GeneralRequest, CommunicationProtocolIDs::Ptp(PtpIDs::PoWithLCmd), wrapper, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Ptp(PtpIDs::PoWithLCmd), wrapper, write=true)?;
        Ok(None)
    }
    

    fn set_ptp_joint_params(
        &mut self,
        params: TagPTPJointParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPJointParams, CommunicationProtocolIDs::Ptp(PtpIDs::JointParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPJointParams, CommunicationProtocolIDs::Ptp(PtpIDs::JointParams), params, write=true)?;
        Ok(None)
    }

    fn set_ptp_coordinate_params(
        &mut self,
        params: TagPTPCoordinateParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPCoordinateParams, CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPCoordinateParams, CommunicationProtocolIDs::Ptp(PtpIDs::CoordinateParams), params, write=true)?;
        Ok(None)
    }

    fn set_ptp_jump_params(
        &mut self,
        params: TagPTPJumpParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPJumpParams, CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPJumpParams, CommunicationProtocolIDs::Ptp(PtpIDs::JumpParams), params, write=true)?;
        Ok(None)
    }

    fn set_ptp_common_params(
        &mut self,
        params: TagPTPCommonParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPCommonParams, CommunicationProtocolIDs::Ptp(PtpIDs::CommonParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPCommonParams, CommunicationProtocolIDs::Ptp(PtpIDs::CommonParams), params, write=true)?;
        Ok(None)
    }

    fn set_ptp_cmd(
        &mut self,
        cmd: TagPTPCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagPTPCmd, CommunicationProtocolIDs::Ptp(PtpIDs::Cmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagPTPCmd, CommunicationProtocolIDs::Ptp(PtpIDs::Cmd), cmd, write=true)?;
        Ok(None)
    }
}
