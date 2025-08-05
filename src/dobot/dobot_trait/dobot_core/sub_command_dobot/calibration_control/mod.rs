use crate::dobot::dobot_trait::dobot_core::dobot_error::DobotError;

pub trait CalibrationControl {
    /// Sets static error for angle sensors.
    ///
    /// `rear_arm_angle_error`: Static error for the rear arm angle sensor.
    /// `front_arm_angle_error`: Static error for the front arm angle sensor.
    ///
    /// Returns the response message from the Dobot.
    fn set_angle_sensor_static_error(
        &mut self,
        rear_arm_angle_error: f32,
        front_arm_angle_error: f32,
    ) -> Result<(), DobotError>;

    /// Gets static error for angle sensors.
    ///
    /// Returns a tuple containing the (rear_arm_angle_error, front_arm_angle_error).
    fn get_angle_sensor_static_error(&mut self) -> Result<(f32, f32), DobotError>;
}
