mod state;
mod system;
mod gnss;
mod v2x;

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

use rand::Rng;

use tower_http::cors::CorsLayer;

use state::AppState;

use system::SystemStatus;
use gnss::GnssStatus;
use v2x::{
    V2xMessage,
    V2xStatus,
};

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
    loop {
        let message =
            V2xMessage::random();

        {
            let mut v2x =
                state.v2x
                    .write()
                    .await;

            match message.direction.as_str() {
                "RX" => {
                    v2x.rx_packets += 1;
                }

                "TX" => {
                    v2x.tx_packets += 1;
                }

                _ => {}
            }

            match message.message_type.as_str() {
                "CAM" => {
                    v2x.cam_count += 1;
                }

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
                .send(message);

        let delay = {
            let mut rng =
                rand::rng();

            rng.random_range(
                100..800
            )
        };

        tokio::time::sleep(
            Duration::from_millis(delay)
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

    let app =
        Router::new()
            .route("/api/system", get(api_system))
            .route("/api/gnss", get(api_gnss))
            .route("/api/v2x", get(api_v2x))
            .route("/ws", get(ws_handler))
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