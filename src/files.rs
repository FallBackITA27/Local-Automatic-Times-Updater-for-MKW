use colored::Colorize;
use std::io::Write;

pub struct UserVars {
    pub chadsoft_id: String,
    pub mkwpp_id: String
}

impl Default for UserVars {
    fn default() -> Self {
        UserVars {
            chadsoft_id: "".to_string(),
            mkwpp_id: "".to_string()
        }
    }
}

pub fn create_config(path: &std::path::Path) {
    let mut config_file = std::fs::File::create(path).unwrap();

    config_file.write_all("## Do not modify this file manually unless you know what you're doing. Use the CLI to modify the values here. ##\n".as_bytes()).unwrap();
    config_file.write_all("CHADSOFTUSER=\n".as_bytes()).unwrap();
    config_file.write_all("MKWPPUSER=\n".as_bytes()).unwrap();

    println!("{} {} {}","Created".bright_blue(), "config.cfg".bright_blue().bold(), "file.".bright_blue());
    println!("{} {}\n\n","Remember to link your Chadsoft account with".bright_blue(), "` cfg chadsoft <chadsoft-url> `".bright_blue().bold());
}

pub fn read_config() -> UserVars {
    let file = std::fs::read_to_string("./config.cfg").unwrap();
    let mut user_variables = UserVars::default();
    let split = file.split("\n");
    for line in split {
        if line.starts_with("#") {
            continue;
        }
        let mut pair = line.split("=");
        let key = pair.next().unwrap();
        let val = pair.next().unwrap_or("").to_string();
        // Do not delete, I should implement this error somewhere in the future.
        // println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        match key {
            "CHADSOFTUSER" => user_variables.chadsoft_id = val,
            "MKWPPUSER" => user_variables.mkwpp_id = val,
            _ => continue,
        }
    }
    return user_variables;
}

pub fn write_config(key: String, val: String) {
    // This is all written assuming the key exists.
    let path = "./config.cfg";
    let file_string = std::fs::read_to_string(path).unwrap();
    std::fs::remove_file(path).unwrap();
    let mut file = std::fs::File::create(path).unwrap();
    let split_param = key + "=";
    let mut split = file_string.split(&split_param);
    let part_one = split.next().unwrap();
    let part_two = split.next().unwrap_or("");
    let overwrite = val + "\n" + part_two.splitn(2,"\n").last().unwrap_or("");
    file.write_all((part_one.to_string()+&split_param+&overwrite).as_bytes()).unwrap();
}

/*
## Do not modify this file manually unless you know what you're doing. Use the CLI to modify the values here. ##
CHADSOFTUSER=1f/7b7d3331a3a008.html
MKWPPUSER=1662
*/