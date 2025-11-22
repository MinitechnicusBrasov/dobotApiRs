use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::tag_end_effector_params::TagEndEffectorParams,
};

pub trait EndEffectorControl {
    fn set_gripper_state(&mut self, enable: bool, grip: bool, is_queued: bool) -> Result<Option<u64>, DobotError>;
    fn set_suction_cup_state(&mut self, enable: bool, suck: bool, is_queued: bool) -> Result<Option<u64>, DobotError>;
    fn set_laser_state(&mut self, enable_ctrl: bool, on: bool, is_queued: bool) -> Result<Option<u64>, DobotError>;
    fn get_gripper_state(&mut self) -> Result<(bool, bool), DobotError>;
    fn get_suction_cup_state(&mut self) -> Result<(bool, bool), DobotError>;
    fn get_laser_state(&mut self) -> Result<(bool, bool), DobotError>;

    fn set_end_effector_params(&mut self, params: TagEndEffectorParams, is_queued: bool) -> Result<Option<u64>, DobotError>;
    fn get_end_effector_params(&mut self) -> Result<TagEndEffectorParams, DobotError>;
}
