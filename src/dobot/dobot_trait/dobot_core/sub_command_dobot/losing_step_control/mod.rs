use crate::dobot::dobot_trait::dobot_core::dobot_error::DobotError;

pub trait LosingStepControl {
    /// Sets parameters for losing-step detection threshold.
    ///
    /// `value`: Threshold value for lost step parameters.
    ///
    /// Returns the response message from the Dobot.
    fn set_lost_step_params(&mut self, value: f32) -> Result<(), DobotError>;

    /// Executes a losing-step detection command.
    ///
    /// `wait`: If `true` and the command is queued, waits for execution.
    /// `is_queued`: If `true`, the command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_lost_step_cmd(&mut self, is_queued: bool) -> Result<Option<u64>, DobotError>;
}
