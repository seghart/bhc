pub fn _file(pattern: &str, path: &str) -> Result<Option<String>, std::io::Error> {
    let _x = std::fs::read_to_string(path);
    match _x {
        Ok(t) => {
            for line in t.lines() {
                if line.contains(&pattern) {
                    //println!("{}",line);
                    let result = Ok(Some(line.to_string()));
                    return result;
                }
            }
            Ok(None)
        }
        Err(error) => {
            panic!("报错信息:{}", error);
        }
    }
}
