use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
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

pub fn get_gnss_status() -> GnssStatus {
    let mut rng = rand::rng();

    // Simulovaná pevná poloha RSU + drobný GNSS šum
    let base_latitude = 49.5928809;
    let base_longitude = 17.2566705;

    GnssStatus {
        fix: true,
        fix_type: "3D".to_string(),

        latitude: base_latitude + rng.random_range(-0.000002..0.000002),
        longitude: base_longitude + rng.random_range(-0.000002..0.000002),

        altitude: 220.4 + rng.random_range(-0.2..0.2),

        satellites: rng.random_range(18..27),

        hdop: rng.random_range(0.5..0.9),

        rtk_status: "FIXED".to_string(),
    }
}