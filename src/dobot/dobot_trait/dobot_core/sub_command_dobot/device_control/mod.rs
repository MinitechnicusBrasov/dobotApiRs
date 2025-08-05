use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError, protocol::bodies::tag_with_l::TagWithL,
};

pub trait DeviceControl {
    /// Sets the device serial number.
    ///
    /// The `device_serial_number` is provided as a byte slice.
    /// The implementation should handle encoding (e.g., UTF-8) and
    /// truncation if the provided slice exceeds the device's maximum capacity.
    fn set_device_sn(&mut self, device_serial_number: &[u8]) -> Result<(), DobotError>;

    /// Gets the device serial number and writes it into the provided buffer.
    ///
    /// Returns the number of bytes written to the buffer, or an error if the
    /// buffer is too small or communication fails. The returned bytes are expected
    /// to be UTF-8 encoded.
    fn get_device_sn(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError>;

    /// Sets the device name.
    ///
    /// The `device_name` is provided as a byte slice.
    /// The implementation should handle encoding (e.g., UTF-8) and
    /// truncation if the provided slice exceeds the device's maximum capacity.
    fn set_device_name(&mut self, device_name: &[u8]) -> Result<(), DobotError>;

    /// Gets the device name and writes it into the provided buffer.
    ///
    /// Returns the number of bytes written to the buffer, or an error if the
    /// buffer is too small or communication fails. The returned bytes are expected
    /// to be UTF-8 encoded.
    fn get_device_name(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError>;

    /// Gets the device firmware version.
    /// Returns a tuple (Major version, Minor version, Revision version).
    fn get_device_version(&mut self) -> Result<(u8, u8, u8), DobotError>;

    /// Sets the device's rail capability.
    fn set_device_rail_capability(&mut self, params: TagWithL) -> Result<(), DobotError>;

    /// Gets device's rail capability status.
    /// Returns True if rail enabled/present, False otherwise.
    fn get_device_rail_capability(&mut self) -> Result<bool, DobotError>;

    /// Gets the device's internal time (system tick).
    /// Returns System Tick (uint32_t).
    fn get_device_time(&mut self) -> Result<u32, DobotError>;

    /// Gets the device ID.
    /// Returns Device ID as a tuple of 3 uint32_t values.
    fn get_device_id(&mut self) -> Result<(u32, u32, u32), DobotError>;
}
