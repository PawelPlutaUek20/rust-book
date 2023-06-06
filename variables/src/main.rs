extern crate job_scheduler;

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("0 3 * * * *".parse().unwrap(), || {
        println!("I get executed every 1 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}