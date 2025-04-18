use crate::settings::SERVER_PORT;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::spawn;
use std::thread::JoinHandle;
use tokio::sync::Notify;
use tower_http::services::ServeDir;

/**
 * The purpose of this module is to serve the folder where the HLS stream is saved.
 */

pub struct Server {
    handle: Option<JoinHandle<()>>,
    notify: Arc<Notify>,
}

impl Server {
    pub fn new(serve_path: PathBuf) -> Self {
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        Self {
            handle: Some(spawn(move || server_main(serve_path, notify_clone))),
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
async fn server_main(path: PathBuf, notify: Arc<Notify>) {
    let app = Router::new().nest_service("/hls", ServeDir::new(path));

    // 0.0.0.0 is the global IPv4 address
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{SERVER_PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(notify))
        .await
        .unwrap();
}

async fn shutdown_signal(notify: Arc<Notify>) {
    notify.notified().await
}
