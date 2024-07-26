use mpsc::{caster, player};
use std::env;

pub fn main() {
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
