use mpsc::{caster, player};
use std::env;

pub fn main() {
    os_hello();

    let args: Vec<String> = env::args().collect();
    let terminal = &args[1];
    println!("{:?}", terminal);
    if terminal == "caster" {
        caster::setup();
    } else if terminal == "player" {
        player::setup();
    } else {
        println!("Unrecognised command line arg");
    }
}

fn os_hello() {
    if cfg!(target_os = "macos") {
        println!("Think different!");
    } else if cfg!(target_os = "windows") {
        println!("We like blue screens");
    } else if cfg!(target_os = "linux") {
        println!("We don't like the government!");
    } else {
        println!("We are not supported!");
    }
}
