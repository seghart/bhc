// Define the Config struct to store parsed configuration information for each line.
#[derive(Debug)]
pub struct Config {
    pub ip: String,       // Stores the IP address
    pub port: u16,        // Stores the port number, u16 represents an unsigned 16-bit integer
    pub user: String,     // Stores the username
    pub password: String, // Stores the password
}

// Function: Reads the configuration file and returns a Result containing a Vec<Config> for multiple configurations or an I/O error.
pub fn read_config_file(file_path: &str) -> std::io::Result<Vec<Config>> {
    // Use std::path::Path to create the file path
    let path = std::path::Path::new(file_path);
    // Attempt to open the file, return Err if it fails, `?` propagates the error
    let file = std::fs::File::open(&path)?;
    // Wrap the file object with BufReader to read the file line by line
    let reader = std::io::BufReader::new(file);

    // Create an empty Vec to store the parsed Config structs
    let mut configs = Vec::new();

    // Iterate over each line in the file. Each line is of type Result<String, io::Error>
    for line in std::io::BufRead::lines(reader) {
        // Unwrap the Result using `?`, returning an error if one occurs
        let line = line?;

        // If the line is not empty, proceed with processing
        if !line.is_empty() {
            // Split the line by ',' to separate parameters, collecting the results into a Vec<&str>
            let params: Vec<&str> = line.split(',').collect();

            // Create an empty Config struct to populate with data
            let mut config = Config {
                ip: String::new(),
                port: 0, // Default port is 0, will be parsed later
                user: String::new(),
                password: String::new(),
            };

            // Iterate over each split key-value pair
            for param in params {
                // Split each parameter by '=' into a key and value, `pair` is a Vec<&str>
                let pair: Vec<&str> = param.split('=').collect();

                // If `pair` has two elements (key and value), process it
                if pair.len() == 2 {
                    // Match the key name to parse and store the corresponding value
                    match pair[0] {
                        // If the key is "ip", assign the value to `config.ip`
                        "ip" => config.ip = pair[1].to_string(),
                        // If the key is "port", attempt to parse it as u16, default to 22 if parsing fails
                        "port" => config.port = pair[1].parse().unwrap_or(22),
                        // If the key is "user", assign the value to `config.user`
                        "user" => config.user = pair[1].to_string(),
                        // If the key is "password", assign the value to `config.password`
                        "password" => config.password = pair[1].to_string(),
                        // Ignore other irrelevant key-value pairs
                        _ => (),
                    }
                }
            }

            // Add the parsed Config struct to the configs vector
            configs.push(config);
        }
    }

    // Return Ok wrapping the Vec<Config> if all configurations are successfully read
    Ok(configs)
}
