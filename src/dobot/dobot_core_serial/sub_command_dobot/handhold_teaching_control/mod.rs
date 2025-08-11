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
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse, hht_trig_mode::HHTTrigMode, tag_auto_leveling_params::TagAutoLevelingParams, tag_empty_body::EmptyBody, tag_home_cmd::TagHomeCmd, tag_home_params::TagHomeParams, tag_queue::received::TagQueue
        }, command_id::{HhtIDs, HomeIDs}, CommunicationProtocolIDs, ProtocolError
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
        mode: HHTTrigMode,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request_buffer = [mode as u8];
        let request = GeneralRequest { params: &request_buffer };
        
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Hht(HhtIDs::SetGetHhtTrigMode), request, write=true)?;
        
        Ok(())
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
