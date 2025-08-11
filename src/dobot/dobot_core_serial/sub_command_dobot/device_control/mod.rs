#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::device_control::DeviceControl,
    },
    protocol::{
        Body, CommunicationProtocolIDs, ProtocolError,
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse,
            tag_empty_body::EmptyBody, tag_with_l::TagWithL,
        },
        command_id::DeviceInfoIDs,
    },
    rwlock::RwLock,
};

pub struct DeviceSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> DeviceSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> DeviceControl for DeviceSerialControl<'a, T> {
    fn set_device_sn(&mut self, device_serial_number: &[u8]) -> Result<(), DobotError> {
        let request_body = GeneralRequest {
            params: device_serial_number,
        };

        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn), request_body, write=true)?;

        Ok(())
    }

    fn get_device_sn(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Sn), buffer)?;

        Ok(response_body.params.len())
    }

    fn set_device_name(&mut self, device_name: &[u8]) -> Result<(), DobotError> {
        let request_body = GeneralRequest {
            params: device_name,
        };
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name), request_body, write=true)?;

        Ok(())
    }

    fn get_device_name(&mut self, buffer: &mut [u8]) -> Result<usize, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Name), buffer)?;
        Ok(response_body.params.len())
    }

    fn get_device_version(&mut self) -> Result<(u8, u8, u8), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 3];
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Version), &mut response_buffer)?;

        let params = response_body.params;
        if params.len() < 3 {
            return Err(DobotError::Protocol(ProtocolError::InvalidEnumValue));
        }

        Ok((params[0], params[1], params[2]))
    }

    fn set_device_rail_capability(&mut self, params: TagWithL) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;

        send_cmd!(send sender, TagWithL, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail), params, write=true)?;
        Ok(())
    }

    fn get_device_rail_capability(&mut self) -> Result<bool, DobotError> {
        let mut response_buffer = [0u8; 1];
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::WithRail), &mut response_buffer)?;

        if response_body.params.is_empty() {
            return Err(DobotError::Protocol(ProtocolError::InvalidOperation));
        }

        Ok(response_body.params[0] != 0)
    }

    fn get_device_time(&mut self) -> Result<u32, DobotError> {
        let mut response_buffer = [0u8; 4];
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Time), &mut response_buffer)?;

        let params = response_body.params;
        if params.len() < 4 {
            return Err(DobotError::Protocol(ProtocolError::InvalidOperation));
        }

        let time = u32::from_le_bytes(params[0..4].try_into().unwrap());
        Ok(time)
    }

    fn get_device_id(&mut self) -> Result<(u32, u32, u32), DobotError> {
        let mut response_buffer = [0u8; 12];
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DeviceInfo(DeviceInfoIDs::Id), &mut response_buffer)?;

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
