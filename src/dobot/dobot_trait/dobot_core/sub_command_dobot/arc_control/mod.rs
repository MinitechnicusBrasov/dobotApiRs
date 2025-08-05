use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{tag_arc_cmd::TagARCCmd, tag_arc_params::TagARCParams},
};

pub trait ArcControl {
    /// Sets parameters for ARC (Arc) movements.
    ///
    /// `params`: ARC parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_arc_params(
        &mut self,
        params: TagARCParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for ARC movements.
    /// Returns current ARC parameters.
    fn get_arc_params(&mut self) -> Result<TagARCParams, DobotError>;

    /// Executes an ARC (Arc) movement command.
    ///
    /// `cmd`: ARC command parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_arc_cmd(
        &mut self,
        cmd: TagARCCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;
}
