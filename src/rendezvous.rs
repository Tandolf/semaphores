use rand::{thread_rng, Rng};
use std::{sync::mpsc::sync_channel, thread, time::Duration};

use crate::Runner;

// Rendezvous pattern
// Using a channel with zero allocations to wait and signal between threads.
// Whomever is finished first will signal and wait for the other,
// Important to signal first, and recv second.
// Other way around will work, but might be extra overhead.
pub struct Rendezvous;

impl Runner for Rendezvous {
    fn run() {
        let (atx, arx) = sync_channel(0);
        let (btx, brx) = sync_channel(0);

        println!("Main: Running thread A");
        let a = thread::spawn(move || {
            println!("A: Starting work zug zug");
            let mut rng = thread_rng();
            let duration = Duration::from_millis(rng.gen_range(1000..8000));
            println!("A: working for {}", duration.as_millis());
            thread::sleep(duration);
            println!("A: Work complete");
            println!("A: Signaling B");
            let _ = brx.try_recv();
            println!("A: Waiting for B");
            let _ = atx.send(());
            println!("A: Thread complete");
        });

        println!("Main: Running thread B");
        let b = thread::spawn(move || {
            println!("B: Starting work zug zug");
            let mut rng = thread_rng();
            let duration = Duration::from_millis(rng.gen_range(1000..8000));
            println!("B: working for {}", duration.as_millis());
            thread::sleep(duration);
            println!("B: Work complete");
            println!("B: Signaling A");
            let _ = arx.try_recv();
            println!("B: Waiting for A");
            let _ = btx.send(());
            println!("B: Thread complete");
        });

        let _ = a.join();
        let _ = b.join();
        println!("Main: Both threads joined");
    }
}
