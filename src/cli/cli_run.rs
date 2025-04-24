use std::env;
#[derive()]
pub struct Cli {
    pub status: Modes,
}
pub enum Modes {
    Upload {
        conn_path: String,
        local_path: String,
        remote_path: String,
    },
    Command {
        conn_path: String,
        command: String,
    },
}
/// `Cli` 结构体提供了从命令行参数中获取命令路径和连接路径的方法。
///
/// # 方法
///
/// - `get_cmd_path(&self) -> &String`:
///   返回命令路径字符串的引用。
///
/// - `get_conn_path(&self) -> &String`:
///   返回连接路径字符串的引用。
///
/// - `cmd() -> Cli`:
///   通过读取命令行参数中的命令路径和连接路径来构造一个 `Cli` 实例。
///   如果没有提供参数，则为相应字段分配默认的错误消息。
impl Cli {
    // pub fn get_cmd_path(&self) -> &String {
    //     let string_pattern = &self.cmd_path;
    //     string_pattern
    // }
    // pub fn get_conn_path(&self) -> &String {
    //     let string_path = &self.conn_path;
    //     string_path
    // }
    pub fn get_command_params(&self) -> (&str, &str) {
        match &self.status {
            Modes::Command { conn_path, command } => (conn_path, command),
            _ => panic!("当前模式不是 Command"),
        }
    }
    pub fn get_upload_params(&self) -> (&str, &str, &str) {
        match &self.status {
            Modes::Upload {
                conn_path,
                local_path,
                remote_path,
            } => (conn_path, local_path, remote_path),
            _ => panic!("当前模式不是 Upload"),
        }
    }

    pub fn new() -> Self {
        let comm = env::args().nth(1).expect("没有提供Upload或Command参数");
        match comm.as_str() {
            "Upload" => {
                let conn_path = env::args().nth(2).expect("没有提供连接路径参数");
                let local_path = env::args().nth(3).expect("没有提供本地路径参数");
                let remote_path = env::args().nth(4).expect("没有提供远程路径参数");
                Cli {
                    status: Modes::Upload {
                        conn_path,
                        local_path,
                        remote_path,
                    },
                }
            }
            "Command" => {
                let conn_path = env::args().nth(2).expect("没有提供连接路径参数");
                let command = env::args().nth(3).expect("没有提供命令参数");
                Cli {
                    status: Modes::Command { conn_path, command },
                }
            }
            _ => {
                eprintln!("无效的参数，请使用 Upload 或 Command");
                std::process::exit(1);
            }
        }
    }
}
