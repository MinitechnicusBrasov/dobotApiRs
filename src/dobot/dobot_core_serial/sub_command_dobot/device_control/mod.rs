#[cfg(feature = "std")]
mod test;
use critical_section::Mutex;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::CommandSender, dobot_error::DobotError,
        sub_command_dobot::device_control::DeviceControl,
    },
    protocol::{
        Body, CommunicationProtocolIDs, ProtocolError,
        bodies::{general_request::GeneralRequest, tag_with_l::TagWithL},
        command_id::DeviceInfoIDs,
    },
};

pub struct DeviceSerialControl<'a, T: CommandSender> {
    command_sender: &'a mut Mutex<T>,
}

impl<'a, T: CommandSender> DeviceSerialControl<'a, T> {
    pub fn new(command_sender: &'a mut Mutex<T>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> DeviceControl for DeviceSerialControl<'a, T> {
    fn set_device_sn(&mut self, device_serial_number: &[u8]) -> Result<(), DobotError> {
        let request_body = GeneralRequest {
            params: device_serial_number,
        };

        let mut response_buffer = [0u8; 128];
        let _response = critical_section::with(|cs| {
            self.command_sender.borrow(cs);
            let borrowed = self.command_sender.get_mut();

            borrowed.send_command_with_params(
                CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn),
                false,
                request_body,
                &mut response_buffer,
            )
        })?;
        Ok(())
    }

    fn get_device_sn(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError> {
        let sender = self.command_sender.get_mut();
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn),
            true,
            GeneralRequest { params: &[] },
            buffer,
        )?;

        Ok(response_body.params.len())
    }

    fn set_device_name(&mut self, device_name: &[u8]) -> Result<(), DobotError> {
        let request_body = GeneralRequest {
            params: device_name,
        };
        let sender = self.command_sender.get_mut();
        let mut response_buffer = [0u8; 128];

        sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            false,
            request_body,
            &mut response_buffer,
        )?;
        Ok(())
    }

    fn get_device_name(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError> {
        let sender = self.command_sender.get_mut();
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name),
            true,
            GeneralRequest { params: &[] },
            buffer,
        )?;

        Ok(response_body.params.len())
    }

    fn get_device_version(&mut self) -> Result<(u8, u8, u8), DobotError> {
        let sender = self.command_sender.get_mut();
        let mut response_buffer = [0u8; 128];
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Version),
            true,
            GeneralRequest { params: &[] },
            &mut response_buffer,
        )?;

        let params = response_body.params;
        if params.len() < 3 {
            return Err(DobotError::Protocol(ProtocolError::InvalidEnumValue));
        }

        Ok((params[0], params[1], params[2]))
    }

    fn set_device_rail_capability(&mut self, params: TagWithL) -> Result<(), DobotError> {
        let mut response_buffer = [0u8; 128];
        let sender = self.command_sender.get_mut();

        sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail),
            false,
            params,
            &mut response_buffer,
        )?;
        Ok(())
    }

    fn get_device_rail_capability(&mut self) -> Result<bool, DobotError> {
        let mut response_buffer = [0u8; 128];
        let sender = self.command_sender.get_mut();
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail),
            true,
            GeneralRequest { params: &[] },
            &mut response_buffer,
        )?;

        if response_body.params.is_empty() {
            return Err(DobotError::Protocol(ProtocolError::InvalidOperation));
        }

        Ok(response_body.params[0] != 0)
    }

    fn get_device_time(&mut self) -> Result<u32, DobotError> {
        let mut response_buffer = [0u8; 128];
        let sender = self.command_sender.get_mut();
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Time),
            true,
            GeneralRequest { params: &[] },
            &mut response_buffer,
        )?;

        let params = response_body.params;
        if params.len() < 4 {
            return Err(DobotError::Protocol(ProtocolError::InvalidOperation));
        }

        let time = u32::from_le_bytes(params[0..4].try_into().unwrap());
        Ok(time)
    }

    fn get_device_id(&mut self) -> Result<(u32, u32, u32), DobotError> {
        let mut response_buffer = [0u8; 128];
        let sender = self.command_sender.get_mut();
        let response_body = sender.send_command_with_params(
            CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Id),
            true,
            GeneralRequest { params: &[] },
            &mut response_buffer,
        )?;

        let params = response_body.params;
        if params.len() < 12 {
            return Err(DobotError::Protocol(ProtocolError::InvalidOperation));
        }

        let id1 = u32::from_le_bytes(params[0..4].try_into().unwrap());
        let id2 = u32::from_le_bytes(params[4..8].try_into().unwrap());
        let id3 = u32::from_le_bytes(params[8..12].try_into().unwrap());

        Ok((id1, id2, id3))
    }
}
