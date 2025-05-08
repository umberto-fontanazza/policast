use crate::settings::SERVER_PORT;
use axum::Router;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;
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

enum DiscoveryCommand {
    Stop,
    SendIPs,
}

pub struct DiscoveryService {
    discovery_thread: Option<JoinHandle<()>>,
    req_sender: Sender<DiscoveryCommand>,
    ips_receiver: Receiver<Vec<Ipv4Addr>>,
}

impl DiscoveryService {
    pub fn new() -> Self {
        let (req_sender, req_receiver) = channel::<DiscoveryCommand>();
        let (ips_sender, ips_receiver) = channel::<Vec<Ipv4Addr>>();
        let discovery_thread = Some(spawn(move || {
            let mut casters = HashMap::<Ipv4Addr, Instant>::new();
            loop {
                match req_receiver.try_recv() {
                    Ok(DiscoveryCommand::SendIPs) => {
                        let _ = ips_sender.send(get_caster_ips(&mut casters));
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => (),
                    Ok(DiscoveryCommand::Stop) => {
                        return;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        return;
                    }
                };
                discover(&mut casters);
            }
        }));
        Self {
            discovery_thread,
            req_sender,
            ips_receiver,
        }
    }

    pub fn get_casters(&self) -> Vec<Ipv4Addr> {
        self.req_sender.send(DiscoveryCommand::SendIPs).unwrap();
        self.ips_receiver.recv().unwrap()
    }
}

impl Drop for DiscoveryService {
    fn drop(&mut self) {
        self.req_sender.send(DiscoveryCommand::Stop).unwrap();
        self.discovery_thread.take().unwrap().join().unwrap();
    }
}

pub fn discover(casters: &mut HashMap<Ipv4Addr, Instant>) {
    let mut buffer = [0u8; 1024];
    let socket = UdpSocket::bind(format!("0.0.0.0:{SERVER_PORT}")).unwrap();
    socket.set_nonblocking(true).unwrap();
    while let Ok((length, sender)) = socket.recv_from(&mut buffer) {
        match String::from_utf8_lossy(&buffer[..length]).parse::<String>() {
            Ok(message) if message.as_bytes() == DISCOVERY_MESSAGE => {
                let addr = match sender {
                    std::net::SocketAddr::V4(socket_addr_v4) => socket_addr_v4,
                    _ => panic!("Unsupported ipv6"),
                };
                casters.insert(addr.ip().to_owned(), Instant::now());
            }
            _ => (),
        }
    }
}

const TTL: Duration = Duration::from_secs(5);

fn get_caster_ips(casters: &mut HashMap<Ipv4Addr, Instant>) -> Vec<Ipv4Addr> {
    let now = Instant::now();
    *casters = casters
        .into_iter()
        .filter(|(_, t_created)| now - **t_created < TTL)
        .map(|(k, v)| (*k, *v))
        .collect::<HashMap<Ipv4Addr, Instant>>();
    casters
        .iter()
        .map(|(k, _)| k.clone())
        .collect::<Vec<Ipv4Addr>>()
}
