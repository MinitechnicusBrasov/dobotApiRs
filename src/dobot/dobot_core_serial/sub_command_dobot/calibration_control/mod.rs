#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            calibration_control::CalibrationControl, cp_control::CPControl,
            home_control::HomeControl, jog_control::JOGControl, ptp_control::PTPControl,
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

pub struct CalibrationSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> CalibrationSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> CalibrationControl for CalibrationSerialControl<'a, T> {
    fn set_angle_sensor_static_error(
        &mut self,
        rear_arm_angle_error: f32,
        front_arm_angle_error: f32,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_angle_sensor_static_error(&mut self) -> Result<(f32, f32), DobotError> {
        todo!()
    }
}
