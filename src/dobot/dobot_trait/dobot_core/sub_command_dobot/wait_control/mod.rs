use crate::dobot::dobot_trait::dobot_core::dobot_error::DobotError;

pub trait WaitControl {
    /// Adds a wait command to the queue.
    ///
    /// `params`: Wait command parameters (timeout in ms).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_wait_cmd(
        &mut self,
        timeout: u32,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;
}
