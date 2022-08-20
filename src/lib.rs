use std::io;
use std::process;

// Structure of Process
#[derive(Debug)]
pub struct Process {
    pub id: u32,
    pub priority: u32,
    pub is_active: bool,
    // next_process: &process,
}

// Process implementation
impl Process {
    // Create new instance of process
    pub fn new(id: u32, priority: u32, is_active: bool) -> Self {
        Process {
            id,
            priority,
            is_active,
        }
    }
    // Make process fail
    pub fn fail(&mut self) {
        self.is_active = false;
        println!("Process {} failed!", self.id);
    }
    // Restart failed process
    pub fn restart(&mut self) {
        self.is_active = true;
        println!("Process {} restarted!", self.id);
    }
}

// Function to get number of processes from user
pub fn get_number_from_user() -> u32 {
    println!("Enter number of processes: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read number");
    let number = match n.trim().parse::<u32>() {
        Ok(i) => i,
        Err(_) => {
            println!("cannot create '{}' processes.", n);
            process::exit(1);
        }
    };
    number
}

// Function to create number of instances of Process
pub fn create_processes(number: u32) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();
    for i in 0..number {
        processes.push(Process::new(i, i, true));
    }
    processes
}

// Function to run election for coordinator
pub fn elect_coordinator(processes: &Vec<Process>, id: u32) -> usize {
    println!("Process {} is running election", id);
    let mut new_coordinator = id as usize;
    for process in processes.iter() {
        if process.priority > processes[id as usize].priority && process.is_active {
            println!("Process {} handed election to Process {}", id, process.id);
            new_coordinator = elect_coordinator(&processes, process.id);
            break;
        }
    }
    new_coordinator
}
