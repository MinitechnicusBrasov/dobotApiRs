use crate::dobot::dobot_trait::dobot_core::dobot_error::DobotError;

pub trait QueueControl {
    /// Forces stop of command execution in the queue.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_force_stop_exec(&mut self) -> Result<(), DobotError>;

    /// Starts downloading commands to the queue for offline execution.
    ///
    /// `total_loop`: Total number of loops for command download.
    /// `line_per_loop`: Number of lines per loop for command download.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_start_download(
        &mut self,
        total_loop: u32,
        line_per_loop: u32,
    ) -> Result<(), DobotError>;

    /// Stops downloading commands to the queue.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_stop_download(&mut self) -> Result<(), DobotError>;

    /// Starts execution of commands in the queue.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_start_exec(&mut self) -> Result<(), DobotError>;

    /// Stops execution of commands in the queue.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_stop_exec(&mut self) -> Result<(), DobotError>;

    /// Clears the command queue.
    ///
    /// Returns the response message from the Dobot.
    fn set_queued_cmd_clear(&mut self) -> Result<(), DobotError>;

    /// Retrieves the current index of the command being executed in the queue.
    ///
    /// Returns the current command index.
    fn get_queued_cmd_current_index(&mut self) -> Result<u64, DobotError>;
}
