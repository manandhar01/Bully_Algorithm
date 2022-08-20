extern crate bully_algorithm;
extern crate rand;

use bully_algorithm::{create_processes, elect_coordinator, get_number_from_user};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Get number of processes from user
    let number = get_number_from_user();
    let mut processes = create_processes(number);
    let mut coordinator = (number - 1) as usize;
    let mut rng = rand::thread_rng();
    let mut random_id = rng.gen_range(0..number);

    // Choose process with highest priority as initial coordinator
    println!("Initial Coordinator is {}", coordinator);

    // Loop for simulating Bully algorithm
    loop {
        sleep(Duration::from_secs(2));
        let mut active_count = 0; // count of active processes
        for process in processes.iter() {
            if process.is_active {
                active_count += 1;
            }
        }
        // This is done to make sure at lease one of the processes is active so that the election can take place
        if active_count > 1 {
            random_id = rng.gen_range(0..number);
            if processes[random_id as usize].is_active {
                processes[random_id as usize].fail();
            }
        }
        // Run election if the coordinator fails
        if !processes[coordinator].is_active {
            while !processes[random_id as usize].is_active {
                random_id = rng.gen_range(0..number);
            }
            coordinator = elect_coordinator(&processes, random_id);
            println!("The new coordinator is: {}", coordinator);
        }
        sleep(Duration::from_secs(2));
        random_id = rng.gen_range(0..number);
        // Wake up random process that has failed
        if !processes[random_id as usize].is_active {
            processes[random_id as usize].restart();
            if processes[random_id as usize].priority > processes[coordinator].priority {
                println!(
                    "Process {} bullied process {}",
                    processes[random_id as usize].id, processes[coordinator].id
                );
                coordinator = random_id as usize;
                println!("The new coordinator is: {}", coordinator);
            }
        }
    }
}
