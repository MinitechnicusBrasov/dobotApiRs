use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{
        tag_jog_cmd::TagJOGCmd, tag_jog_common_params::TagJOGCommonParams,
        tag_jog_coordinate_params::TagJOGCoordinateParams, tag_jog_joint_params::TagJOGJointParams,
        tag_jog_l_params::TagJOGLParams,
    },
};

pub trait JOGControl {
    /// Sets parameters for joint mode JOG movements.
    ///
    /// `params`: JOG joint parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_jog_joint_params(
        &mut self,
        params: TagJOGJointParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for joint mode JOG movements.
    /// Returns current JOG joint parameters.
    fn get_jog_joint_params(&mut self) -> Result<TagJOGJointParams, DobotError>;

    /// Sets parameters for coordinate mode JOG movements.
    ///
    /// `params`: JOG coordinate parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_jog_coordinate_params(
        &mut self,
        params: TagJOGCoordinateParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for coordinate mode JOG movements.
    /// Returns current JOG coordinate parameters.
    fn get_jog_coordinate_params(&mut self) -> Result<TagJOGCoordinateParams, DobotError>;

    /// Sets common JOG parameters (velocity/acceleration ratios).
    ///
    /// `params`: Common JOG parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_jog_common_params(
        &mut self,
        params: TagJOGCommonParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets common JOG parameters.
    /// Returns current common JOG parameters.
    fn get_jog_common_params(&mut self) -> Result<TagJOGCommonParams, DobotError>;

    /// Executes a JOG command.
    ///
    /// `cmd`: JOG command parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_jog_cmd(
        &mut self,
        cmd: TagJOGCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets parameters for JOGL (linear jog) mode.
    ///
    /// `params`: JOGL parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_jogl_params(
        &mut self,
        params: TagJOGLParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for JOGL (linear jog) mode.
    /// Returns current JOGL parameters.
    fn get_jogl_params(&mut self) -> Result<TagJOGLParams, DobotError>;
}
