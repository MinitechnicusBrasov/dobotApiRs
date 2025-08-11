#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{home_control::HomeControl, jog_control::JOGControl},
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

pub struct JOGSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> JOGSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> JOGControl for JOGSerialControl<'a, T> {
    fn set_jog_joint_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_jog_joint_params::TagJOGJointParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_jog_joint_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_jog_joint_params::TagJOGJointParams,
        DobotError,
    > {
        todo!()
    }

    fn set_jog_coordinate_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_jog_coordinate_params::TagJOGCoordinateParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_jog_coordinate_params(&mut self) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_jog_coordinate_params::TagJOGCoordinateParams, DobotError>{
        todo!()
    }

    fn set_jog_common_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_jog_common_params::TagJOGCommonParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_jog_common_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_jog_common_params::TagJOGCommonParams,
        DobotError,
    > {
        todo!()
    }

    fn set_jog_cmd(
        &mut self,
        cmd: crate::dobot::dobot_trait::protocol::bodies::tag_jog_cmd::TagJOGCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_jogl_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_jog_l_params::TagJOGLParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_jogl_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_jog_l_params::TagJOGLParams,
        DobotError,
    > {
        todo!()
    }
}
