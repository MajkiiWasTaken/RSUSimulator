mod system;
mod gnss;

use axum::{
    routing::get,
    Json,
    Router,
};

use crate::system::SystemStatus;
use crate::gnss::GnssStatus;

async fn api_system() -> Json<SystemStatus> {
    Json(system::get_system_status())
}

async fn api_gnss() -> Json<GnssStatus> {
    Json(gnss::get_gnss_status())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/system", get(api_system))
        .route("/api/gnss", get(api_gnss));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000")
        .await
        .expect("Failed to bind simulator port");

    println!("RSU Simulator");
    println!("-----------------------------");
    println!("System: http://localhost:9000/api/system");
    println!("GNSS:   http://localhost:9000/api/gnss");
    println!("-----------------------------");

    axum::serve(listener, app)
        .await
        .expect("RSU Simulator crashed");
}