#[derive(Debug)]
pub struct Comm {
    pub command: String,
}

pub fn read_comm_config_file(cmd_path: &str) -> std::io::Result<Vec<Comm>> {
    let path = std::path::Path::new(cmd_path);
    // Try to open the file, return Err if it fails, `?` will propagate the error
    let file = std::fs::File::open(&path)?;
    // Wrap the file object with BufReader for reading the file line by line
    let reader = std::io::BufReader::new(file);
    let mut commands = Vec::new();
    // Iterate through each line in the file. Each line is of type Result<String, io::Error>
    for line in std::io::BufRead::lines(reader) {
        // Unwrap the Result using `?`, return the error if any
        let line = line?;

        // If the line is not empty, proceed
        if !line.is_empty() {
            // Split the line by ',' and collect the results into a Vec<&str>
            let params: Vec<&str> = line.split(',').collect();
            let mut comm = Comm {
                command: String::new(),
            };
            // Iterate through each split key-value pair
            for param in params {
                // Split each parameter by '=' into key and value, `pair` is Vec<&str>
                let pair: Vec<&str> = param.split('=').collect();

                // If `pair` has two elements (key and value), process it
                if pair.len() == 2 {
                    // Match the key name, parse and store the corresponding value
                    match pair[0] {
                        // If the key is "command", assign the value to `comm.command`
                        "command" => comm.command = pair[1].to_string(),
                        // Ignore other irrelevant key-value pairs
                        _ => (),
                    }
                }
            }
            // Add the parsed Comm struct to the commands vector
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
