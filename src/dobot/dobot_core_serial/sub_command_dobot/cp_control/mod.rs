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
        cmd: crate::dobot::dobot_trait::protocol::bodies::tag_cp_cmd::TagCPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_cp_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_cp_params::TagCPParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_cp_params(
        &mut self,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_cp_params::TagCPParams, DobotError>
    {
        todo!()
    }

    fn set_cp_le_cmd(
        &mut self,
        cmd: crate::dobot::dobot_trait::protocol::bodies::tag_cp_cmd::TagCPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }
}
