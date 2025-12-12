#[cfg(test)]
mod tests {

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::trigger_control::TriggerSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::trigger_control::TriggerControl,
            },
            protocol::{
                bodies::tag_trig_cmd::{TagTRIGCmd, TriggerCondition, TriggerMode},
                command_id::TrigIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_trig_cmd_level_mode_not_queued() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 100,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_ad_mode_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            &queue_idx.to_le_bytes()
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 2,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreater,
            threshold: 500,
        };
        let result = control.set_trig_cmd(cmd, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_set_trig_cmd_all_trigger_modes() {
        let modes = [TriggerMode::Level, TriggerMode::Ad];

        for mode in modes.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = TriggerSerialControl::new(&mutex);

            let cmd = TagTRIGCmd {
                address: 1,
                mode: *mode,
                condition: TriggerCondition::LevelEqualOrAdLess,
                threshold: 100,
            };
            let result = control.set_trig_cmd(cmd, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_set_trig_cmd_all_trigger_conditions() {
        let conditions = [
            TriggerCondition::LevelEqualOrAdLess,
            TriggerCondition::LevelUnequalOrAdLessEqual,
            TriggerCondition::AdGreaterEqual,
            TriggerCondition::AdGreater,
        ];

        for condition in conditions.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = TriggerSerialControl::new(&mutex);

            let cmd = TagTRIGCmd {
                address: 1,
                mode: TriggerMode::Level,
                condition: *condition,
                threshold: 100,
            };
            let result = control.set_trig_cmd(cmd, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_set_trig_cmd_level_equal_condition() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 3,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 1,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_level_unequal_condition() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 4,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelUnequalOrAdLessEqual,
            threshold: 0,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_ad_greater_equal_condition() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 5,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreaterEqual,
            threshold: 512,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_ad_greater_condition() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 6,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreater,
            threshold: 1023,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_zero_threshold() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 0,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_max_threshold() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreater,
            threshold: u16::MAX,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_trig_cmd_various_addresses() {
        let addresses = [0u8, 1, 5, 10, 20, 100, 255];

        for address in addresses.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = TriggerSerialControl::new(&mutex);

            let cmd = TagTRIGCmd {
                address: *address,
                mode: TriggerMode::Level,
                condition: TriggerCondition::LevelEqualOrAdLess,
                threshold: 100,
            };
            let result = control.set_trig_cmd(cmd, false);
            assert!(result.is_ok(), "Failed for address: {}", address);
        }
    }

    #[test]
    fn test_set_trig_cmd_various_thresholds() {
        let thresholds = [0u16, 1, 100, 255, 256, 512, 1023, 1024, 2048, 4096, 8192, 32767, 65535];

        for threshold in thresholds.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = TriggerSerialControl::new(&mutex);

            let cmd = TagTRIGCmd {
                address: 1,
                mode: TriggerMode::Ad,
                condition: TriggerCondition::AdGreaterEqual,
                threshold: *threshold,
            };
            let result = control.set_trig_cmd(cmd, false);
            assert!(result.is_ok(), "Failed for threshold: {}", threshold);
        }
    }

    #[test]
    fn test_set_trig_cmd_queued_various_queue_indices() {
        let queue_indices = [0u64, 1, 100, 1000, 10000, u64::MAX];

        for queue_idx in queue_indices.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                &queue_idx.to_le_bytes()
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = TriggerSerialControl::new(&mutex);

            let cmd = TagTRIGCmd {
                address: 1,
                mode: TriggerMode::Level,
                condition: TriggerCondition::LevelEqualOrAdLess,
                threshold: 100,
            };
            let result = control.set_trig_cmd(cmd, true);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Some(*queue_idx));
        }
    }

    #[test]
    fn test_set_trig_cmd_communication_error_not_queued() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 100,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }


    #[test]
    fn test_set_trig_cmd_all_combinations() {
        // Test all combinations of mode and condition
        let modes = [TriggerMode::Level, TriggerMode::Ad];
        let conditions = [
            TriggerCondition::LevelEqualOrAdLess,
            TriggerCondition::LevelUnequalOrAdLessEqual,
            TriggerCondition::AdGreaterEqual,
            TriggerCondition::AdGreater,
        ];

        for mode in modes.iter() {
            for condition in conditions.iter() {
                let mock_response = create_response_packet(
                    CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
                    b""
                );
                let length = mock_response.len();
                let mock_sender = MockCommandSender::new(mock_response, Ok(length));
                let mutex = create_mock_sender_lock!(mock_sender);
                let mut control = TriggerSerialControl::new(&mutex);

                let cmd = TagTRIGCmd {
                    address: 1,
                    mode: *mode,
                    condition: *condition,
                    threshold: 100,
                };
                let result = control.set_trig_cmd(cmd, false);
                assert!(result.is_ok(), "Failed for mode: {:?}, condition: {:?}", mode, condition);
            }
        }
    }

    #[test]
    fn test_set_trig_cmd_typical_level_trigger() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        // Typical use case: trigger on digital input 1 when level equals 1
        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 1,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_trig_cmd_typical_ad_trigger() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = TriggerSerialControl::new(&mutex);

        // Typical use case: trigger on analog input when value > 512 (mid-range)
        let cmd = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreater,
            threshold: 512,
        };
        let result = control.set_trig_cmd(cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_trig_cmd_workflow_sequence() {
        // Test typical workflow: set multiple triggers in sequence
        
        // Trigger 1: Level trigger, not queued
        let mock_response1 = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            b""
        );
        let length1 = mock_response1.len();
        let mock_sender1 = MockCommandSender::new(mock_response1, Ok(length1));
        let mutex1 = create_mock_sender_lock!(mock_sender1);
        let mut control1 = TriggerSerialControl::new(&mutex1);
        
        let cmd1 = TagTRIGCmd {
            address: 1,
            mode: TriggerMode::Level,
            condition: TriggerCondition::LevelEqualOrAdLess,
            threshold: 1,
        };
        let result1 = control1.set_trig_cmd(cmd1, false);
        assert!(result1.is_ok());

        // Trigger 2: AD trigger, queued
        let queue_idx2: u64 = 1;
        let mock_response2 = create_response_packet(
            CommunicationProtocolIDs::Trig(TrigIDs::TrigCmd),
            &queue_idx2.to_le_bytes()
        );
        let length2 = mock_response2.len();
        let mock_sender2 = MockCommandSender::new(mock_response2, Ok(length2));
        let mutex2 = create_mock_sender_lock!(mock_sender2);
        let mut control2 = TriggerSerialControl::new(&mutex2);
        
        let cmd2 = TagTRIGCmd {
            address: 2,
            mode: TriggerMode::Ad,
            condition: TriggerCondition::AdGreaterEqual,
            threshold: 800,
        };
        let result2 = control2.set_trig_cmd(cmd2, true);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), Some(queue_idx2));
    }
}
