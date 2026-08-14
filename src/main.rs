mod state;
mod system;
mod gnss;
mod v2x;
mod network;
mod services;
mod hardware;
mod logs;
mod settings;

use network::NetworkStatus;

use services::ServiceStatus;
use hardware::HardwareComponent;
use logs::LogEntry;
use settings::RsuSettings;

use std::time::Duration;

use axum::{
    extract::{
        ws::{
            Message,
            WebSocket,
            WebSocketUpgrade,
        },
        State,
    },
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};

use tower_http::cors::CorsLayer;

use state::AppState;

use system::SystemStatus;
use gnss::GnssStatus;
use v2x::{
    V2xMessage,
    V2xStatus,
};

use crate::v2x::V2xSimulator;

async fn api_system(
    State(state): State<AppState>,
) -> Json<SystemStatus> {

    let system =
        state.system
            .read()
            .await
            .clone();

    Json(system)
}

async fn api_gnss(
    State(state): State<AppState>,
) -> Json<GnssStatus> {

    let gnss =
        state.gnss
            .read()
            .await
            .clone();

    Json(gnss)
}

async fn api_v2x(
    State(state): State<AppState>,
) -> Json<V2xStatus> {

    let v2x =
        state.v2x
            .read()
            .await
            .clone();

    Json(v2x)
}


async fn api_network(
    State(state): State<AppState>,
) -> Json<NetworkStatus> {

    let network =
        state.network
            .read()
            .await
            .clone();

    Json(network)
}


async fn api_services()
    -> Json<Vec<ServiceStatus>>
{
    Json(
        services::get_services()
    )
}


async fn api_hardware()
    -> Json<Vec<HardwareComponent>>
{
    Json(
        hardware::get_hardware()
    )
}


async fn api_logs()
    -> Json<Vec<LogEntry>>
{
    Json(
        logs::get_logs()
    )
}


async fn api_settings()
    -> Json<RsuSettings>
{
    Json(
        settings::get_settings()
    )
}

async fn system_task(
    state: AppState,
) {
    loop {
        {
            let mut system =
                state.system
                    .write()
                    .await;

            system.update();
        }

        tokio::time::sleep(
            Duration::from_secs(1)
        ).await;
    }
}

async fn gnss_task(
    state: AppState,
) {
    loop {
        {
            let mut gnss =
                state.gnss
                    .write()
                    .await;

            gnss.update();
        }

        tokio::time::sleep(
            Duration::from_millis(200)
        ).await;
    }
}

async fn v2x_task(
    state: AppState,
) {
    let mut simulator =
        V2xSimulator::new();


    loop {

        // 500 ms update
        simulator.update_vehicles(
            0.5
        );


        // ====================================================
        // CAM
        // ====================================================

        let cam_messages =
            simulator
                .generate_cam_messages();


        let cam_count =
            cam_messages.len();


        for message in
            cam_messages
        {

            {
                let mut v2x =
                    state.v2x
                        .write()
                        .await;


                v2x.rx_packets += 1;

                v2x.cam_count += 1;
            }


            let _ =
                state.v2x_tx
                    .send(
                        message
                    );
        }


        // ====================================================
        // DENM / SRV
        // ====================================================

        if let Some(message) =
            simulator
                .random_special_message()
        {

            {
                let mut v2x =
                    state.v2x
                        .write()
                        .await;


                if message.direction
                    ==
                    "RX"
                {

                    v2x.rx_packets += 1;
                }
                else {

                    v2x.tx_packets += 1;
                }


                match message
                    .message_type
                    .as_str()
                {

                    "DENM" => {
                        v2x.denm_count += 1;
                    }

                    "SRV" => {
                        v2x.srv_count += 1;
                    }

                    _ => {}
                }
            }


            let _ =
                state.v2x_tx
                    .send(
                        message
                    );
        }


        // ====================================================
        // RATE
        // ====================================================

        {
            let mut v2x =
                state.v2x
                    .write()
                    .await;


            // 6 CAM každých 0.5 s
            // => cca 12 RX/s

            v2x.rx_per_second =
                cam_count as f32
                *
                2.0;
        }


        tokio::time::sleep(
            Duration::from_millis(
                500
            )
        )
        .await;
    }
}

async fn network_task(
    state: AppState,
) {
    loop {
        {
            let mut network =
                state.network
                    .write()
                    .await;

            network.update();
        }

        tokio::time::sleep(
            Duration::from_secs(1)
        )
        .await;
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(
        move |socket| {
            handle_socket(
                socket,
                state,
            )
        }
    )
}

async fn handle_socket(
    mut socket: WebSocket,
    state: AppState,
) {
    println!(
        "WebSocket client connected"
    );

    let mut receiver =
        state.v2x_tx
            .subscribe();

    loop {
        match receiver.recv().await {
            Ok(v2x_message) => {

                let json =
                    match serde_json::to_string(
                        &v2x_message
                    ) {
                        Ok(json) => json,

                        Err(error) => {
                            eprintln!(
                                "JSON serialization error: {}",
                                error
                            );

                            continue;
                        }
                    };

                if socket
                    .send(
                        Message::Text(
                            json.into()
                        )
                    )
                    .await
                    .is_err()
                {
                    break;
                }
            }

            Err(
                tokio::sync::broadcast::error::RecvError::Lagged(
                    skipped
                )
            ) => {
                println!(
                    "WebSocket client lagged, skipped {} messages",
                    skipped
                );
            }

            Err(_) => {
                break;
            }
        }
    }

    println!(
        "WebSocket client disconnected"
    );
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    
    let state =
        AppState::new();

    tokio::spawn(
        system_task(
            state.clone()
        )
    );

    tokio::spawn(
        gnss_task(
            state.clone()
        )
    );

    tokio::spawn(
        v2x_task(
            state.clone()
        )
    );

    tokio::spawn(
        network_task(
            state.clone()
        )
    );

    let app =
        Router::new()
            .route("/api/system", get(api_system))
            .route("/api/gnss", get(api_gnss))
            .route("/api/v2x", get(api_v2x))
            .route("/ws", get(ws_handler))
            .route(
                "/api/network",
                get(api_network)
            )

            .route(
                "/api/services",
                get(api_services)
            )

            .route(
                "/api/hardware",
                get(api_hardware)
            )

            .route(
                "/api/logs",
                get(api_logs)
            )

            .route(
                "/api/settings",
                get(api_settings)
            )
            .layer(CorsLayer::permissive())
            .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(
            "0.0.0.0:9000"
        )
        .await
        .expect(
            "Failed to bind simulator port"
        );

    println!();
    println!("RSU Simulator");
    println!("--------------------------------");
    println!("System: http://localhost:9000/api/system");
    println!("GNSS:   http://localhost:9000/api/gnss");
    println!("V2X:    http://localhost:9000/api/v2x");
    println!("--------------------------------");
    println!();

    axum::serve(
        listener,
        app
    )
    .await
    .expect(
        "RSU Simulator crashed"
    );
}