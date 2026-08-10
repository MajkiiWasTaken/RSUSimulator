use rand::Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct GnssStatus {
    pub fix: bool,
    pub fix_type: String,

    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,

    pub satellites: u8,
    pub hdop: f32,

    pub rtk_status: String,
}

impl GnssStatus {
    pub fn new() -> Self {
        Self {
            fix: true,
            fix_type: "3D".to_string(),

            latitude: 49.5928809,
            longitude: 17.2566705,
            altitude: 220.4,

            satellites: 22,
            hdop: 0.7,

            rtk_status: "FIXED".to_string(),
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();

        let base_latitude = 49.5928809;
        let base_longitude = 17.2566705;

        self.latitude =
            base_latitude
            + rng.random_range(-0.000002..0.000002);

        self.longitude =
            base_longitude
            + rng.random_range(-0.000002..0.000002);

        self.altitude =
            220.4
            + rng.random_range(-0.2..0.2);

        self.satellites =
            rng.random_range(18..28);

        self.hdop =
            rng.random_range(0.5..0.9);
    }
}