use std::{collections::HashMap, path, fs, io::Write};
use colored::Colorize;

#[allow(clippy::needless_return)]
#[deny(clippy::needless_borrow)]

macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

struct UserVars {
    ChadsoftId: String,
    MkwppId: String
}

impl Default for UserVars {
    fn default() -> Self {
        UserVars {
            ChadsoftId: "".to_string(),
            MkwppId: "".to_string()
        }
    }
}

#[cfg(any(target_os = "linux",target_os="mac_os"))]
fn clear_terminal() {
    std::process::Command::new("clear").output().unwrap();
}
#[cfg(target_os = "windows")]
fn clear_terminal() {
    std::process::Command::new("cls").output().unwrap();
}

#[tokio::main]
async fn main() {
    let tracks_hash_thread = std::thread::spawn(grab_tracks_hashmap);
    let tracks_arr_thread = std::thread::spawn(grab_tracks_array);

    clear_terminal();

    println!("\n\nWelcome to the Automatic Times Updater for Mario Kart Wii!");
    println!("Just write \"help\" to start if you don't know what you're doing.");
    println!("\n{}\n\n\n\n\n\n","Written by FalB.".purple());

    let config_path = path::Path::new("./config.cfg");
    if !config_path.exists() {
        let mut config_file = fs::File::create(config_path).unwrap();

        config_file.write_all("## Do not modify this file manually unless you know what you're doing. Use the CLI to modify the values here. ##\n".as_bytes()).unwrap();
        config_file.write_all("CHADSOFTUSER=\n".as_bytes()).unwrap();
        config_file.write_all("MKWPPUSER=\n".as_bytes()).unwrap();

        println!("{} {} {}","Created".bright_blue(), "config.cfg".bright_blue().bold(), "file.".bright_blue());
        println!("{} {} {}\n\n","Remember to link your Chadsoft account with".bright_blue(), "` --config --chadsoft=<chadsoft-url> `".bright_blue().bold(), "!!".bright_blue());
    }

    read_str!(input);

    println!(">> {input}\n");
    let args: Vec<&str> = input.split(" ").collect();
    if args.contains(&"q") || args.contains(&"quit") {
        return;
    }

    let user = read_config();

    let tracks_hash = tracks_hash_thread.join().unwrap().await;
    let tracks_arr = tracks_arr_thread.join().unwrap().await;
}

async fn grab_tracks_array() -> Vec<[String; 2]> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_array.json");
    let json: Vec<[String; 2]> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_tracks_hashmap() -> HashMap<String,String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_mapping.json");
    let json: HashMap<String,String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

fn read_config() -> UserVars {
    let file = fs::read_to_string("./config.cfg").unwrap();
    let mut user_variables = UserVars::default();
    let split = file.split("\n");
    for line in split {
        if line.starts_with("#") {
            continue;
        }
        let mut pair = line.split("=");
        let key = pair.next().unwrap();
        let val = pair.next().unwrap();
        match key {
            "CHADSOFTUSER" => {
                if val.is_empty() {
                    println!("{} {} {}","You must link your Chadsoft account with".red(),"` --config --chadsoft=<chadsoft-url> `".red().bold(),"!!".red());
                    panic!("");
                }
                user_variables.ChadsoftId = val.to_string()
            },
            "MKWPPUSER" => user_variables.MkwppId = val.to_string(),
            _ => continue,
        }
    }
    return user_variables;
}

fn write_config(key: String, val: String) {
    // This is all written assuming the key exists.
    let mut file = fs::File::open("./config.cfg").unwrap();
    let file_string = fs::read_to_string("./config.cfg").unwrap();
    let split_param = key + "=";
    let mut split = file_string.split(&split_param);
    let part_one = split.next().unwrap();
    let part_two = split.next().unwrap();
    let overwrite = val + "\n" + part_two.splitn(1,"\n").last().unwrap();
    file.write_all((part_one.to_string()+&split_param+&overwrite).as_bytes()).unwrap();
}