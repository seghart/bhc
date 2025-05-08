# batch host command Project

## Overview
BHC is a Rust-based application designed to facilitate SSH connections for executing commands and uploading files. It provides a command-line interface (CLI) for users to interact with remote servers securely.

## Features
- Convert user and command configurations from files.
- Execute commands on remote servers via SSH.
- Upload files to remote servers.
- Log outputs and errors to separate files.

## Project Structure
```
bhc
├── src
│   ├── main.rs          # Main entry point of the application
│   ├── cli              # Command-line interface implementation
│   │   └── mod.rs
│   ├── readfiles        # Module for reading configuration files
│   │   ├── user.rs      # User configuration struct
│   │   ├── command.rs   # Command configuration struct
│   │   └── mod.rs
│   ├── ssh              # SSH connection logic
│   │   ├── mode_conn.rs # Functions for executing commands and uploading files
│   │   └── mod.rs
├── Cargo.toml           # Cargo configuration file
├── Cargo.lock           # Dependency lock file
└── README.md            # Project documentation
```

## Setup Instructions
1. Ensure you have Rust and Cargo installed on your machine.
2. Clone the repository:
   ```bash
   git clone <repository-url>
   cd bhc
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the application:
   ```bash
   cargo run
   ```

## Usage
1. **Command Mode**:
   ```bash
   ./bhc Command <config_file_path> <command_file_path>
   ```
   Example:
   ```bash
   ./bhc Command configs.txt commands.txt
   ```

2. **Upload Mode**:
   ```bash
   ./bhc Upload <config_file_path> <local_file_path> <remote_file_path>
   ```
   Example:
   ```bash
   ./bhc Upload configs.txt local.txt /remote/path/remote.txt
   ```

## Logging
- **`ssh_results.txt`**: Logs successful operations.
- **`ssh_error.txt`**: Logs errors encountered during operations.

## Dependencies
- **Tokio**: For asynchronous file and network operations.
- **Russh**: For SSH connections.
- **Russh-SFTP**: For file transfers over SSH.

## Build Instructions
1. Install Rust and Cargo.
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the binary:
   ```bash
   ./target/release/bhc
   ```

## Testing
Unit tests are provided for the `readfiles` module. Run tests using:
```bash
cargo test
```

## Contributing
Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.