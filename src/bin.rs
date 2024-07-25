use mpsc::{caster, player};
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let terminal = &args[1];
    println!("{:?}", terminal);
    if terminal == "caster" {
        caster::listen();
    } else if terminal == "win" {
        player::send_udp_packet(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 52)), 3400);
    } else {
        println!("Unrecognised command line arg");
    }
}
