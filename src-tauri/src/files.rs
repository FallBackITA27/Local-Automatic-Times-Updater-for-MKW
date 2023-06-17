use std::io::Write;

fn create_config(path: &std::path::Path) {
    let mut config_file = std::fs::File::create(path).unwrap();
    config_file.write_all("## Do not modify this file manually unless you know what you're doing. Use the CLI to modify the values here. ##\n".as_bytes()).unwrap();
    config_file.write_all("CHADSOFTUSER=\n".as_bytes()).unwrap();
    config_file.write_all("MKWPPUSER=\n".as_bytes()).unwrap();
    config_file.write_all("MKLUSER=\n".as_bytes()).unwrap();
}

#[tauri::command]
pub fn read_config() -> String {
    let path = std::path::Path::new("./config.cfg");
    if !path.exists() {
        create_config(path);
    }
    let file = std::fs::read_to_string("./config.cfg").unwrap();
    let mut output = "{".to_string();
    let split = file.split('\n');
    for line in split {
        if line.starts_with('#') {
            continue;
        }
        let mut pair = line.split('=');
        let key = pair.next().unwrap();
        if let Some(val) = pair.next() {
            if val.is_empty() { continue }
            match key {
                "CHADSOFTUSER" => output = format!(r##"{output}"chadUser":"https://www.chadsoft.co.uk/time-trials/players/{val}.html","##),
                "MKWPPUSER" => output = format!(r##"{output}"mkwppUser":"https://www.mariokart64.com/mkw/profile.php?pid={val}","##),
                "MKLUSER" => output = format!(r##"{output}"mklUser":"https://www.mkleaderboards.com/mkw/players/{val}","##),
                _ => continue,
            }
        }
    }
    output+="}";
    return output.replace(",}","}");
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
    let overwrite = val + "\n" + part_two.splitn(2,'\n').last().unwrap_or("");
    file.write_all((part_one.to_string()+&split_param+&overwrite).as_bytes()).unwrap();
}