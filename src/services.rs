/************************************************
* File: services.rs 
* Author: Michal Švrček
* 
* RSU services
*
* ver. 1.0.0.4
*************************************************/

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ServiceStatus {
    pub name: String,
    pub description: String,
    pub state: String,
    pub uptime: u64,
}

pub fn get_services() -> Vec<ServiceStatus> {
    vec![
        ServiceStatus {
            name: "rsu-core".to_string(),
            description: "Main RSU service".to_string(),
            state: "running".to_string(),
            uptime: 12500,
        },

        ServiceStatus {
            name: "gnss-service".to_string(),
            description: "GNSS receiver service".to_string(),
            state: "running".to_string(),
            uptime: 12490,
        },

        ServiceStatus {
            name: "v2x-service".to_string(),
            description: "V2X communication service".to_string(),
            state: "running".to_string(),
            uptime: 12480,
        },

        ServiceStatus {
            name: "network-manager".to_string(),
            description: "Network management".to_string(),
            state: "running".to_string(),
            uptime: 12510,
        },

        ServiceStatus {
            name: "watchdog".to_string(),
            description: "Hardware watchdog".to_string(),
            state: "running".to_string(),
            uptime: 12500,
        },
    ]
}