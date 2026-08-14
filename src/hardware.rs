/************************************************
* File: hardware.rs 
* Author: Michal Švrček
* 
* RSU hardware logic
*
* ver. 1.0.0.4
*************************************************/

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct HardwareComponent {
    pub name: String,
    pub component_type: String,
    pub status: String,
    pub details: String,
}

pub fn get_hardware() -> Vec<HardwareComponent> {
    vec![
        HardwareComponent {
            name: "Compute Module 5".to_string(),
            component_type: "CPU".to_string(),
            status: "ok".to_string(),
            details: "CM5 operational".to_string(),
        },

        HardwareComponent {
            name: "GNSS".to_string(),
            component_type: "GNSS".to_string(),
            status: "ok".to_string(),
            details: "GNSS receiver detected".to_string(),
        },

        HardwareComponent {
            name: "V2X Modem".to_string(),
            component_type: "V2X".to_string(),
            status: "ok".to_string(),
            details: "Modem communication active".to_string(),
        },

        HardwareComponent {
            name: "SSD".to_string(),
            component_type: "Storage".to_string(),
            status: "ok".to_string(),
            details: "NVMe SSD detected".to_string(),
        },

        HardwareComponent {
            name: "Ethernet PHY 1".to_string(),
            component_type: "Network".to_string(),
            status: "ok".to_string(),
            details: "1 Gbit link".to_string(),
        },

        HardwareComponent {
            name: "Ethernet PHY 2".to_string(),
            component_type: "Network".to_string(),
            status: "ok".to_string(),
            details: "1 Gbit link".to_string(),
        },

        HardwareComponent {
            name: "LTE Modem".to_string(),
            component_type: "LTE".to_string(),
            status: "warning".to_string(),
            details: "Weak signal".to_string(),
        },

        HardwareComponent {
            name: "Watchdog".to_string(),
            component_type: "System".to_string(),
            status: "ok".to_string(),
            details: "Hardware watchdog active".to_string(),
        },
    ]
}