#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            cp_control::CPControl, home_control::HomeControl, io_control::IOControl,
            jog_control::JOGControl, ptp_control::PTPControl,
        },
    },
    protocol::{
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse, level::Level, tag_auto_leveling_params::TagAutoLevelingParams, tag_color::TagColor, tag_device::TagDevice, tag_emotor::TagEMotor, tag_empty_body::EmptyBody, tag_home_cmd::TagHomeCmd, tag_home_params::TagHomeParams, tag_io_do::TagIODO, tag_io_multiplexing::TagIOMultiplexing, tag_io_pwm::TagIOPWM, tag_queue::received::TagQueue
        }, command_id::{EioIDs, HomeIDs}, CommunicationProtocolIDs, ProtocolError
    },
    rwlock::RwLock,
};

pub struct IOSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> IOSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> IOControl for IOSerialControl<'a, T> {
    fn set_io_multiplexing(
        &mut self,
        params: TagIOMultiplexing,
        is_queued: bool
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagIOMultiplexing, CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagIOMultiplexing, CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), params, write=true)?;
        Ok(None)
    }

    fn get_io_multiplexing(
        &mut self,
        address: u8,
    ) -> Result<
        TagIOMultiplexing,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[address]};
        let mut response_buffer = [0u8; 2];
        let response_body = send_cmd!(get sender, GeneralRequest, TagIOMultiplexing, CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), request, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_io_do(
        &mut self,
        params: TagIODO,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagIODO, CommunicationProtocolIDs::Eio(EioIDs::Iodo), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagIODO, CommunicationProtocolIDs::Eio(EioIDs::Iodo), params, write=true)?;
        Ok(None)
    }

    fn get_io_do(
        &mut self,
        address: u8,
    ) -> Result<Level, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[address]};
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralRequest, Level, CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), request, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_io_pwm(
        &mut self,
        params: TagIOPWM,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagIOPWM, CommunicationProtocolIDs::Eio(EioIDs::IoPwm), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagIOPWM, CommunicationProtocolIDs::Eio(EioIDs::IoPwm), params, write=true)?;
        Ok(None)
    }

    fn get_io_pwm(
        &mut self,
        address: u8,
    ) -> Result<TagIOPWM, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[address]};
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralRequest, TagIOPWM, CommunicationProtocolIDs::Eio(EioIDs::IoPwm), request, &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_io_di(
        &mut self,
        address: u8,
    ) -> Result<Level, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[address]};
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralRequest, Level, CommunicationProtocolIDs::Eio(EioIDs::Iodi), request, &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_io_adc(&mut self, address: u8) -> Result<u16, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[address]};
        let mut response_buffer = [0u8; 2];
        let response_body = send_cmd!(get sender, GeneralRequest, GeneralResponse, CommunicationProtocolIDs::Eio(EioIDs::Iodi), request, &mut response_buffer)?;
        if response_body.params.len() != 2 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        let adc_value = u16::from_le_bytes(response_buffer);

        Ok(adc_value)
    }

    fn set_e_motor(
        &mut self,
        params: TagEMotor,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagEMotor, CommunicationProtocolIDs::Eio(EioIDs::Emotor), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagEMotor, CommunicationProtocolIDs::Eio(EioIDs::Emotor), params, write=true)?;
        Ok(None)
    }

    fn set_color_sensor(
        &mut self,
        params: TagDevice,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagDevice, CommunicationProtocolIDs::Eio(EioIDs::ColorSensor), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagDevice, CommunicationProtocolIDs::Eio(EioIDs::ColorSensor), params, write=true)?;
        Ok(None)
    }

    fn get_color_sensor(
        &mut self,
        port: u8,
    ) -> Result<TagColor, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[port]};
        let mut response_buffer = [0u8; 3];
        let response_body = send_cmd!(get sender, GeneralRequest, TagColor, CommunicationProtocolIDs::Eio(EioIDs::ColorSensor), request, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_ir_switch(
        &mut self,
        params: TagDevice,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        if is_queued {
            let mut response = [0u8; 8];
            let queue_idx = send_cmd!(get_queue sender, TagDevice, CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), params, &mut response, write=true)?;
            return Ok(Some(queue_idx.queue_idx));
        }
        send_cmd!(send sender, TagDevice, CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), params, write=true)?;
        Ok(None)
    }

    fn get_ir_switch(&mut self, port: u8) -> Result<bool, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request = GeneralRequest { params: &[port]};
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, GeneralRequest, GeneralResponse, CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), request, &mut response_buffer)?;

        Ok(response_body.params[0] != 0)
    }
}
