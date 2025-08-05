mod test;

use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

/// Represents the trigger mode for Handhold Teaching (HHT).
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum HHTTrigMode {
    /// The teaching mode is triggered when a key is released.
    TriggeredOnKeyRelease = 0x0,
    /// The teaching mode is triggered on a periodic interval.
    TriggeredOnPeriodicInterval = 0x1,
}

impl HHTTrigMode {
    /// Attempts to convert a `u8` into an `HHTTrigMode` enum variant.
    ///
    /// This function performs a manual lookup and returns a `Result` to
    /// handle cases where the input byte does not correspond to a known
    /// trigger mode.
    pub fn try_from_u8(value: u8) -> Result<Self, ProtocolError> {
        match value {
            0x0 => Ok(HHTTrigMode::TriggeredOnKeyRelease),
            0x1 => Ok(HHTTrigMode::TriggeredOnPeriodicInterval),
            code => Err(ProtocolError::InvalidHHTTrigMode(code)),
        }
    }
}
