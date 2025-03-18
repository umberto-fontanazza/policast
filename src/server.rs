use axum::Router;
use std::path::PathBuf;
use std::thread::spawn;
use tower_http::services::ServeDir;

/**
 * The purpose of this module is to serve the folder where the HLS stream is saved.
 */

pub struct Server {
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            handle: Some(spawn(|| server_main())),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}

#[tokio::main]
async fn server_main() {
    let path = ["tmp", "test"].iter().collect::<PathBuf>(); //TODO: use the capture save path from the Settings module
    let app = Router::new().nest_service("/hls", ServeDir::new(path));

    // 0.0.0.0 is the global IPv4 address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
