#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::protocol::bodies::tag_empty_body::EmptyBody;
use crate::dobot::dobot_trait::protocol::bodies::tag_queue::received::TagQueue;
use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            arc_control::ArcControl
        ,
    },
    protocol::{
        bodies::{
            tag_arc_cmd::TagARCCmd, tag_arc_params::TagARCParams
        }, command_id::ArcIDs, CommunicationProtocolIDs
    },
    rwlock::RwLock,
};

pub struct ArcSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> ArcSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> ArcControl for ArcSerialControl<'a, T> {
    fn set_arc_params(
        &mut self,
        params: TagARCParams,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagARCParams, CommunicationProtocolIDs::Arc(ArcIDs::ArcParams), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagARCParams, CommunicationProtocolIDs::Arc(ArcIDs::ArcParams), params, write=true)?;
        Ok(None)
    }

    fn get_arc_params(
        &mut self,
    ) -> Result<TagARCParams, DobotError>
    {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 16];
        let response = send_cmd!(get sender, TagARCParams, CommunicationProtocolIDs::Arc(ArcIDs::ArcParams), &mut response_buffer)?;
        Ok(response)
    }

    fn set_arc_cmd(
        &mut self,
        cmd: TagARCCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagARCCmd, CommunicationProtocolIDs::Arc(ArcIDs::ArcCmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagARCCmd, CommunicationProtocolIDs::Arc(ArcIDs::ArcParams), cmd, write=true)?;
        Ok(None)
    }
}
