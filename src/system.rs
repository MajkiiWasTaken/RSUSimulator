use rand::Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct SystemStatus {
    pub rsu_id: String,
    pub hostname: String,
    pub uptime: u64,

    pub cpu_usage: f32,
    pub cpu_temperature: f32,

    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
}

impl SystemStatus {
    pub fn new() -> Self {
        Self {
            rsu_id: "RSU-SIM-001".to_string(),
            hostname: "rsu-simulator".to_string(),

            uptime: 0,

            cpu_usage: 10.0,
            cpu_temperature: 45.0,

            ram_used_mb: 900,
            ram_total_mb: 4096,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();

        self.uptime += 1;

        self.cpu_usage =
            rng.random_range(5.0..35.0);

        self.cpu_temperature =
            rng.random_range(40.0..60.0);

        self.ram_used_mb =
            rng.random_range(700..1500);
    }
}