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

pub struct IOSerialControl<'a, T: CommandSender> {
    command_sender: &'a mut RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> IOSerialControl<'a, T> {
    pub fn new(command_sender: &'a mut RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> IOControl for IOSerialControl<'a, T> {
    fn set_io_multiplexing(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_io_multiplexing::TagIOMultiplexing,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_io_multiplexing(
        &mut self,
        address: u8,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_io_multiplexing::TagIOMultiplexing,
        DobotError,
    > {
        todo!()
    }

    fn set_io_do(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_io_do::TagIODO,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_io_do(
        &mut self,
        address: u8,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::level::Level, DobotError> {
        todo!()
    }

    fn set_io_pwm(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_io_pwm::TagIOPWM,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_io_pwm(
        &mut self,
        address: u8,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_io_pwm::TagIOPWM, DobotError> {
        todo!()
    }

    fn get_io_di(
        &mut self,
        address: u8,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::level::Level, DobotError> {
        todo!()
    }

    fn get_io_adc(&mut self, address: u8) -> Result<u16, DobotError> {
        todo!()
    }

    fn set_e_motor(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_emotor::TagEMotor,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_color_sensor(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_device::TagDevice,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_color_sensor(
        &mut self,
        port: u8,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_color::TagColor, DobotError> {
        todo!()
    }

    fn set_ir_switch(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_device::TagDevice,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_ir_switch(&mut self, port: u8) -> Result<bool, DobotError> {
        todo!()
    }
}
