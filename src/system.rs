use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemStatus {
    pub rsu_id: String,
    pub hostname: String,
    pub uptime: u64,
    pub cpu_usage: f32,
    pub cpu_temperature: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
}

pub fn get_system_status() -> SystemStatus {
    let mut rng = rand::rng();

    SystemStatus {
        rsu_id: "RSU-SIM-001".to_string(),
        hostname: "rsu-simulator".to_string(),

        uptime: 128_450,

        cpu_usage: rng.random_range(5.0..35.0),
        cpu_temperature: rng.random_range(40.0..58.0),

        ram_used_mb: rng.random_range(700..1400),
        ram_total_mb: 4096,
    }
}