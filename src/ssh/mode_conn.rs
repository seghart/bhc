use async_trait::async_trait;
use russh::client::{self, Config as RusshConfig, Handler};
use russh_keys::key::PublicKey;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::OpenFlags;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
        // Temporarily accept the server key
        Ok(true)
    }
}
impl Config {
    async fn connect_and_authenticate(
        &self,
    ) -> Result<client::Handle<MyHandler>, Box<dyn std::error::Error + Send + Sync>> {
        let russh_config: Arc<RusshConfig> = Arc::new(RusshConfig::default());

        let address: String = format!("{}:{}", self.ip, self.port);
        let handler: MyHandler = MyHandler;

        // Connect to the SSH server
        let mut session: client::Handle<MyHandler> =
            client::connect(russh_config, address, handler).await?;

        // Authenticate using the provided password
        session
            .authenticate_password(&self.user, &self.password)
            .await?;

        Ok(session)
    }
    async fn open_sftp_session(
        &self,
    ) -> Result<SftpSession, Box<dyn std::error::Error + Send + Sync>> {
        let session = self.connect_and_authenticate().await?;
        let channel: russh::Channel<client::Msg> = session.channel_open_session().await?;
        channel.request_subsystem(true, "sftp").await.unwrap();
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
        Ok(sftp)
    }
}
impl Comm {
    pub fn new(command: &str) -> Self {
        Comm {
            command: command.to_string(),
        }
    }
    async fn execute_command(
        &self,
        session: &mut client::Handle<MyHandler>,
        command: &str,
        password: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Open a session channel
        let mut channel: russh::Channel<client::Msg> = session.channel_open_session().await?;
        if command.contains("sudo") {
            // Construct the full command to execute with sudo
            let sudo_command = format!("echo {} | sudo -S {}", password, command); // Use echo to pass the password
            channel.exec(true, sudo_command.as_bytes()).await?; // Convert to byte slice
        } else {
            channel.exec(true, command.as_bytes()).await?; // Convert to byte slice
        }
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
        // Convert the output to a string
        let output_str = String::from_utf8_lossy(&output).to_string();
        // Return the command output
        Ok(output_str)
    }
}

pub async fn ssh_command_mode_conn(
    config: Config,
    command: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut session = config.connect_and_authenticate().await?;
    let comm = Comm::new(command);
    let out_put = comm
        .execute_command(&mut session, command, &config.password)
        .await?;
    Ok(out_put)
}

pub async fn ssh_upload_mode_conn(
    config: Config,
    local_path: &str,
    remote_path: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Connect to the SSH server and open an SFTP session
    let sftp = config.open_sftp_session().await?;

    // Open the local file and read its contents
    let mut local_file = File::open(local_path).await.unwrap();
    let mut buffer = Vec::new();
    local_file.read_to_end(&mut buffer).await.unwrap();

    // Open the remote file and write the contents
    let mut remote_file = sftp
        .open_with_flags(
            remote_path,
            OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE,
        )
        .await
        .unwrap();
    remote_file.write_all(&buffer).await.unwrap();
    remote_file.flush().await.unwrap();

    // Return the result of the SFTP operation
    Ok(format!("File successfully transferred to {}", remote_path))
}
