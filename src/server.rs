use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
/**
 * Here goes the implementation of the HLS server
 */
use std::thread::spawn;

pub struct Server {
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Server {
    pub fn new() -> Self {
        println!("Creating server");
        let handle = spawn(move || {
            let a = server_main();
            ()
        });
        Self {
            handle: Some(handle),
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
    // tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    println!("Request, received");
    "Hello, World!"
}
