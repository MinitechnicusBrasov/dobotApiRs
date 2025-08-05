use crate::dobot::dobot_trait::{
    dobot_core::dobot_error::DobotError,
    protocol::bodies::{
        tag_wifi_dns::TagWIFIDNS, tag_wifi_gateway::TagWIFIGateway,
        tag_wifi_ip_address::TagWIFIIPAddress, tag_wifi_netmask::TagWIFINetmask,
    },
};

pub trait WifiControl {
    /// Enables or disables Wi-Fi configuration mode.
    ///
    /// `enable`: `true` to enable, `false` to disable.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_config_mode(&mut self, enable: bool) -> Result<(), DobotError>;

    /// Gets Wi-Fi configuration mode status.
    ///
    /// Returns `true` if Wi-Fi config mode is enabled, `false` otherwise.
    fn get_wifi_config_mode(&mut self) -> Result<bool, DobotError>;

    /// Sets the Wi-Fi SSID.
    ///
    /// `ssid`: The Wi-Fi SSID as a byte slice.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_ssid(&mut self, ssid: &[u8]) -> Result<(), DobotError>;

    /// Gets the Wi-Fi SSID.
    ///
    /// `buffer`: A mutable slice to store the SSID.
    ///
    /// Returns the number of bytes written to the buffer, or an error.
    fn get_wifi_ssid<'a>(&'a mut self, buffer: &'a mut [u8]) -> Result<&'a str, DobotError>;

    /// Sets the Wi-Fi password.
    ///
    /// `password`: The Wi-Fi password as a byte slice.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_password(&mut self, password: &[u8]) -> Result<(), DobotError>;

    /// Gets the Wi-Fi password.
    ///
    /// `buffer`: A mutable slice to store the password.
    ///
    /// Returns the number of bytes written to the buffer, or an error.
    fn get_wifi_password<'a>(&'a mut self, buffer: &'a mut [u8]) -> Result<&'a str, DobotError>;

    /// Sets Wi-Fi IP address settings.
    ///
    /// `params`: WIFIIPAddress structure.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_ip_address(&mut self, params: TagWIFIIPAddress) -> Result<(), DobotError>;

    /// Gets Wi-Fi IP address settings.
    ///
    /// Returns the WIFIIPAddress structure.
    fn get_wifi_ip_address(&mut self) -> Result<TagWIFIIPAddress, DobotError>;

    /// Sets Wi-Fi netmask settings.
    ///
    /// `params`: WIFINetmask structure.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_netmask(&mut self, params: TagWIFINetmask) -> Result<(), DobotError>;

    /// Gets Wi-Fi netmask settings.
    ///
    /// Returns the WIFINetmask structure.
    fn get_wifi_netmask(&mut self) -> Result<TagWIFINetmask, DobotError>;

    /// Sets Wi-Fi gateway settings.
    ///
    /// `params`: WIFIGateway structure.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_gateway(&mut self, params: TagWIFIGateway) -> Result<(), DobotError>;

    /// Gets Wi-Fi gateway settings.
    ///
    /// Returns the WIFIGateway structure.
    fn get_wifi_gateway(&mut self) -> Result<TagWIFINetmask, DobotError>;

    /// Sets Wi-Fi DNS settings.
    ///
    /// `params`: WIFIDNS structure.
    ///
    /// Returns the response message from the Dobot.
    fn set_wifi_dns(&mut self, params: TagWIFIDNS) -> Result<(), DobotError>;

    /// Gets Wi-Fi DNS settings.
    ///
    /// Returns the WIFIDNS structure.
    fn get_wifi_dns(&mut self) -> Result<TagWIFIDNS, DobotError>;

    /// Gets Wi-Fi connection status.
    ///
    /// Returns `true` if connected, `false` otherwise.
    fn get_wifi_connect_status(&mut self) -> Result<bool, DobotError>;
}
