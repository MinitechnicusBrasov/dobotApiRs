use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError, protocol::bodies::tag_trig_cmd::TagTRIGCmd,
};

pub trait TriggerControl {
    /// Executes a TRIG (Trigger) command.
    ///
    /// `cmd`: TRIG command parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_trig_cmd(
        &mut self,
        cmd: TagTRIGCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;
}
