/************************************************
* File: services.rs 
* Author: Michal Švrček
* 
* RSU settings 
*
* ver. 1.0.0.4
*************************************************/

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RsuSettings {
    pub rsu_id: String,
    pub hostname: String,

    pub gnss_update_ms: u32,
    pub v2x_enabled: bool,

    pub log_level: String,

    pub vehicle_timeout_seconds: u32,
}

pub fn get_settings() -> RsuSettings {
    RsuSettings {
        rsu_id: "RSU-SIM-001".to_string(),
        hostname: "rsu-simulator".to_string(),

        gnss_update_ms: 200,
        v2x_enabled: true,

        log_level: "INFO".to_string(),

        vehicle_timeout_seconds: 30,
    }
}
