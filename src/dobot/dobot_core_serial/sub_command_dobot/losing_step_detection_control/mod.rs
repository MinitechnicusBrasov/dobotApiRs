#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::protocol::bodies::tag_queue::received::TagQueue;
use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::losing_step_control::LosingStepControl,
    },
    protocol::{
        CommunicationProtocolIDs,
        bodies::{general_request::GeneralRequest, tag_empty_body::EmptyBody},
        command_id::LostStepIDs,
    },
    rwlock::RwLock,
};

pub struct LosingStepDetectionSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> LosingStepDetectionSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> LosingStepControl for LosingStepDetectionSerialControl<'a, T> {
    fn set_lost_step_params(&mut self, value: f32) -> Result<(), DobotError> {
        let request_body = GeneralRequest {
            params: &value.to_le_bytes(),
        };

        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepParams), request_body, write=true)?;

        Ok(())
    }

    fn set_lost_step_cmd(&mut self, is_queued: bool) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;

        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, EmptyBody, CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd), EmptyBody {  }, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::LostStep(LostStepIDs::SetLostStepCmd), EmptyBody {  }, write=true)?;
        Ok(None)
    }
}
