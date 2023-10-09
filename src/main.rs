use rendezvous::Rendezvous;

use crate::{barrier::BarrierRunner, multiplex::Multiplex};

mod barrier;
mod multiplex;
mod rendezvous;

pub trait Runner {
    fn run();
}

fn main() {
    println!("Main: Rendezvous is starting");
    Rendezvous::run();
    println!("Main: Rendezvous has stopped");

    println!("Main: Barrier is starting");
    BarrierRunner::run();
    println!("Main: Barrier has stopped");

    println!("Main: Multiplex is starting");
    Multiplex::run();
    println!("Main: Multiplex has stopped");
}
