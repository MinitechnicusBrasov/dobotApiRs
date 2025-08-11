#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{end_effector_control::EndEffectorControl, home_control::HomeControl},
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

pub struct EndEffectorSerialControl<'a, T: CommandSender> {
    command_sender: &'a mut RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> EndEffectorSerialControl<'a, T> {
    pub fn new(command_sender: &'a mut RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> EndEffectorControl for EndEffectorSerialControl<'a, T> {
    fn set_gripper_state(&mut self, enable: bool, grip: bool) -> Result<(), DobotError> {
        todo!()
    }

    fn set_suction_cup_state(&mut self, enable: bool) -> Result<(), DobotError> {
        todo!()
    }

    fn set_laser_state(&mut self, enable_ctrl: bool, on: bool) -> Result<(), DobotError> {
        todo!()
    }

    fn get_gripper_state(&mut self) -> Result<(bool, bool), DobotError> {
        todo!()
    }

    fn get_suction_cup_state(&mut self) -> Result<(bool, bool), DobotError> {
        todo!()
    }

    fn get_laser_state(&mut self) -> Result<(bool, bool), DobotError> {
        todo!()
    }

    fn set_end_effector_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_end_effector_params::TagEndEffectorParams,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_end_effector_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_end_effector_params::TagEndEffectorParams,
        DobotError,
    > {
        todo!()
    }
}
