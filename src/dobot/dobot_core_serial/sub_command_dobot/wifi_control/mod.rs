#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::{
            cp_control::CPControl, home_control::HomeControl, jog_control::JOGControl,
            ptp_control::PTPControl, wifi_control::WifiControl,
        },
    },
    protocol::{
        CommunicationProtocolIDs, ProtocolError,
        bodies::{
            general_response::GeneralResponse, tag_auto_leveling_params::TagAutoLevelingParams,
            tag_empty_body::EmptyBody, tag_home_cmd::TagHomeCmd, tag_home_params::TagHomeParams,
            tag_queue::received::TagQueue,
        },
        command_id::HomeIDs,
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
        todo!()
    }

    fn get_wifi_config_mode(&mut self) -> Result<bool, DobotError> {
        todo!()
    }

    fn set_wifi_ssid(&mut self, ssid: &[u8]) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_ssid<'b>(&'b mut self, buffer: &'b mut [u8]) -> Result<&'b str, DobotError> {
        todo!()
    }

    fn set_wifi_password(&mut self, password: &[u8]) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_password<'b>(&'b mut self, buffer: &'b mut [u8]) -> Result<&'b str, DobotError> {
        todo!()
    }

    fn set_wifi_ip_address(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_wifi_ip_address::TagWIFIIPAddress,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_ip_address(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_wifi_ip_address::TagWIFIIPAddress,
        DobotError,
    > {
        todo!()
    }

    fn set_wifi_netmask(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_wifi_netmask::TagWIFINetmask,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_netmask(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_wifi_netmask::TagWIFINetmask,
        DobotError,
    > {
        todo!()
    }

    fn set_wifi_gateway(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_wifi_gateway::TagWIFIGateway,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_gateway(
        &mut self,
    ) -> Result<
        crate::dobot::dobot_trait::protocol::bodies::tag_wifi_netmask::TagWIFINetmask,
        DobotError,
    > {
        todo!()
    }

    fn set_wifi_dns(
        &mut self,
        params: crate::dobot::dobot_trait::protocol::bodies::tag_wifi_dns::TagWIFIDNS,
    ) -> Result<(), DobotError> {
        todo!()
    }

    fn get_wifi_dns(
        &mut self,
    ) -> Result<crate::dobot::dobot_trait::protocol::bodies::tag_wifi_dns::TagWIFIDNS, DobotError>
    {
        todo!()
    }

    fn get_wifi_connect_status(&mut self) -> Result<bool, DobotError> {
        todo!()
    }
}
