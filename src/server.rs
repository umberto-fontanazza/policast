use axum::Router;
use egui::mutex::RwLock;
use std::sync::Arc;
use std::thread::spawn;
use tokio::sync::Notify;
use tower_http::services::ServeDir;

use crate::settings::Settings;

/**
 * The purpose of this module is to serve the folder where the HLS stream is saved.
 */

pub struct Server {
    handle: Option<std::thread::JoinHandle<()>>,
    notify: Arc<Notify>,
}

impl Server {
    pub fn new(settings: Arc<RwLock<Settings>>) -> Self {
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        Self {
            handle: Some(spawn(move || server_main(settings, notify_clone))),
            notify,
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.notify.notify_one();
        self.handle.take().unwrap().join().unwrap();
    }
}

#[tokio::main]
async fn server_main(settings: Arc<RwLock<Settings>>, notify: Arc<Notify>) {
    let path = settings.read().get_save_dir();
    let app = Router::new().nest_service("/hls", ServeDir::new(path));

    // 0.0.0.0 is the global IPv4 address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(notify))
        .await
        .unwrap();
}

async fn shutdown_signal(notify: Arc<Notify>) {
    notify.notified().await
}
