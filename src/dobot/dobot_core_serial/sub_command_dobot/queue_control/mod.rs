#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            cp_control::CPControl, home_control::HomeControl, jog_control::JOGControl,
            ptp_control::PTPControl, queue_control::QueueControl,
        },
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
        todo!()
    }

    fn set_queued_cmd_start_download(
        &mut self,
        total_loop: u32,
        line_per_loop: u32,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn set_queued_cmd_stop_download(&mut self) -> Result<(), DobotError> {
        todo!()
    }

    fn set_queued_cmd_start_exec(&mut self) -> Result<(), DobotError> {
        todo!()
    }

    fn set_queued_cmd_stop_exec(&mut self) -> Result<(), DobotError> {
        todo!()
    }

    fn set_queued_cmd_clear(&mut self) -> Result<(), DobotError> {
        todo!()
    }

    fn get_queued_cmd_current_index(&mut self) -> Result<u64, DobotError> {
        todo!()
    }
}
