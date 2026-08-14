/************************************************
* File: logs.rs 
* Author: Michal Švrček
* 
* RSU logs
*
* ver. 1.0.0.4
*************************************************/

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
}

pub fn get_logs() -> Vec<LogEntry> {
    vec![
        LogEntry {
            timestamp: "09:41:01".to_string(),
            level: "INFO".to_string(),
            source: "SYSTEM".to_string(),
            message: "RSU started successfully".to_string(),
        },

        LogEntry {
            timestamp: "09:41:03".to_string(),
            level: "INFO".to_string(),
            source: "GNSS".to_string(),
            message: "GNSS fix acquired".to_string(),
        },

        LogEntry {
            timestamp: "09:41:05".to_string(),
            level: "INFO".to_string(),
            source: "V2X".to_string(),
            message: "V2X modem connected".to_string(),
        },

        LogEntry {
            timestamp: "09:41:08".to_string(),
            level: "WARN".to_string(),
            source: "LTE".to_string(),
            message: "LTE signal strength low".to_string(),
        },
    ]
}