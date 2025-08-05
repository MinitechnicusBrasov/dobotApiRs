use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{
        tag_auto_leveling_params::TagAutoLevelingParams, tag_home_cmd::TagHomeCmd,
        tag_home_params::TagHomeParams,
    },
};

pub trait HomeControl {
    /// Sets homing parameters (target coordinates).
    ///
    /// `params`: Homing parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_home_params(
        &mut self,
        params: TagHomeParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets homing parameters.
    /// Returns current homing parameters.
    fn get_home_params(&mut self) -> Result<TagHomeParams, DobotError>;

    /// Executes the homing function.
    ///
    /// `params`: Homing command options (reserved).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_home_cmd(
        &mut self,
        params: TagHomeCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets auto-leveling parameters and initiates auto-leveling.
    ///
    /// `params`: Auto-leveling parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_autoleveling(
        &mut self,
        params: TagAutoLevelingParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets automatic leveling result/status.
    /// Returns AutoLevelingResult (float, accuracy or status).
    fn get_autoleveling(&mut self) -> Result<f32, DobotError>;
}
