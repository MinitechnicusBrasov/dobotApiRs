#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            handhold_teaching_control::HandholdTeachingControl, home_control::HomeControl,
            jog_control::JOGControl,
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

pub struct HandholdTeachingSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> HandholdTeachingSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> HandholdTeachingControl for HandholdTeachingSerialControl<'a, T> {
    fn set_hht_trig_mode(
        &mut self,
        mode: crate::dobot::dobot_trait::protocol::bodies::hht_trig_mode::HHTTrigMode,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_hht_trig_mode(
        &mut self,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::hht_trig_mode::HHTTrigMode, DobotError>
    {
        todo!()
    }

    fn set_hht_trig_output_enabled(&mut self, is_enabled: bool) -> Result<(), DobotError> {
        todo!()
    }

    fn get_hht_trig_output_enabled(&mut self) -> Result<bool, DobotError> {
        todo!()
    }

    fn get_hht_trig_output(&mut self) -> Result<bool, DobotError> {
        todo!()
    }
}
