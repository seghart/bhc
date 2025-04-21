#[derive(Debug)]
pub struct Comm{
    pub command:String,
}


pub fn read_comm_config_file(cmd_path:&str) -> std::io::Result<Vec<Comm>> {
    let path = std::path::Path::new(cmd_path);
    // 尝试打开文件，如果失败则返回 Err，`?` 会自动传播错误
    let file = std::fs::File::open(&path)?;
    // 使用 BufReader 包装文件对象，用于逐行读取文件内容
    let reader = std::io::BufReader::new(file); 
    let mut commands = Vec::new();
    // 遍历每一行文件内容。每一行是 Result<String, io::Error> 类型
    for line in std::io::BufRead::lines(reader) {
        // 通过 `?` 操作符解包 Result，遇到错误会直接返回错误
        let line = line?;

        // 如果行内容不为空，继续处理
        if !line.is_empty() {
            // 使用 `split(',')` 按逗号分割参数，结果收集到一个 Vec<&str>
            let params: Vec<&str> = line.split(',').collect();
            let mut comm = Comm{
                command:String::new(),
            };
                        // 遍历每个分割后的键值对
            for param in params {
                // 每个参数按 `=` 分割成键和值，`pair` 是 Vec<&str>
                let pair: Vec<&str> = param.split('=').collect();

                // 如果 `pair` 有两个元素（键和值），则进行处理
                if pair.len() == 2 {
                    // 根据键名来匹配，解析并存储相应的值
                    match pair[0] {
                        // 如果键名是 "command"，则赋值给 `config.ip`
                        "command" => comm.command = pair[1].to_string(),
                        // 忽略其他无关的键值对
                        _ => (),
                    }
                }
            }
            // 将解析后的 Config 结构体添加到 configs 向量中
            commands.push(comm);

        }
    }
    Ok(commands)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_comm_config_file() {
        let test_data = "\
        command=ls
        command=pwd
        command=echo Hello, world!
        ";
        let test_path = "/tmp/test_comm_config.txt";
        std::fs::write(test_path, test_data).expect("Unable to write test file");

        let result = read_comm_config_file(test_path).expect("Failed to read config file");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].command, "ls");
        assert_eq!(result[1].command, "pwd");
        assert_eq!(result[2].command, "echo Hello, world!");

        std::fs::remove_file(test_path).expect("Unable to delete test file");
    }
}