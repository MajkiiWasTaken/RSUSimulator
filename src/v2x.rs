use rand::Rng;
use serde::Serialize;

const RSU_LATITUDE: f64 = 49.5928809;
const RSU_LONGITUDE: f64 = 17.2566705;

const VEHICLE_COUNT: usize = 6;

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


pub struct SimVehicle {
    pub station_id: u32,

    pub latitude: f64,
    pub longitude: f64,

    pub speed_kmh: f32,
    pub heading: f32,
}


pub struct V2xSimulator {
    pub vehicles: Vec<SimVehicle>,
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


impl V2xSimulator {
    pub fn new() -> Self {
        let mut vehicles =
            Vec::new();

        let mut rng =
            rand::rng();


        for i in 0..VEHICLE_COUNT {

            let angle =
                (
                    i as f64
                    /
                    VEHICLE_COUNT as f64
                )
                *
                std::f64::consts::TAU;


            let distance =
                rng.random_range(
                    100.0..350.0
                );


            let latitude =
                RSU_LATITUDE
                +
                (
                    distance
                    *
                    angle.cos()
                )
                /
                111_320.0;


            let longitude =
                RSU_LONGITUDE
                +
                (
                    distance
                    *
                    angle.sin()
                )
                /
                (
                    111_320.0
                    *
                    RSU_LATITUDE
                        .to_radians()
                        .cos()
                );


            vehicles.push(
                SimVehicle {
                    station_id:
                        10_001
                        +
                        i as u32,

                    latitude,

                    longitude,

                    speed_kmh:
                        rng.random_range(
                            25.0..70.0
                        ),

                    heading:
                        rng.random_range(
                            0.0..360.0
                        ),
                }
            );
        }


        Self {
            vehicles,
        }
    }


    pub fn update_vehicles(
        &mut self,
        delta_seconds: f32,
    ) {
        let mut rng =
            rand::rng();


        for vehicle in
            &mut self.vehicles
        {

            // lehká náhodná změna směru
            vehicle.heading +=
                rng.random_range(
                    -4.0..4.0
                );


            if vehicle.heading < 0.0 {
                vehicle.heading += 360.0;
            }


            if vehicle.heading >= 360.0 {
                vehicle.heading -= 360.0;
            }


            // lehká změna rychlosti
            vehicle.speed_kmh +=
                rng.random_range(
                    -2.0..2.0
                );


            vehicle.speed_kmh =
                vehicle
                    .speed_kmh
                    .clamp(
                        15.0,
                        80.0
                    );


            let speed_ms =
                vehicle.speed_kmh
                /
                3.6;


            let distance =
                speed_ms
                *
                delta_seconds;


            let heading =
                (
                    vehicle.heading
                    as f64
                )
                .to_radians();


            let north =
                heading.cos()
                *
                distance as f64;


            let east =
                heading.sin()
                *
                distance as f64;


            vehicle.latitude +=
                north
                /
                111_320.0;


            vehicle.longitude +=
                east
                /
                (
                    111_320.0
                    *
                    vehicle
                        .latitude
                        .to_radians()
                        .cos()
                );


            keep_vehicle_near_rsu(
                vehicle
            );
        }
    }


    pub fn generate_cam_messages(
        &self,
    ) -> Vec<V2xMessage> {

        self.vehicles
            .iter()
            .map(
                |vehicle| {
                    V2xMessage {
                        direction:
                            "RX".to_string(),

                        message_type:
                            "CAM".to_string(),

                        station_id:
                            vehicle.station_id,

                        latitude:
                            vehicle.latitude,

                        longitude:
                            vehicle.longitude,

                        speed_kmh:
                            vehicle.speed_kmh,

                        heading:
                            vehicle.heading,

                        raw:
                            generate_fake_hex(),
                    }
                }
            )
            .collect()
    }


    pub fn random_special_message(
        &self,
    ) -> Option<V2xMessage> {

        let mut rng =
            rand::rng();


        let chance =
            rng.random_range(
                0..1000
            );


        // cca 1 %
        if chance < 10 {

            let vehicle =
                &self.vehicles[
                    rng.random_range(
                        0..self.vehicles.len()
                    )
                ];


            return Some(
                V2xMessage {
                    direction:
                        "RX".to_string(),

                    message_type:
                        "DENM".to_string(),

                    station_id:
                        vehicle.station_id,

                    latitude:
                        vehicle.latitude,

                    longitude:
                        vehicle.longitude,

                    speed_kmh:
                        vehicle.speed_kmh,

                    heading:
                        vehicle.heading,

                    raw:
                        generate_fake_hex(),
                }
            );
        }


        // cca 2 %
        if chance < 30 {

            return Some(
                V2xMessage {
                    direction:
                        "TX".to_string(),

                    message_type:
                        "SRV".to_string(),

                    station_id:
                        1,

                    latitude:
                        RSU_LATITUDE,

                    longitude:
                        RSU_LONGITUDE,

                    speed_kmh:
                        0.0,

                    heading:
                        0.0,

                    raw:
                        generate_fake_hex(),
                }
            );
        }


        None
    }
}


fn keep_vehicle_near_rsu(
    vehicle: &mut SimVehicle,
) {

    let lat_distance =
        (
            vehicle.latitude
            -
            RSU_LATITUDE
        )
        *
        111_320.0;


    let lon_distance =
        (
            vehicle.longitude
            -
            RSU_LONGITUDE
        )
        *
        111_320.0
        *
        RSU_LATITUDE
            .to_radians()
            .cos();


    let distance =
        (
            lat_distance
            *
            lat_distance
            +
            lon_distance
            *
            lon_distance
        )
        .sqrt();


    // když auto ujede moc daleko,
    // otočíme ho zpět směrem k RSU

    if distance > 500.0 {

        let delta_lat =
            RSU_LATITUDE
            -
            vehicle.latitude;


        let delta_lon =
            RSU_LONGITUDE
            -
            vehicle.longitude;


        let heading =
            delta_lon
                .atan2(
                    delta_lat
                )
                .to_degrees();


        vehicle.heading =
            heading as f32;


        if vehicle.heading < 0.0 {
            vehicle.heading += 360.0;
        }
    }
}


fn generate_fake_hex() -> String {
    let mut rng =
        rand::rng();


    let length =
        rng.random_range(
            30..70
        );


    let mut result =
        String::new();


    for _ in 0..length {

        let value: u8 =
            rng.random();


        result.push_str(
            &format!(
                "{:02X}",
                value
            )
        );
    }


    result
}