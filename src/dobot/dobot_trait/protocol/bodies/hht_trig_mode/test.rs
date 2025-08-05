#[cfg(test)]
mod tests {
    use crate::dobot::dobot_trait::protocol::bodies::hht_trig_mode::HHTTrigMode;
    use crate::dobot::dobot_trait::protocol::protocol_error::ProtocolError;

    /// Test case to ensure the discriminant value of a specific HHTTrigMode is correct.
    #[test]
    fn test_hht_trig_mode_discriminant_value() {
        // Assert that the byte value of a specific mode is as expected.
        assert_eq!(HHTTrigMode::TriggeredOnKeyRelease as u8, 0x0);
        assert_eq!(HHTTrigMode::TriggeredOnPeriodicInterval as u8, 0x1);
    }

    /// Test case for a successful conversion from a valid `u8` to an HHTTrigMode.
    #[test]
    fn test_hht_trig_mode_try_from_u8_success() {
        // Test `TriggeredOnKeyRelease`
        let byte_value_release = 0x0;
        let expected_mode_release = HHTTrigMode::TriggeredOnKeyRelease;
        let result_release = HHTTrigMode::try_from_u8(byte_value_release);
        assert_eq!(result_release, Ok(expected_mode_release));

        // Test `TriggeredOnPeriodicInterval`
        let byte_value_interval = 0x1;
        let expected_mode_interval = HHTTrigMode::TriggeredOnPeriodicInterval;
        let result_interval = HHTTrigMode::try_from_u8(byte_value_interval);
        assert_eq!(result_interval, Ok(expected_mode_interval));
    }

    /// Test case for a failed conversion from an invalid `u8` to an HHTTrigMode.
    #[test]
    fn test_hht_trig_mode_try_from_u8_failure() {
        let invalid_byte_value = 0x02;
        let result = HHTTrigMode::try_from_u8(invalid_byte_value);

        // Assert that the conversion failed with the expected error.
        assert_eq!(
            result,
            Err(ProtocolError::InvalidHHTTrigMode(invalid_byte_value))
        );
    }
}
