#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            handhold_teaching_control::HandholdTeachingControl
        ,
    }, protocol::{bodies::{general_request::GeneralRequest, general_response::GeneralResponse, hht_trig_mode::HHTTrigMode, tag_empty_body::EmptyBody}, command_id::HHTIDs, CommunicationProtocolIDs, ProtocolError}, rwlock::RwLock};

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
        
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode), request, write=true)?;
        
        Ok(())
    }

    fn get_hht_trig_mode(
        &mut self,
    ) -> Result<HHTTrigMode, DobotError>
    {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigMode), &mut response_buffer)?;

        if response_body.params.len() != 1 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok(HHTTrigMode::try_from_u8(response_body.params[0])?)
    }

    fn set_hht_trig_output_enabled(&mut self, is_enabled: bool) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request_buffer = [is_enabled as u8];
        let request = GeneralRequest { params: &request_buffer };
        
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled), request, write=true)?;
        
        Ok(())
    }

    fn get_hht_trig_output_enabled(&mut self) -> Result<bool, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutputEnabled), &mut response_buffer)?;

        if response_body.params.len() != 1 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok(response_body.params[0] != 0)
    }

    fn get_hht_trig_output(&mut self) -> Result<bool, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::HHT(HHTIDs::HHTTrigOutput), &mut response_buffer)?;

        if response_body.params.len() != 1 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        Ok(response_body.params[0] != 0)
    }
}
