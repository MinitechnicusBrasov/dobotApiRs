use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError, protocol::bodies::hht_trig_mode::HHTTrigMode,
};

pub trait HandholdTeachingControl {
    /// Sets Hand Hold Teaching trigger mode.
    ///
    /// `mode`: The HHT trigger mode.
    fn set_hht_trig_mode(&mut self, mode: HHTTrigMode) -> Result<(), DobotError>;

    /// Gets Hand Hold Teaching trigger mode.
    /// Returns current `HHTTrigMode`.
    fn get_hht_trig_mode(&mut self) -> Result<HHTTrigMode, DobotError>;

    /// Enables/disables Hand Hold Teaching trigger output.
    ///
    /// `is_enabled`: `true` to enable, `false` to disable.
    fn set_hht_trig_output_enabled(&mut self, is_enabled: bool) -> Result<(), DobotError>;

    /// Checks if Hand Hold Teaching trigger output is enabled.
    /// Returns `true` if enabled, `false` otherwise.
    fn get_hht_trig_output_enabled(&mut self) -> Result<bool, DobotError>;

    /// Gets current Hand Hold Teaching trigger output value.
    /// Returns `true` if triggered, `false` otherwise.
    fn get_hht_trig_output(&mut self) -> Result<bool, DobotError>;
}
