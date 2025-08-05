use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{
        tag_po_cmd::TagPOCmd, tag_ptp_cmd::TagPTPCmd, tag_ptp_common_params::TagPTPCommonParams,
        tag_ptp_coordinate_params::TagPTPCoordinateParams, tag_ptp_joint_params::TagPTPJointParams,
        tag_ptp_jump_params::TagPTPJumpParams, tag_ptp_jump2_params::TagPTPJump2Params,
        tag_ptp_with_l_cmd::TagPTPWithLCmd, tag_ptpl_params::TagPTPLParams,
    },
};

pub trait PTPControl {
    /// Gets PTP joint parameters.
    /// Returns current PTP joint parameters.
    fn get_ptp_joint_params(&mut self) -> Result<TagPTPJointParams, DobotError>;

    /// Gets PTP coordinate parameters.
    /// Returns current PTP coordinate parameters.
    fn get_ptp_coordinate_params(&mut self) -> Result<TagPTPCoordinateParams, DobotError>;

    /// Gets PTP jump parameters.
    /// Returns current PTP jump parameters.
    fn get_ptp_jump_params(&mut self) -> Result<TagPTPJumpParams, DobotError>;

    /// Gets PTP common parameters.
    /// Returns current PTP common parameters.
    fn get_ptp_common_params(&mut self) -> Result<TagPTPCommonParams, DobotError>;

    /// Sets parameters for PTPL (Point-to-Point Linear) mode.
    ///
    /// `params`: PTPL parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptpl_params(
        &mut self,
        params: TagPTPLParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets parameters for PTPL mode.
    /// Returns current PTPL parameters.
    fn get_ptpl_params(&mut self) -> Result<TagPTPLParams, DobotError>;

    /// Executes a PTP command with rail movement.
    ///
    /// `cmd`: PTP command with rail parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_with_rail_cmd(
        &mut self,
        cmd: TagPTPWithLCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets extended jump parameters for PTP movements.
    ///
    /// `params`: PTP jump2 parameters (start/end jump heights).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_jump2_params(
        &mut self,
        params: TagPTPJump2Params,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets extended jump parameters for PTP movements.
    /// Returns current PTP jump2 parameters.
    fn get_ptp_jump2_params(&mut self) -> Result<TagPTPJump2Params, DobotError>;

    /// Executes a PTP command with multiple PO (Point Output) commands.
    ///
    /// `ptp_cmd`: The PTP command.
    /// `po_cmds`: A list of PO commands.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_po_cmd(
        &mut self,
        ptp_cmd: TagPTPCmd,
        po_cmds: &[TagPOCmd],
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Executes a PTP command with rail and PO commands.
    ///
    /// `ptp_cmd`: PTP command with rail.
    /// `po_cmds`: A list of PO commands.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_po_with_rail_cmd(
        &mut self,
        ptp_cmd: TagPTPWithLCmd,
        po_cmds: &[TagPOCmd],
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets velocity/acceleration for joints in PTP mode.
    ///
    /// `params`: Joint PTP parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_joint_params(
        &mut self,
        params: TagPTPJointParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets velocity/acceleration of Cartesian axes in PTP mode.
    ///
    /// `params`: Coordinate PTP parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_coordinate_params(
        &mut self,
        params: TagPTPCoordinateParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets lifting height for JUMP mode in PTP.
    ///
    /// `params`: Jump PTP parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_jump_params(
        &mut self,
        params: TagPTPJumpParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets common velocity/acceleration ratios for PTP mode.
    ///
    /// `params`: Common PTP parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_common_params(
        &mut self,
        params: TagPTPCommonParams,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Executes a PTP (Point-to-Point) movement command.
    ///
    /// `cmd`: PTP command parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ptp_cmd(
        &mut self,
        cmd: TagPTPCmd,
        wait: bool,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;
}
