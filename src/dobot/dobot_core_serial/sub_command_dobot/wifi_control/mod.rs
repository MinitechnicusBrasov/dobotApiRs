#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            wifi_control::WifiControl
        ,
    },
    protocol::{
        bodies::{
            general_request::GeneralRequest, general_response::GeneralResponse, tag_empty_body::EmptyBody, tag_wifi_dns::TagWIFIDNS, tag_wifi_gateway::TagWIFIGateway, tag_wifi_ip_address::TagWIFIIPAddress, tag_wifi_netmask::TagWIFINetmask
        }, command_id::WifiIDs, CommunicationProtocolIDs
    },
    rwlock::RwLock,
};

pub struct WifiSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> WifiSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> WifiControl for WifiSerialControl<'a, T> {
    fn set_wifi_config_mode(&mut self, enable: bool) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request_buffer = [enable as u8; 1];
        let request_body = GeneralRequest { params: &request_buffer };
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode), request_body, write=true)?;
        Ok(())
    }

    fn get_wifi_config_mode(&mut self) -> Result<bool, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, EmptyBody, GeneralResponse, CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode), EmptyBody {  }, &mut response_buffer)?;
        let enabled = response_body.params[0] != 0;

        Ok(enabled)
    }

    fn set_wifi_ssid(&mut self, ssid: &[u8]) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request_body = GeneralRequest { params: ssid };
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Wifi(WifiIDs::Ssid), request_body, write=true)?;
        Ok(())
    }

    fn get_wifi_ssid<'b>(&'b mut self, buffer: &'b mut [u8]) -> Result<&'b str, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, EmptyBody, GeneralResponse, CommunicationProtocolIDs::Wifi(WifiIDs::Ssid), EmptyBody {  }, buffer)?;

        Ok(core::str::from_utf8(response_body.params)?)
    }

    fn set_wifi_password(&mut self, password: &[u8]) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let request_body = GeneralRequest { params: password };
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Wifi(WifiIDs::Password), request_body, write=true)?;
        Ok(())
    }

    fn get_wifi_password<'b>(&'b mut self, buffer: &'b mut [u8]) -> Result<&'b str, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let response_body = send_cmd!(get sender, EmptyBody, GeneralResponse, CommunicationProtocolIDs::Wifi(WifiIDs::Password), EmptyBody {  }, buffer)?;

        Ok(core::str::from_utf8(response_body.params)?)
    }

    fn set_wifi_ip_address(
        &mut self,
        params: TagWIFIIPAddress,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, TagWIFIIPAddress, CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress), params, write=true)?;
        Ok(())
    }

    fn get_wifi_ip_address(
        &mut self,
    ) -> Result<
        TagWIFIIPAddress,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 5];
        let response_body = send_cmd!(get sender, EmptyBody, TagWIFIIPAddress, CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress), EmptyBody {  }, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_wifi_netmask(
        &mut self,
        params: TagWIFINetmask,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, TagWIFINetmask, CommunicationProtocolIDs::Wifi(WifiIDs::Netmask), params, write=true)?;
        Ok(())
    }

    fn get_wifi_netmask(
        &mut self,
    ) -> Result<
        TagWIFINetmask,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 4];
        let response_body = send_cmd!(get sender, EmptyBody, TagWIFINetmask, CommunicationProtocolIDs::Wifi(WifiIDs::Netmask), EmptyBody {  }, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_wifi_gateway(
        &mut self,
        params: TagWIFIGateway,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, TagWIFIGateway, CommunicationProtocolIDs::Wifi(WifiIDs::Gateway), params, write=true)?;
        Ok(())
    }

    fn get_wifi_gateway(
        &mut self,
    ) -> Result<
        TagWIFIGateway,
        DobotError,
    > {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 4];
        let response_body = send_cmd!(get sender, EmptyBody, TagWIFIGateway, CommunicationProtocolIDs::Wifi(WifiIDs::Gateway), EmptyBody {  }, &mut response_buffer)?;

        Ok(response_body)
    }

    fn set_wifi_dns(
        &mut self,
        params: TagWIFIDNS,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, TagWIFIDNS, CommunicationProtocolIDs::Wifi(WifiIDs::Dns), params, write=true)?;
        Ok(())
    }

    fn get_wifi_dns(
        &mut self,
    ) -> Result<TagWIFIDNS, DobotError>
    {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 4];
        let response_body = send_cmd!(get sender, EmptyBody, TagWIFIDNS, CommunicationProtocolIDs::Wifi(WifiIDs::Dns), EmptyBody {  }, &mut response_buffer)?;

        Ok(response_body)
    }

    fn get_wifi_connect_status(&mut self) -> Result<bool, DobotError> {
let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 1];
        let response_body = send_cmd!(get sender, EmptyBody, GeneralResponse, CommunicationProtocolIDs::Wifi(WifiIDs::ConnectStatus), EmptyBody {  }, &mut response_buffer)?;
        let enabled = response_body.params[0] != 0;

        Ok(enabled)
    }
}
