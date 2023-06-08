use std::{collections::HashMap, path, fs, io::{Write, Cursor}, sync::{Arc, atomic::{AtomicBool, Ordering}}};
use colored::Colorize;

mod terminal;

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
    chadsoft_id: String,
    mkwpp_id: String
}

impl Default for UserVars {
    fn default() -> Self {
        UserVars {
            chadsoft_id: "".to_string(),
            mkwpp_id: "".to_string()
        }
    }
}

#[tokio::main]
async fn main() {
    let tracks_hash_thread = std::thread::spawn(grab_tracks_hashmap);
    let tracks_arr_thread = std::thread::spawn(grab_tracks_array);

    terminal::clear();

    println!("\n\nWelcome to the Automatic Times Updater for Mario Kart Wii!");
    println!("Just write {} to start if you don't know what you're doing.","` help `".bold());
    println!("\n{} {}{}\n\n\n","Written by".purple(),"FalB".purple().bold().on_bright_magenta(),".".purple());

    let config_path = path::Path::new("./config.cfg");
    if !config_path.exists() {
        let mut config_file = fs::File::create(config_path).unwrap();

        config_file.write_all("## Do not modify this file manually unless you know what you're doing. Use the CLI to modify the values here. ##\n".as_bytes()).unwrap();
        config_file.write_all("CHADSOFTUSER=\n".as_bytes()).unwrap();
        config_file.write_all("MKWPPUSER=\n".as_bytes()).unwrap();

        println!("{} {} {}","Created".bright_blue(), "config.cfg".bright_blue().bold(), "file.".bright_blue());
        println!("{} {}\n\n","Remember to link your Chadsoft account with".bright_blue(), "` cfg --chadsoft=<chadsoft-url> `".bright_blue().bold());
    }

    let mut user_thread = std::thread::spawn(read_config);

    print!("Track Hashmap");
    while !tracks_hash_thread.is_finished() {
        terminal::loading();
    };
    print!("\t[{}]\nTrack Array","✔".green());
    terminal::flush_stdout();
    let tracks_hash = tracks_hash_thread.join().unwrap().await;
    while !tracks_arr_thread.is_finished() {
        terminal::loading();
    };
    print!("\t[{}]\nUser Data","✔".green());
    terminal::flush_stdout();
    let tracks_arr = tracks_arr_thread.join().unwrap().await;
    while !user_thread.is_finished() {
        terminal::loading();
    };
    let mut user = user_thread.join().unwrap();
    print!("\t[{}]\n\n>> ","✔".green());
    terminal::flush_stdout();

    loop {
        read_str!(input);
        let args: Vec<&str> = input.split(" ").collect();
        if args.contains(&"q") || args.contains(&"quit") {
            quit();
            break;
        }

        print!(">> ");
        terminal::flush_stdout();
    }
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
        let val = pair.next().unwrap_or("").to_string();
        // Do not delete, I should implement this error somewhere in the future.
        // println!("{} {}","You must link your Chadsoft account with".red(),"` cfg --chadsoft=<chadsoft-url> `".red().bold());
        match key {
            "CHADSOFTUSER" => user_variables.chadsoft_id = val,
            "MKWPPUSER" => user_variables.mkwpp_id = val,
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

fn quit() {
    terminal::clear();
    println!("{}","bye bye!".green());
}