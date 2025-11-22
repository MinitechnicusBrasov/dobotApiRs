#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::protocol::bodies::tag_empty_body::EmptyBody;
use crate::dobot::dobot_trait::protocol::bodies::tag_queue::received::TagQueue;
use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::jog_control::JOGControl,
    },
    protocol::{
        CommunicationProtocolIDs,
        bodies::{
            tag_jog_cmd::TagJOGCmd, tag_jog_common_params::TagJOGCommonParams,
            tag_jog_coordinate_params::TagJOGCoordinateParams,
            tag_jog_joint_params::TagJOGJointParams, tag_jog_l_params::TagJOGLParams,
        },
        command_id::JogIDs,
    },
    rwlock::RwLock,
};

pub struct JOGSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> JOGSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> JOGControl for JOGSerialControl<'a, T> {
    fn set_jog_joint_params(
        &mut self,
        params: TagJOGJointParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagJOGJointParams, CommunicationProtocolIDs::Jog(JogIDs::JointParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagJOGJointParams, CommunicationProtocolIDs::Jog(JogIDs::JointParams), params, write=true)?;
        Ok(None)
    }

    fn get_jog_joint_params(&mut self) -> Result<TagJOGJointParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 32];
        let response_body = send_cmd!(get sender, TagJOGJointParams, CommunicationProtocolIDs::Jog(JogIDs::JointParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_jog_coordinate_params(
        &mut self,
        params: TagJOGCoordinateParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagJOGCoordinateParams, CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagJOGCoordinateParams, CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams), params, write=true)?;
        Ok(None)
    }

    fn get_jog_coordinate_params(&mut self) -> Result<TagJOGCoordinateParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 32];
        let response_body = send_cmd!(get sender, TagJOGCoordinateParams, CommunicationProtocolIDs::Jog(JogIDs::CoordinateParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_jog_common_params(
        &mut self,
        params: TagJOGCommonParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagJOGCommonParams, CommunicationProtocolIDs::Jog(JogIDs::CommonParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagJOGCommonParams, CommunicationProtocolIDs::Jog(JogIDs::CommonParams), params, write=true)?;
        Ok(None)
    }

    fn get_jog_common_params(&mut self) -> Result<TagJOGCommonParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 8];
        let response_body = send_cmd!(get sender, TagJOGCommonParams, CommunicationProtocolIDs::Jog(JogIDs::CommonParams), &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_jog_cmd(&mut self, cmd: TagJOGCmd, is_queued: bool) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagJOGCmd, CommunicationProtocolIDs::Jog(JogIDs::Cmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagJOGCmd, CommunicationProtocolIDs::Jog(JogIDs::Cmd), cmd, write=true)?;
        Ok(None)
    }

    fn set_jogl_params(
        &mut self,
        params: TagJOGLParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagJOGLParams, CommunicationProtocolIDs::Jog(JogIDs::LParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagJOGLParams, CommunicationProtocolIDs::Jog(JogIDs::LParams), params, write=true)?;
        Ok(None)
    }

    fn get_jogl_params(&mut self) -> Result<TagJOGLParams, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 8];
        let response_body = send_cmd!(get sender, TagJOGLParams, CommunicationProtocolIDs::Jog(JogIDs::LParams), &mut response_buffer)?;

        Ok(response_body)
    }
}
