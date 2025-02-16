use sysinfo::{ System };
use prettytable::{Table, row};
use chrono::Local;
use std::thread;
use std::time::Duration;
use console::Term;

fn create_table_header_system(table: &mut Table) {
    // Create a table with headers for all system resource fields
    table.add_row(row!["Timestamp", "CPU Usage", "RAM Usage (Free/Used/Total)", "GPU Usage"]);
}

fn create_table_header_process(table: &mut Table) {
    // Create a table with headers for a single process
    table.add_row(row!["Timestamp", "Process name", "CPU Usage", "RAM Usage"]);
}

fn log_system_stats(system: &System, table: &mut Table) {
    // Get current timestamp
    let timestamp = Local::now().to_rfc3339();

    // CPU Usage (Overall and per-core)
    let cpu_overall = system.global_cpu_usage();
    let mut cpu_per_core = String::new();
    for cpu in system.cpus() {
        cpu_per_core.push_str(&format!("{:.2}%, ", cpu.cpu_usage()));
    }
    cpu_per_core.pop(); // Remove the last comma

    // RAM Usage (Free, Used, Total)
    let ram_free = (system.free_memory() as f64) / 1024.0 / 1024.0 / 1024.0; // Convert to GB
    let ram_used = (system.used_memory() as f64) / 1024.0 / 1024.0 / 1024.0; // Convert to GB
    let ram_total = (system.total_memory() as f64) / 1024.0 / 1024.0 / 1024.0; // Convert to GB

    // GPU Usage (if supported, placeholder for now)
    let gpu_usage = "N/A"; // Placeholder for GPU logic

    // Create a row of system stats to log
    let row = row![
        timestamp,
        format!("{:.2}%", cpu_overall),
        format!("{:.2} GB / {:.2} GB / {:.2} GB", ram_used, ram_total, ram_free),
        gpu_usage
    ];

    // Clear terminal (and scrollback buffer) before printing new data
    let term = Term::stdout();
    term.clear_screen().unwrap(); // Clears the screen

    // Clear scrollback buffer for Unix-like systems (Linux/macOS)
    print!("\x1b[3J");

    // Print the row in table format
    table.add_row(row);
    table.printstd();
}

fn log_pid_stats(system: &System, table: &mut Table, pid: u32) {
    // filter through processes and find the wanted pid
    let pid_optional = system.processes().iter()
        .find(|(pp, _)| {
            let u32_pid = pp.as_u32();
            return u32_pid == pid;
        }).map(|(process_pid, _)| process_pid);

    // if no pid is found just throw an error
    // If PID is not found, log the error and return early
    if pid_optional.is_none() {
        eprintln!("Error: PID {} not found", pid);
        std::process::exit(1);
    }

    let pid_found = pid_optional.unwrap();

    if let Some(process) = system.process(*pid_found) {
        let timestamp = Local::now().to_rfc3339();
        let process_name = process.name().to_str().unwrap_or_else(|| "<unknown>");
        let cpu_usage = process.cpu_usage();
        let ram_usage = (process.memory() as f64) / 1024.0 / 1024.0 / 1024.0;

        let row = row![
            timestamp,
            process_name,
            format!("{:.2}%", cpu_usage),
            format!("{:.2} GB", ram_usage),
        ];

        // Clear terminal (and scrollback buffer) before printing new data
        let term = Term::stdout();
        term.clear_screen().unwrap(); // Clears the screen

        // Clear scrollback buffer for Unix-like systems (Linux/macOS)
        print!("\x1b[3J");

        // Print the row in table format
        table.add_row(row);
        table.printstd();
    }

}

fn start_system_monitor(system: &mut System, table: &mut Table) {
    create_table_header_system(table);

    loop {
        system.refresh_all();

        // Log system stats
        log_system_stats(&system, table);

        // Adjust the refresh rate (interval) here as needed
        thread::sleep(Duration::from_secs(2));
    }
}

fn start_process_monitor(system: &mut System, table: &mut Table, pid: u32) {
    create_table_header_process(table);

    loop {
        system.refresh_all();

        // log process stats
        log_pid_stats(system, table, pid);

        // sleep for two seconds
        thread::sleep(Duration::from_secs(2));
    }
}

fn handle_pid_argument(system: &mut System, table: &mut Table, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Missing PID. Please provide a valid PID.");
        std::process::exit(1);
    }

    let pid_arg = &args[2];
    match pid_arg.parse::<u32>() {
        Ok(pid) => {
            start_process_monitor(system, table, pid);
        },
        Err(_) => {
            eprintln!("Invalid PID: {}. Please provide a valid integer.", pid_arg);
            std::process::exit(1);
        }
    }
}

fn main() {
    let mut system = System::new_all();
    let mut table = Table::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            // Handle PID case
            "-pid" => handle_pid_argument(&mut system, &mut table, &args),  // Call the function directly
            _ => {
                eprintln!("Unknown command");
                std::process::exit(1);
            },
        }
    } else {
        start_system_monitor(&mut system, &mut table);
    }
}
