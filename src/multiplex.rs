use rand::{thread_rng, Rng};
use std::{
    sync::{Arc, Condvar, Mutex, RwLock},
    thread,
    time::Duration,
};

use crate::Runner;

pub struct Multiplex;

// Multiplexing in rust
// 9 threads total, 3 threads are allowed to work at any given time.
// Threads wait at condvar while predicate is true
// queue lock is taken and dec before work
// Random work is done
// queue lock is taken and inc after work
// Conditional value is then notified of value change
impl Runner for Multiplex {
    fn run() {
        let n = 9;
        let allowed = 9;
        let semaphore = Arc::new((Mutex::new(allowed), Condvar::new()));
        let mut handles = Vec::with_capacity(n);

        for i in 0..n {
            let semaphore = Arc::clone(&semaphore);
            handles.push(thread::spawn(move || {
                println!("Thread-{i}: Starting");
                let (queue, cvar) = &*semaphore;

                {
                    println!("Thread-{i}: Waiting");
                    let _ = cvar.wait_while(queue.lock().unwrap(), |value| *value == 0);
                }

                {
                    let mut v = queue.lock().unwrap();
                    *v -= 1;
                    println!("Thread-{i}: Starting work, zug zug");
                    println!("Thread-{i}: Current queue value {v}");
                }

                let mut rng = thread_rng();
                let duration = Duration::from_millis(rng.gen_range(3000..8000));

                println!("Thread-{i}: working for {}", duration.as_millis());
                thread::sleep(duration);

                println!("Thread-{i}: Done working");
                {
                    let mut v = queue.lock().unwrap();
                    *v += 1;
                    println!("Thread-{i}: incremented queue value");
                }

                println!("Thread-{i}: Notify next thread of queue change");
                cvar.notify_one();

                println!("Thread-{i}: Exiting");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
