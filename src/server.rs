use axum::routing::get;
use axum::Router;
use egui::mutex::RwLock;
use std::sync::Arc;
use std::thread::spawn;
use tower_http::services::ServeDir;

use crate::settings::Settings;

/**
 * The purpose of this module is to serve the folder where the HLS stream is saved.
 */

pub struct Server {
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Server {
    pub fn new(settings: Arc<RwLock<Settings>>) -> Self {
        Self {
            handle: Some(spawn(move || server_main(settings))),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}

#[tokio::main]
async fn server_main(settings: Arc<RwLock<Settings>>) {
    let path = settings.read().get_save_dir();
    let app = Router::new().nest_service("/hls", ServeDir::new(path));

    // 0.0.0.0 is the global IPv4 address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//look for stream at <device IP>:3000/hls/output.m3u8
