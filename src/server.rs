use crate::settings::SERVER_PORT;
use axum::Router;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;
use tokio::sync::Notify;
use tower_http::services::ServeDir;

pub const DISCOVERY_MESSAGE: &[u8] = b"policast";

/**
 * The purpose of this module is to serve the folder where the HLS stream is saved.
 */

pub struct Server {
    handle: Option<JoinHandle<()>>,
    _discovery_service: AdvertisingService,
    notify: Arc<Notify>,
}

impl Server {
    pub fn new(serve_path: PathBuf) -> Self {
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        Self {
            handle: Some(spawn(move || server_main(serve_path, notify_clone))),
            _discovery_service: AdvertisingService::new(Duration::from_millis(500)),
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

struct AdvertisingService(Option<JoinHandle<()>>, Sender<()>);

impl AdvertisingService {
    fn new(period: Duration) -> Self {
        let (sender, receiver) = channel::<()>();
        Self(
            Some(spawn(move || {
                let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
                socket.set_broadcast(true).unwrap();
                loop {
                    if receiver.try_recv().is_ok() {
                        break;
                    }
                    socket
                        .send_to(
                            DISCOVERY_MESSAGE,
                            SocketAddrV4::new(Ipv4Addr::BROADCAST, SERVER_PORT),
                        )
                        .unwrap();
                    sleep(period);
                }
            })),
            sender,
        )
    }
}

impl Drop for AdvertisingService {
    fn drop(&mut self) {
        self.1.send(()).unwrap();
        self.0.take().unwrap().join().unwrap();
    }
}

//TODO: still under development
pub fn discover() {
    let mut buffer = [0u8; 1024];
    let socket = UdpSocket::bind(format!("0.0.0.0:{SERVER_PORT}")).unwrap();
    while let Ok((length, sender)) = socket.recv_from(&mut buffer) {
        match String::from_utf8_lossy(&buffer[..length]).parse::<String>() {
            Ok(message) if message.as_bytes() == DISCOVERY_MESSAGE => {
                println!("{:?}", sender);
            }
            _ => (),
        }
    }
}
