#[cfg(test)]
mod tests {

    use crate::dobot::{
        dobot_core_serial::sub_command_dobot::wifi_control::WifiSerialControl,
        dobot_trait::{
            dobot_core::{
                command_sender::{mock_command_sender::{create_response_packet, MockCommandSender}, Dobot},
                dobot_error::DobotError,
                sub_command_dobot::wifi_control::WifiControl,
            },
            protocol::{
                bodies::{
                    tag_wifi_dns::TagWIFIDNS,
                    tag_wifi_gateway::TagWIFIGateway,
                    tag_wifi_ip_address::TagWIFIIPAddress,
                    tag_wifi_netmask::TagWIFINetmask,
                },
                command_id::WifiIDs,
                CommunicationProtocolIDs, ProtocolError,
            },
            rwlock::RwLock,
        }
    };

    #[test]
    fn test_set_wifi_config_mode_enabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.set_wifi_config_mode(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_wifi_config_mode_disabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.set_wifi_config_mode(false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_config_mode_enabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            &[1u8]
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_config_mode();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_wifi_config_mode_disabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            &[0u8]
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_config_mode();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_set_wifi_ssid_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Ssid),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let ssid = b"MyWiFiNetwork";
        let result = control.set_wifi_ssid(ssid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_wifi_ssid_empty() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Ssid),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let ssid = b"";
        let result = control.set_wifi_ssid(ssid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_ssid_ok() {
        let ssid_data = b"TestNetwork";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Ssid),
            ssid_data
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let mut buffer = [0u8; 64];
        let result = control.get_wifi_ssid(&mut buffer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "TestNetwork");
    }

    #[test]
    fn test_set_wifi_password_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Password),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let password = b"SecurePassword123";
        let result = control.set_wifi_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_wifi_password_empty() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Password),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let password = b"";
        let result = control.set_wifi_password(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_password_ok() {
        let password_data = b"MyPassword";
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Password),
            password_data
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let mut buffer = [0u8; 64];
        let result = control.get_wifi_password(&mut buffer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "MyPassword");
    }

    #[test]
    fn test_set_wifi_ip_address_dhcp_enabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let params = TagWIFIIPAddress {
            dhcp: true,
            addr: [192, 168, 1, 100],
        };
        let result = control.set_wifi_ip_address(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_wifi_ip_address_dhcp_disabled() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let params = TagWIFIIPAddress {
            dhcp: false,
            addr: [10, 0, 0, 50],
        };
        let result = control.set_wifi_ip_address(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_ip_address_ok() {
        let mut response_body = Vec::new();
        response_body.push(1u8); // dhcp enabled
        response_body.extend(&[192, 168, 1, 100]);

        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress),
            &response_body
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_ip_address();
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert!(ip.dhcp);
        assert_eq!(ip.addr, [192, 168, 1, 100]);
    }

    #[test]
    fn test_set_wifi_netmask_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Netmask),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let params = TagWIFINetmask {
            addr: [255, 255, 255, 0],
        };
        let result = control.set_wifi_netmask(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_netmask_ok() {
        let netmask_data = [255, 255, 255, 0];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Netmask),
            &netmask_data
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_netmask();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().addr, [255, 255, 255, 0]);
    }

    #[test]
    fn test_set_wifi_gateway_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Gateway),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let params = TagWIFIGateway {
            addr: [192, 168, 1, 1],
        };
        let result = control.set_wifi_gateway(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_gateway_ok() {
        let gateway_data = [192, 168, 1, 1];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Gateway),
            &gateway_data
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_gateway();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().addr, [192, 168, 1, 1]);
    }

    #[test]
    fn test_set_wifi_dns_ok() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Dns),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let params = TagWIFIDNS {
            addr: [8, 8, 8, 8],
        };
        let result = control.set_wifi_dns(params);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_dns_ok() {
        let dns_data = [8, 8, 8, 8];
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Dns),
            &dns_data
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_dns();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().addr, [8, 8, 8, 8]);
    }

    #[test]
    fn test_get_wifi_connect_status_connected() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConnectStatus),
            &[1u8]
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_connect_status();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_get_wifi_connect_status_disconnected() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConnectStatus),
            &[0u8]
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_connect_status();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_set_wifi_ip_address_various_addresses() {
        let addresses = [
            [192, 168, 1, 1],
            [10, 0, 0, 1],
            [172, 16, 0, 1],
            [127, 0, 0, 1],
            [0, 0, 0, 0],
            [255, 255, 255, 255],
        ];

        for addr in addresses.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WifiSerialControl::new(&mutex);

            let params = TagWIFIIPAddress {
                dhcp: false,
                addr: *addr,
            };
            let result = control.set_wifi_ip_address(params);
            assert!(result.is_ok(), "Failed for address: {:?}", addr);
        }
    }

    #[test]
    fn test_set_wifi_netmask_various_masks() {
        let netmasks = [
            [255, 255, 255, 255],
            [255, 255, 255, 0],
            [255, 255, 0, 0],
            [255, 0, 0, 0],
            [255, 255, 255, 128],
        ];

        for mask in netmasks.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wifi(WifiIDs::Netmask),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WifiSerialControl::new(&mutex);

            let params = TagWIFINetmask {
                addr: *mask,
            };
            let result = control.set_wifi_netmask(params);
            assert!(result.is_ok(), "Failed for netmask: {:?}", mask);
        }
    }

    #[test]
    fn test_set_wifi_dns_common_servers() {
        let dns_servers = [
            [8, 8, 8, 8],          // Google DNS
            [8, 8, 4, 4],          // Google DNS secondary
            [1, 1, 1, 1],          // Cloudflare DNS
            [208, 67, 222, 222],   // OpenDNS
        ];

        for dns in dns_servers.iter() {
            let mock_response = create_response_packet(
                CommunicationProtocolIDs::Wifi(WifiIDs::Dns),
                b""
            );
            let length = mock_response.len();
            let mock_sender = MockCommandSender::new(mock_response, Ok(length));
            let mutex = create_mock_sender_lock!(mock_sender);
            let mut control = WifiSerialControl::new(&mutex);

            let params = TagWIFIDNS {
                addr: *dns,
            };
            let result = control.set_wifi_dns(params);
            assert!(result.is_ok(), "Failed for DNS: {:?}", dns);
        }
    }

    #[test]
    fn test_communication_error_set_config_mode() {
        let mock_sender = MockCommandSender::new(
            Vec::new(),
            Err(DobotError::Protocol(ProtocolError::ChecksumError)),
        );
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.set_wifi_config_mode(true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DobotError::Protocol(ProtocolError::ChecksumError)
        ));
    }

    #[test]
    fn test_wifi_configuration_workflow() {
        // Test typical WiFi configuration workflow
        
        // 1. Enable config mode
        let mock_response1 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            b""
        );
        let length1 = mock_response1.len();
        let mock_sender1 = MockCommandSender::new(mock_response1, Ok(length1));
        let mutex1 = create_mock_sender_lock!(mock_sender1);
        let mut control1 = WifiSerialControl::new(&mutex1);
        let result1 = control1.set_wifi_config_mode(true);
        assert!(result1.is_ok());

        // 2. Set SSID
        let mock_response2 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Ssid),
            b""
        );
        let length2 = mock_response2.len();
        let mock_sender2 = MockCommandSender::new(mock_response2, Ok(length2));
        let mutex2 = create_mock_sender_lock!(mock_sender2);
        let mut control2 = WifiSerialControl::new(&mutex2);
        let result2 = control2.set_wifi_ssid(b"MyNetwork");
        assert!(result2.is_ok());

        // 3. Set password
        let mock_response3 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Password),
            b""
        );
        let length3 = mock_response3.len();
        let mock_sender3 = MockCommandSender::new(mock_response3, Ok(length3));
        let mutex3 = create_mock_sender_lock!(mock_sender3);
        let mut control3 = WifiSerialControl::new(&mutex3);
        let result3 = control3.set_wifi_password(b"SecurePass");
        assert!(result3.is_ok());

        // 4. Set static IP
        let mock_response4 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::IpAddress),
            b""
        );
        let length4 = mock_response4.len();
        let mock_sender4 = MockCommandSender::new(mock_response4, Ok(length4));
        let mutex4 = create_mock_sender_lock!(mock_sender4);
        let mut control4 = WifiSerialControl::new(&mutex4);
        let ip_params = TagWIFIIPAddress {
            dhcp: false,
            addr: [192, 168, 1, 100],
        };
        let result4 = control4.set_wifi_ip_address(ip_params);
        assert!(result4.is_ok());

        // 5. Set netmask
        let mock_response5 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Netmask),
            b""
        );
        let length5 = mock_response5.len();
        let mock_sender5 = MockCommandSender::new(mock_response5, Ok(length5));
        let mutex5 = create_mock_sender_lock!(mock_sender5);
        let mut control5 = WifiSerialControl::new(&mutex5);
        let netmask_params = TagWIFINetmask {
            addr: [255, 255, 255, 0],
        };
        let result5 = control5.set_wifi_netmask(netmask_params);
        assert!(result5.is_ok());

        // 6. Set gateway
        let mock_response6 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Gateway),
            b""
        );
        let length6 = mock_response6.len();
        let mock_sender6 = MockCommandSender::new(mock_response6, Ok(length6));
        let mutex6 = create_mock_sender_lock!(mock_sender6);
        let mut control6 = WifiSerialControl::new(&mutex6);
        let gateway_params = TagWIFIGateway {
            addr: [192, 168, 1, 1],
        };
        let result6 = control6.set_wifi_gateway(gateway_params);
        assert!(result6.is_ok());

        // 7. Set DNS
        let mock_response7 = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Dns),
            b""
        );
        let length7 = mock_response7.len();
        let mock_sender7 = MockCommandSender::new(mock_response7, Ok(length7));
        let mutex7 = create_mock_sender_lock!(mock_sender7);
        let mut control7 = WifiSerialControl::new(&mutex7);
        let dns_params = TagWIFIDNS {
            addr: [8, 8, 8, 8],
        };
        let result7 = control7.set_wifi_dns(dns_params);
        assert!(result7.is_ok());
    }

    #[test]
    fn test_set_wifi_ssid_with_special_characters() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::Ssid),
            b""
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let ssid = b"Test-Network_5G";
        let result = control.set_wifi_ssid(ssid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_wifi_config_mode_nonzero_as_true() {
        let mock_response = create_response_packet(
            CommunicationProtocolIDs::Wifi(WifiIDs::ConfigMode),
            &[255u8]
        );
        let length = mock_response.len();
        let mock_sender = MockCommandSender::new(mock_response, Ok(length));
        let mutex = create_mock_sender_lock!(mock_sender);
        let mut control = WifiSerialControl::new(&mutex);

        let result = control.get_wifi_config_mode();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
