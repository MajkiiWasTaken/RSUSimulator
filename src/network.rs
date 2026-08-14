/************************************************
* File: network.rs 
* Author: Michal Švrček
* 
* RSU network statistics 
*
* ver. 1.0.0.4
*************************************************/

use rand::Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub connected: bool,
    pub ip_address: String,
    pub mac_address: String,

    pub rx_bytes: u64,
    pub tx_bytes: u64,

    pub link_speed_mbps: u32,
}

#[derive(Clone, Serialize)]
pub struct NetworkStatus {
    pub gateway: String,
    pub dns: String,

    pub internet_available: bool,
    pub latency_ms: u32,

    pub vpn_connected: bool,
    pub vpn_ip: String,

    pub interfaces: Vec<NetworkInterface>,
}

impl NetworkStatus {
    pub fn new() -> Self {
        Self {
            gateway: "192.168.1.1".to_string(),
            dns: "1.1.1.1".to_string(),

            internet_available: true,
            latency_ms: 14,

            vpn_connected: true,
            vpn_ip: "10.100.0.10".to_string(),

            interfaces: vec![
                NetworkInterface {
                    name: "eth0".to_string(),
                    connected: true,
                    ip_address: "192.168.1.50".to_string(),
                    mac_address: "02:42:AC:11:00:01".to_string(),

                    rx_bytes: 0,
                    tx_bytes: 0,

                    link_speed_mbps: 1000,
                },

                NetworkInterface {
                    name: "eth1".to_string(),
                    connected: true,
                    ip_address: "10.52.110.200".to_string(),
                    mac_address: "02:42:AC:11:00:02".to_string(),

                    rx_bytes: 0,
                    tx_bytes: 0,

                    link_speed_mbps: 1000,
                },

                NetworkInterface {
                    name: "lte0".to_string(),
                    connected: true,
                    ip_address: "100.72.18.42".to_string(),
                    mac_address: "02:42:AC:11:00:03".to_string(),

                    rx_bytes: 0,
                    tx_bytes: 0,

                    link_speed_mbps: 150,
                },
            ],
        }
    }

    pub fn update(&mut self) {
        let mut rng =
            rand::rng();

        for interface in
            &mut self.interfaces
        {
            if interface.connected {
                interface.rx_bytes +=
                    rng.random_range(
                        5000..100000
                    );

                interface.tx_bytes +=
                    rng.random_range(
                        1000..50000
                    );
            }
        }

        self.latency_ms =
            rng.random_range(
                8..40
            );
    }
}