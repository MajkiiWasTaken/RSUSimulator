use std::sync::Arc;

use tokio::sync::{
    broadcast,
    RwLock,
};

use crate::{
    gnss::GnssStatus,
    system::SystemStatus,
    v2x::{
        V2xMessage,
        V2xStatus,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub system: Arc<RwLock<SystemStatus>>,
    pub gnss: Arc<RwLock<GnssStatus>>,
    pub v2x: Arc<RwLock<V2xStatus>>,

    pub v2x_tx: broadcast::Sender<V2xMessage>,
}

impl AppState {
    pub fn new() -> Self {
        let (v2x_tx, _) =
            broadcast::channel(256);

        Self {
            system: Arc::new(
                RwLock::new(
                    SystemStatus::new()
                )
            ),

            gnss: Arc::new(
                RwLock::new(
                    GnssStatus::new()
                )
            ),

            v2x: Arc::new(
                RwLock::new(
                    V2xStatus::new()
                )
            ),

            v2x_tx,
        }
    }
}