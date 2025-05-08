mod cli;
mod readfiles;
mod ssh;
use ssh::mode_conn;
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Converts user configuration from `readfiles::user::Config` to `mode_conn::Config`.
fn convert_config(user_config: Vec<readfiles::user::Config>) -> Vec<mode_conn::Config> {
    user_config
        .into_iter()
        .map(|c: readfiles::user::Config| mode_conn::Config {
            ip: c.ip,
            port: c.port,
            user: c.user,
            password: c.password,
        })
        .collect()
}

/// Converts command configuration from `readfiles::command::Comm` to `mode_conn::Comm`.
fn convert_comm(command_comm: Vec<readfiles::command::Comm>) -> Vec<mode_conn::Comm> {
    command_comm
        .into_iter()
        .map(|c| mode_conn::Comm { command: c.command })
        .collect()
}

/// Handles the SSH mode logic, including executing commands or uploading files.
async fn ssh_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = cli::cli_run::Cli::new();
    // Create or open output files
    let mut output_file = File::create("ssh_results.txt")
        .await
        .expect("Failed to create success log file");
    let mut output_err_file = File::create("ssh_error.txt")
        .await
        .expect("Failed to create error log file");

    match cli.status {
        cli::cli_run::Modes::Command { .. } => {
            let (conn_path, command) = cli.get_command_params();

            // Read configuration and commands from files
            let user_configs: Vec<readfiles::user::Config> =
                readfiles::user::read_config_file(&conn_path)?;
            let command_comm: Vec<readfiles::command::Comm> =
                readfiles::command::read_comm_config_file(&command)?;

            // Convert configurations and commands to the required types for the SSH module
            let configs: Vec<mode_conn::Config> = convert_config(user_configs);
            let commands: Vec<mode_conn::Comm> = convert_comm(command_comm);

            // Iterate through all configurations and commands to perform SSH connections
            for config in configs {
                for command in &commands {
                    match mode_conn::ssh_command_mode_conn(config.clone(), command.command.as_str())
                        .await
                    {
                        Ok(output) => {
                            let success_message: String = format!(
                                "Server {} successfully executed SSH command '{}'.\nOutput:\n{}\n",
                                config.ip, command.command, output
                            );
                            output_file
                                .write_all(success_message.as_bytes())
                                .await
                                .expect("Failed to write to success log file");
                        }
                        Err(e) => {
                            let error_message: String = format!(
                                "Server {} failed to execute SSH command: {:?}\nPlease check if the IP address is correct and the network is stable.",
                                config.ip, e
                            );
                            eprintln!("{}", error_message);
                            output_err_file
                                .write_all(error_message.as_bytes())
                                .await
                                .expect("Failed to write to error log file");
                        }
                    }
                }
            }
        }
        cli::cli_run::Modes::Upload { .. } => {
            let (conn_path, local_path, remote_path) = cli.get_upload_params();

            // Read configuration from file
            let user_configs: Vec<readfiles::user::Config> =
                readfiles::user::read_config_file(&conn_path)?;
            // Convert configurations to the required types for the SSH module
            let configs: Vec<mode_conn::Config> = convert_config(user_configs);

            // Iterate through all configurations to perform file uploads
            for config in configs {
                match mode_conn::ssh_upload_mode_conn(config.clone(), local_path, remote_path).await
                {
                    Ok(output) => {
                        let success_message: String = format!(
                            "Server {} successfully uploaded file '{}' to '{}'.\nOutput:\n{}\n",
                            config.ip, local_path, remote_path, output
                        );
                        output_file
                            .write_all(success_message.as_bytes())
                            .await
                            .expect("Failed to write to success log file");
                    }
                    Err(e) => {
                        let error_message: String = format!(
                            "Server {} failed to upload file: {:?}\nPlease check if the IP address is correct and the network is stable.",
                            config.ip, e
                        );
                        eprintln!("{}", error_message);
                        output_err_file
                            .write_all(error_message.as_bytes())
                            .await
                            .expect("Failed to write to error log file");
                    }
                }
            }
        }
    };

    Ok(())
}

/// Main entry point of the application.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ssh_mode().await.expect("Main program execution failed");
    Ok(())
}

//cargo vendor --versioned-dirs
//mkdir .cargo
//mv vendor/config.toml .cargo/
// [source.crates-io]
// replace-with = "vendored-sources"

// [source.vendored-sources]
// directory = "vendor"
//cargo build --offline --frozen --release
