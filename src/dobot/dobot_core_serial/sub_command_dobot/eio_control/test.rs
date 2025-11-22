#[cfg(test)]
mod tests {
    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::eio_control::IOSerialControl, dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::io_control::IOControl,
            },
            protocol::{
                bodies::{
                    level::Level,
                    // tag_color::TagColor,
                    tag_device::{TagDevice, TagVersionColorSensorAndIR},
                    tag_emotor::{EMotorIndex, TagEMotor},
                    tag_io_do::TagIODO,
                    tag_io_multiplexing::{IOFunction, TagIOMultiplexing},
                    tag_io_pwm::TagIOPWM,
                },
                command_id::EioIDs,
                CommunicationProtocolIDs, ProtocolError,
            }, rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_io_multiplexing_ok_not_queued() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagIOMultiplexing {
            address: 1,
            multiplex: IOFunction::Pwm,
        };
        let result = io_control.set_io_multiplexing(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_io_multiplexing_ok_queued() {
        let queue_idx: u64 = 123;
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), &queue_idx.to_le_bytes());
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagIOMultiplexing {
            address: 1,
            multiplex: IOFunction::Adc,
        };
        let result = io_control.set_io_multiplexing(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_io_multiplexing_ok() {
        let response_params = [1, 2]; // address: 1, multiplex: Pwm
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IoMultiplexing), &response_params);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_multiplexing(1);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val.address, 1);
        assert_eq!(val.multiplex, IOFunction::Pwm);
    }

    #[test]
    fn test_set_io_do_ok_not_queued() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Iodo), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagIODO {
            address: 1,
            level: Level::High,
        };
        
        let result = io_control.set_io_do(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_get_io_do_ok() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Iodo), &[1]);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_do(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Level::High);
    }

    #[test]
    fn test_set_io_pwm_ok_queued() {
        let queue_idx: u64 = 456;
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IoPwm), &queue_idx.to_le_bytes());
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagIOPWM {
            address: 1,
            frequency: 1000.0,
            duty_cycle: 50.0,
        };
        let result = io_control.set_io_pwm(params, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(queue_idx));
    }

    #[test]
    fn test_get_io_pwm_ok() {
        let mut response_body = Vec::new();
        response_body.push(1); // address
        response_body.extend((1000.0f32).to_le_bytes()); // frequency
        response_body.extend((50.0f32).to_le_bytes()); // duty_cycle

        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IoPwm), &response_body);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_pwm(1);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val.address, 1);
        assert_eq!(val.frequency, 1000.0);
        assert_eq!(val.duty_cycle, 50.0);
    }

    #[test]
    fn test_get_io_di_ok() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Iodi), &[0]);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_di(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Level::Low);
    }

    #[test]
    fn test_get_io_adc_ok() {
        let adc_val: u16 = 1023;
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Iodi), &adc_val.to_le_bytes());
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_adc(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), adc_val);
    }

    #[test]
    fn test_get_io_adc_invalid_response() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Iodi), &[1]);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_adc(1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::BufferTooSmall)
        ));
    }

    #[test]
    fn test_set_e_motor_ok_not_queued() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::Emotor), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagEMotor {
            address: EMotorIndex::Stepper1,
            ins_enabled: true,
            speed: 5000.,
        };
        let result = io_control.set_e_motor(params, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_set_color_sensor_ok() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::ColorSensor), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagDevice {
            is_enabled: true,
            port: 1,
            version: TagVersionColorSensorAndIR::Version2
        };
        let result = io_control.set_color_sensor(params, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_color_sensor_ok() {
        let response_params = [255, 128, 64]; // r, g, b
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::ColorSensor), &response_params);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_color_sensor(1);
        assert!(result.is_ok());
        let color = result.unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_set_ir_switch_ok() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), b"");
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let params = TagDevice {
            port: 1,
            is_enabled: true,
            version: TagVersionColorSensorAndIR::Version2
        };
        let result = io_control.set_ir_switch(params, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_ir_switch_true() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), &[1]);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_ir_switch(1);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_ir_switch_false() {
        let mock_response = create_response_packet(CommunicationProtocolIDs::Eio(EioIDs::IrSwitch), &[0]);
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_ir_switch(1);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_communication_error() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut io_control = IOSerialControl::new(&mutex);

        let result = io_control.get_io_di(1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }
}
