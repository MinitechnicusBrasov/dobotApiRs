#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::trigger_control::TriggerControl,
    },
    protocol::{
        CommunicationProtocolIDs,
        bodies::{
            tag_empty_body::EmptyBody, tag_queue::received::TagQueue, tag_trig_cmd::TagTRIGCmd,
        },
        command_id::TrigIDs,
    },
    rwlock::RwLock,
};

pub struct TriggerSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> TriggerSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> TriggerControl for TriggerSerialControl<'a, T> {
    fn set_trig_cmd(
        &mut self,
        cmd: TagTRIGCmd,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagTRIGCmd, CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd), cmd, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagTRIGCmd, CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd), cmd, write=true)?;
        Ok(None)
    }
}
