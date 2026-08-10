use rand::Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct V2xStatus {
    pub modem_connected: bool,

    pub rx_packets: u64,
    pub tx_packets: u64,

    pub cam_count: u64,
    pub denm_count: u64,
    pub srv_count: u64,

    pub errors: u64,

    pub rx_per_second: f32,
    pub tx_per_second: f32,
}

#[derive(Clone, Serialize)]
pub struct V2xMessage {
    pub direction: String,
    pub message_type: String,

    pub station_id: u32,

    pub latitude: f64,
    pub longitude: f64,

    pub speed_kmh: f32,
    pub heading: f32,

    pub raw: String,
}

impl V2xStatus {
    pub fn new() -> Self {
        Self {
            modem_connected: true,

            rx_packets: 0,
            tx_packets: 0,

            cam_count: 0,
            denm_count: 0,
            srv_count: 0,

            errors: 0,

            rx_per_second: 0.0,
            tx_per_second: 0.0,
        }
    }
}

impl V2xMessage {
    pub fn random() -> Self {
        let mut rng = rand::rng();

        let message_type = match rng.random_range(0..100) {
            0..=84 => "CAM",
            85..=94 => "DENM",
            _ => "SRV",
        };

        let direction = if message_type == "SRV" {
            "TX"
        } else {
            "RX"
        };

        Self {
            direction: direction.to_string(),
            message_type: message_type.to_string(),

            station_id:
                rng.random_range(1000..99999),

            latitude:
                49.5928809
                + rng.random_range(
                    -0.003..0.003
                ),

            longitude:
                17.2566705
                + rng.random_range(
                    -0.003..0.003
                ),

            speed_kmh:
                rng.random_range(0.0..90.0),

            heading:
                rng.random_range(0.0..360.0),

            raw:
                generate_fake_hex(),
        }
    }
}

fn generate_fake_hex() -> String {
    let mut rng = rand::rng();

    let length =
        rng.random_range(20..60);

    let mut result =
        String::new();

    for _ in 0..length {
        let value: u8 =
            rng.random();

        result.push_str(
            &format!("{:02X}", value)
        );
    }

    result
}