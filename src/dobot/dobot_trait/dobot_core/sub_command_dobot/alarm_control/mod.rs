use crate::dobot::dobot_trait::{dobot_core::dobot_error::DobotError, protocol::alarm::Alarm};

pub trait AlarmControl {
    /// Gets the current active alarms of the Dobot.
    ///
    /// Returns a `[Alarm; 16]` array representing the alarm state.
    /// Each bit in the 16 bytes corresponds to an alarm state.
    fn get_active_alarms(&mut self) -> Result<[Option<Alarm>; 128], DobotError>;

    /// Clears all alarm states of the Dobot.
    fn clear_all_alarms_state(&mut self) -> Result<(), DobotError>;
}
