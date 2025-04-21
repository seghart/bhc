use async_trait::async_trait;
use russh::client::{self, Config as RusshConfig, Handler};
use russh_keys::key::PublicKey;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Comm {
    pub command: String,
}

#[derive(Debug)]
struct MyHandler;

#[async_trait]
impl Handler for MyHandler {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn check_server_key(self: &mut Self, _: &PublicKey) -> Result<bool, Self::Error> {
        // 临时接受服务器密钥
        Ok(true)
    }
}

pub async fn ssh_connection(
    config: Config,
    command: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let russh_config: Arc<RusshConfig> = Arc::new(RusshConfig::default());

    let address: String = format!("{}:{}", config.ip, config.port);
    let handler: MyHandler = MyHandler;

    // 连接到 SSH 服务器
    let mut session: client::Handle<MyHandler> =
        client::connect(russh_config, address, handler).await?;

    // 使用提供的密码进行认证
    session
        .authenticate_password(&config.user, &config.password)
        .await?;

    // 打开会话通道
    let mut channel: russh::Channel<client::Msg> = session.channel_open_session().await?;
    if command.contains("sudo") {
        // 构建执行 sudo 命令的完整命令
        let sudo_command = format!("echo {} | sudo -S {}", config.password, command); // 使用 echo 来传递密码
        channel.exec(true, sudo_command.as_bytes()).await?; // 转换为字节切片
    } else {
        channel.exec(true, command.as_bytes()).await?; // 转换为字节切片
    }
    // // 构建执行 sudo 命令的完整命令
    // let sudo_command = format!("echo {} | sudo -S {}", config.password, command); // 使用 echo 来传递密码
    // channel.exec(true, sudo_command.as_bytes()).await?; // 转换为字节切片

    let mut output: Vec<u8> = Vec::new();
    loop {
        match channel.wait().await {
            Some(russh::ChannelMsg::Data { data }) => {
                output.extend_from_slice(&data);
            }
            Some(russh::ChannelMsg::Eof) => {
                break;
            }
            None => {
                break;
            }
            _ => {}
        }
    }

    // 将输出转换为字符串
    let output_str = String::from_utf8_lossy(&output).to_string();
    //println!("命令输出: {}", output_str);

    // 返回命令输出
    Ok(output_str)
}
