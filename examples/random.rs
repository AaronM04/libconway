extern crate conway;
extern crate rand;

use rand::Rng;
use std::{thread, time};
use conway::*;

fn main() {
    let mut uni = Universe::new(128,32).unwrap();
    let step_time = time::Duration::from_millis(30);

    let mut rng = rand::thread_rng();
    loop {
        println!("\x1b[H\x1b[2J{}", uni);
        println!("Gen: {}", uni.latest_gen());
        let rand_word: u64 = rng.gen::<u8>() as u64;
        uni.set_word(0,15, (rand_word >> 3) & 7); // RANDOM!!!
        uni.set_word(0,16,  rand_word       & 7); // RANDOM!!!
        uni.next();
        thread::sleep(step_time);
    }
}