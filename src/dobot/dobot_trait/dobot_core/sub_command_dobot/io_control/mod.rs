use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{
        level::Level, tag_color::TagColor, tag_device::TagDevice, tag_emotor::TagEMotor,
        tag_io_do::TagIODO, tag_io_multiplexing::TagIOMultiplexing, tag_io_pwm::TagIOPWM,
    },
};

pub trait IOControl {
    /// Sets I/O multiplexing configuration.
    ///
    /// `params`: I/O multiplexing parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_io_multiplexing(
        &mut self,
        params: TagIOMultiplexing,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets I/O multiplexing configuration.
    ///
    /// `address`: The address of the I/O multiplexing to get.
    ///
    /// Returns the current I/O multiplexing configuration.
    fn get_io_multiplexing(&mut self, address: u8) -> Result<TagIOMultiplexing, DobotError>;

    /// Sets digital output (DO) for a specific I/O.
    ///
    /// `params`: I/O DO parameters (address, level).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_io_do(
        &mut self,
        params: TagIODO,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets digital output (DO) status for an I/O.
    ///
    /// `address`: The EIO address (1-20) to get the level of.
    ///
    /// Returns the Level of the DO.
    fn get_io_do(&mut self, address: u8) -> Result<Level, DobotError>;

    /// Sets PWM output for a specific I/O.
    ///
    /// `params`: I/O PWM parameters (address, frequency, dutyCycle).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_io_pwm(
        &mut self,
        params: TagIOPWM,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets PWM output status for an I/O.
    ///
    /// `address`: The EIO address (1-20) to get the PWM configuration of.
    ///
    /// Returns the PWM configuration.
    fn get_io_pwm(&mut self, address: u8) -> Result<TagIOPWM, DobotError>;

    /// Gets digital input (DI) status for an I/O.
    ///
    /// `address`: The EIO address (1-20) to get the level of.
    ///
    /// Returns the Level of the DI.
    fn get_io_di(&mut self, address: u8) -> Result<Level, DobotError>;

    /// Gets ADC value for an I/O.
    ///
    /// `address`: The EIO address (1-20) to get the ADC value of.
    ///
    /// Returns the ADC value (0-4095).
    fn get_io_adc(&mut self, address: u8) -> Result<u16, DobotError>;

    /// Controls an external motor (stepper).
    ///
    /// `params`: External motor parameters.
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_e_motor(
        &mut self,
        params: TagEMotor,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Sets parameters for the color sensor.
    ///
    /// `params`: Device parameters for color sensor (isEnable, port, version).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_color_sensor(
        &mut self,
        params: TagDevice,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets readings (R, G, B) from the color sensor.
    ///
    /// `port`: The port the color sensor is connected to.
    ///
    /// Returns a tuple of (r, g, b) color values (u8 each).
    fn get_color_sensor(&mut self, port: u8) -> Result<TagColor, DobotError>;

    /// Sets parameters for the IR (Infrared) switch.
    ///
    /// `params`: Device parameters for IR switch (isEnable, port, version).
    /// `wait`: If `true` and command is queued, waits for execution.
    /// `is_queued`: If `true`, command is added to the queue.
    ///
    /// Returns the queued command index if `is_queued` is `true`, otherwise `None`.
    fn set_ir_switch(
        &mut self,
        params: TagDevice,
        is_queued: bool,
    ) -> Result<Option<u64>, DobotError>;

    /// Gets the status of the IR switch.
    ///
    /// `port`: The port the IR switch is connected to.
    ///
    /// Returns `true` if IR switch is triggered/active, `false` otherwise.
    fn get_ir_switch(&mut self, port: u8) -> Result<bool, DobotError>;
}
