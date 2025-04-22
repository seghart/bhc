mod cli;
mod readfiles;
mod ssh;
use cli::cli_run::Cli;
use ssh::ssh_mode_conn;
use std::fs::File;
use std::io::Write;
use tokio;

fn convert_config(user_config: Vec<readfiles::user::Config>) -> Vec<ssh_mode_conn::Config> {
    user_config
        .into_iter()
        .map(|c: readfiles::user::Config| ssh_mode_conn::Config {
            ip: c.ip,
            port: c.port,
            user: c.user,
            password: c.password,
        })
        .collect()
}

fn convert_comm(command_comm: Vec<readfiles::command::Comm>) -> Vec<ssh_mode_conn::Comm> {
    command_comm
        .into_iter()
        .map(|c| ssh_mode_conn::Comm { command: c.command })
        .collect()
}
async fn ssh_cmd_command(cli: Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (conn_path, command) = cli.get_command_params();

    // 从文件中读取配置和命令
    let user_configs: Vec<readfiles::user::Config> = readfiles::user::read_config_file(&conn_path)?;
    let command_comm: Vec<readfiles::command::Comm> =
        readfiles::command::read_comm_config_file(&command)?;

    // 将配置和命令转换为 rsssh 模块中所需的类型
    let configs: Vec<ssh_mode_conn::Config> = convert_config(user_configs);
    let commands: Vec<ssh_mode_conn::Comm> = convert_comm(command_comm);

    // 创建或打开输出文件
    let mut output_file = File::create("ssh_results.txt")?;

    // 遍历所有配置和命令进行 SSH 连接
    for config in configs {
        for command in &commands {
            match ssh_mode_conn::ssh_command_mode_conn(config.clone(), command.command.as_str())
                .await
            {
                Ok(output) => {
                    let success_message: String = format!(
                        "服务器 {} SSH 执行 {} 命令成功完成。\n输出:\n{}\n",
                        config.ip, command.command, output
                    );
                    // 控制台输出
                    output_file
                        .write_all(success_message.as_bytes())
                        .expect("成功日志文件写入失败"); // 写入文件
                }
                Err(e) => {
                    let error_message: String =
                        format!("服务器 {} SSH 执行失败: {:?}\n", config.ip, e);
                    let output_err_file = File::create("ssh_error.txt");
                    eprintln!("{}", error_message); // 控制台输出
                    output_err_file
                        .expect("错误日志文件创建失败,")
                        .write_all(error_message.as_bytes())
                        .expect("错误日志文件写入失败"); // 写入文件
                }
            }
        }
    }

    Ok(())
}

async fn ssh_upload_mode(cli: Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (conn_path, local_path, remote_path) = cli.get_upload_params();

    // 从文件中读取配置和命令
    let user_configs: Vec<readfiles::user::Config> = readfiles::user::read_config_file(&conn_path)?;

    // 将配置和命令转换为 rsssh 模块中所需的类型
    let configs: Vec<ssh_mode_conn::Config> = convert_config(user_configs);

    // 创建或打开输出文件
    let mut output_file = File::create("ssh_results.txt")?;

    // 遍历所有配置和命令进行 SSH 连接
    for config in configs {
        match ssh_mode_conn::ssh_upload_mode_conn(config.clone(), local_path, remote_path).await {
            Ok(output) => {
                let success_message: String = format!(
                    "服务器 {} SSH 上传文件{}到{}成功完成。\n输出:\n{}\n",
                    config.ip, local_path, remote_path, output
                );
                // 控制台输出
                output_file
                    .write_all(success_message.as_bytes())
                    .expect("成功日志文件写入失败"); // 写入文件
            }
            Err(e) => {
                let error_message: String = format!("服务器 {} SSH 执行失败: {:?}\n", config.ip, e);
                let output_err_file = File::create("ssh_error.txt");
                eprintln!("{}", error_message); // 控制台输出
                output_err_file
                    .expect("错误日志文件创建失败,")
                    .write_all(error_message.as_bytes())
                    .expect("错误日志文件写入失败"); // 写入文件
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = cli::cli_run::Cli::cmd();
    match cli.status {
        cli::cli_run::Modes::Command { .. } => {
            ssh_cmd_command(cli).await.expect("SSH 命令执行失败");
        }
        cli::cli_run::Modes::Upload { .. } => {
            ssh_upload_mode(cli).await.expect("SSH 上传文件失败");
        }
    }
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
