use rand::{thread_rng, Rng};
use std::{
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};

use crate::Runner;

pub struct BarrierRunner;

impl Runner for BarrierRunner {
    fn run() {
        let no_threads = 10;
        let mut handles = Vec::with_capacity(no_threads);
        let barrier = Arc::new(Barrier::new(no_threads));

        for i in 0..no_threads {
            let c = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                println!("Thread-{i}: Started");
                let mut rng = thread_rng();
                let duration = Duration::from_millis(rng.gen_range(3000..8000));
                println!("Thread-{i}: working for {}", duration.as_millis());
                thread::sleep(duration);
                println!("Thread-{i}: Waiting for others");
                c.wait();
                println!("Thread-{i}: Exiting");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
