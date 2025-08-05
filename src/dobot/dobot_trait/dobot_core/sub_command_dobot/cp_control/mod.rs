use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{tag_cp_cmd::TagCPCmd, tag_cp_params::TagCPParams},
};

pub trait CPControl {
    /// Executes a CP (Continuous Path) command.
    ///
    /// `cmd`: CP command parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_cp_cmd(
        &mut self,
        cmd: TagCPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets parameters for CP (Continuous Path) movements.
    ///
    /// `params`: CP parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_cp_params(
        &mut self,
        params: TagCPParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for CP movements.
    /// Returns current CP parameters.
    fn get_cp_params(&mut self) -> Result<TagCPParams, DobotError>;

    /// Executes a CP command for laser engraving.
    ///
    /// `cmd`: CP command (used for laser engraving context).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_cp_le_cmd(
        &mut self,
        cmd: TagCPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;
}
