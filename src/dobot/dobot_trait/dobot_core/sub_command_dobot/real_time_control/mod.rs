use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError, protocol::bodies::tag_pose::TagPose,
};

pub trait RealTimeControl {
    /// Resets the real-time pose of the robot.
    ///
    /// `manual`: Manual reset flag (0 = auto, 1 = manual with angles).
    /// `rear_arm_angle`: Rear arm angle for reset (if manual = 1).
    /// `front_arm_angle`: Front arm angle for reset (if manual = 1).
    fn reset_pose(
        &mut self,
        manual: u8,
        rear_arm_angle: f32,
        front_arm_angle: f32,
    ) -> Result<(), DobotError>;

    /// Gets the real-time pose (position and joint angles) of the Dobot.
    /// Returns a `TagPose` struct containing the pose data.
    fn get_pose(&mut self) -> Result<TagPose, DobotError>;

    /// Gets the rail pose (position of sliding rail).
    /// Returns the position of the rail as a `f32`.
    fn get_pose_rail(&mut self) -> Result<f32, DobotError>;
}
