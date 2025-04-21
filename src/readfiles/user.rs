// 定义 Config 结构体，用于存储每一行解析后的配置信息。
#[derive(Debug)]
pub struct Config {
    pub ip: String,        // 用于存储 IP 地址
    pub port: u16,         // 用于存储端口号，u16 类型表示无符号16位整数
    pub user: String,      // 用于存储用户名
    pub password: String,  // 用于存储密码
}

// 函数：读取配置文件并返回 Result，其中包含 Vec<Config> 代表多个配置，或是 I/O 错误。
pub fn read_config_file(file_path: &str) -> std::io::Result<Vec<Config>> {
    // 使用 std::path::Path 来创建文件路径
    let path = std::path::Path::new(file_path);
    // 尝试打开文件，如果失败则返回 Err，`?` 会自动传播错误
    let file = std::fs::File::open(&path)?;
    // 使用 BufReader 包装文件对象，用于逐行读取文件内容
    let reader = std::io::BufReader::new(file);

    // 创建一个空的 Vec，用于存储解析后的 Config 结构体
    let mut configs = Vec::new();

    // 遍历每一行文件内容。每一行是 Result<String, io::Error> 类型
    for line in std::io::BufRead::lines(reader) {
        // 通过 `?` 操作符解包 Result，遇到错误会直接返回错误
        let line = line?;

        // 如果行内容不为空，继续处理
        if !line.is_empty() {
            // 使用 `split(',')` 按逗号分割参数，结果收集到一个 Vec<&str>
            let params: Vec<&str> = line.split(',').collect();

            // 创建一个空的 Config 结构体，准备填充数据
            let mut config = Config {
                ip: String::new(),
                port: 0, // 默认端口为 0，后面解析
                user: String::new(),
                password: String::new(),
            };

            // 遍历每个分割后的键值对
            for param in params {
                // 每个参数按 `=` 分割成键和值，`pair` 是 Vec<&str>
                let pair: Vec<&str> = param.split('=').collect();

                // 如果 `pair` 有两个元素（键和值），则进行处理
                if pair.len() == 2 {
                    // 根据键名来匹配，解析并存储相应的值
                    match pair[0] {
                        // 如果键名是 "ip"，则赋值给 `config.ip`
                        "ip" => config.ip = pair[1].to_string(),
                        // 如果键名是 "port"，则尝试解析成 u16 类型，解析失败则使用默认值 22
                        "port" => config.port = pair[1].parse().unwrap_or(22),
                        // 如果键名是 "user"，则赋值给 `config.user`
                        "user" => config.user = pair[1].to_string(),
                        // 如果键名是 "password"，则赋值给 `config.password`
                        "password" => config.password = pair[1].to_string(),
                        // 忽略其他无关的键值对
                        _ => (),
                    }
                }
            }

            // 将解析后的 Config 结构体添加到 configs 向量中
            configs.push(config);
        }
    }

    // 如果成功读取所有配置，返回 Ok 包装的 Vec<Config>
    Ok(configs)
}
