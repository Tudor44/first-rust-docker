extern crate chrono;
extern crate rand;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rand::{distributions::Alphanumeric, Rng}; // 0.8

use chrono::prelude::{Utc};

fn generate_string() -> String{
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        return s;
}

fn main() {
    // Create channels for sending and receieving
    let (five_tx, five_rx) = channel();

    // Spawn five second timer
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            five_tx.send(format!("{}: {}",Utc::now(),generate_string())).unwrap();
        }
    });

    loop {
        thread::sleep(Duration::from_millis(50));
        let _ = five_rx.try_recv().map(|reply| println!("{}", reply));
    }
}