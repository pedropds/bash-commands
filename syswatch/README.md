# Syswatch
Syswatch is a system monitoring tool that tracks CPU, RAM, and other system metrics, with optional PID-based process monitoring. It provides a real-time display of system statistics in a tabular format and can be run either globally for the system or for a specific process identified by its PID.

## Features

- Monitor **CPU usage** (Overall and per-core)
- Monitor **RAM usage** (Free, Used, and Total)
- (Placeholder) **GPU usage** (when implemented)
- Monitor **Disk I/O** (Read/Write speeds)
- Option to track a specific process using its **PID**
- Automatically clears the terminal for cleaner output
- Table format for easy readability

## Prerequisites
- **Rust** must be installed on your system to build Syswatch. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).
- A terminal or command-line interface (CLI) on your operating system (Bash on Linux/macOS).

## Installation
1. Clone the repository and change to the **syswatch** directory:
   ```bash
   git clone https://github.com/pedropds/bash-commands.git
   cd bash-commands/syswatch
   ```

2. Build the application
   ```bash
    cargo build --release
   ```
   
3. Install the binary
   After building, the binary file will be located in the `target/release` directory. You can either:
   - **Move the binary to a directory in your PATH** (recommended):
     ```bash
     sudo mv target/release/syswatch /usr/local/bin/
     ```
   - Alternatively, you can run it directly from the `target/release` directory:  
     ```bash
     ./target/release/syswatch
     ```
Use Cargo (Rust’s package manager and build tool) to build Syswatch:

## Usage
### General System Monitoring

To start monitoring the system's CPU, RAM, and other metrics globally, just run:
```bash
  syswatch
```

This will display the system resource usage in a table format, updating every 2 seconds.

### Monitor Specific Process by PID
To monitor a specific process by its PID, use the `-pid` option followed by the PID. For example:

```bash
  syswatch -pid 1234
```
Where `1234` is the PID of the process you want to monitor. Syswatch will display the metrics for the specified process.

### Error Handling
- If a PID is provided but no process is found with that PID, Syswatch will print an error message like this:
  ```bash
  Error: PID 1234 not found
  ```
  
- If an invalid PID is provided: 
  ```bash
  Syswatch will notify you:
  ```

## Contributing
If you’d like to contribute to Syswatch, feel free to fork the repository, create a branch, and submit a pull request. All contributions are welcome!
