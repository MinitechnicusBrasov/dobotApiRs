#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
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

pub struct PTPSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> PTPSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> PTPControl for PTPSerialControl<'a, T> {
    fn get_ptp_joint_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_ptp_joint_params::TagPTPJointParams,
        DobotError,
    > {
        todo!()
    }

    fn get_ptp_coordinate_params(&mut self) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_ptp_coordinate_params::TagPTPCoordinateParams, DobotError>{
        todo!()
    }

    fn get_ptp_jump_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_ptp_jump_params::TagPTPJumpParams,
        DobotError,
    > {
        todo!()
    }

    fn get_ptp_common_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_ptp_common_params::TagPTPCommonParams,
        DobotError,
    > {
        todo!()
    }

    fn set_ptpl_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptpl_params::TagPTPLParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_ptpl_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_ptpl_params::TagPTPLParams,
        DobotError,
    > {
        todo!()
    }

    fn set_ptp_with_rail_cmd(
        &mut self,
        cmd: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_with_l_cmd::TagPTPWithLCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_jump2_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_jump2_params::TagPTPJump2Params,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn get_ptp_jump2_params(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_ptp_jump2_params::TagPTPJump2Params,
        DobotError,
    > {
        todo!()
    }

    fn set_ptp_po_cmd(
        &mut self,
        ptp_cmd: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_cmd::TagPTPCmd,
        po_cmds: &[crate::dobot::dobot_trait::protocol::bodies::tag_po_cmd::TagPOCmd],
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_po_with_rail_cmd(
        &mut self,
        ptp_cmd: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_with_l_cmd::TagPTPWithLCmd,
        po_cmds: &[crate::dobot::dobot_trait::protocol::bodies::tag_po_cmd::TagPOCmd],
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_joint_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_joint_params::TagPTPJointParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_coordinate_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_coordinate_params::TagPTPCoordinateParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_jump_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_jump_params::TagPTPJumpParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_common_params(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_common_params::TagPTPCommonParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }

    fn set_ptp_cmd(
        &mut self,
        cmd: crate::dobot::dobot_trait::protocol::bodies::tag_ptp_cmd::TagPTPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError> {
        todo!()
    }
}
