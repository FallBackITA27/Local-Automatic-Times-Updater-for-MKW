use std::{collections::HashMap, hash::Hash};
use colored::Colorize;

mod terminal;
mod files;
mod sr;

#[allow(clippy::needless_return)]
#[deny(clippy::needless_borrow)]

macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().to_lowercase();
    };
}

// https://rosettacode.org/wiki/Levenshtein_distance#Rust
fn lev_dist(word1: &str, word2: &str) -> u8 {
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
 
    let word1_length = w1.len() + 1;
    let word2_length = w2.len() + 1;
 
    let mut matrix = vec![vec![0; word1_length]; word2_length];
 
    for i in 1..word1_length { matrix[0][i] = i; }
    for j in 1..word2_length { matrix[j][0] = j; }
 
    for j in 1..word2_length {
        for i in 1..word1_length {
            let x: usize = if w1[i-1] == w2[j-1] {
                matrix[j-1][i-1]
            } else {
                1 + std::cmp::min(
                        std::cmp::min(matrix[j][i-1], matrix[j-1][i])
                        , matrix[j-1][i-1])
            };
            matrix[j][i] = x;
        }
    }
    matrix[word2_length-1][word1_length-1] as u8
}

#[tokio::main]
async fn main() {
    let tracks_chadsoft_hash_thread = std::thread::spawn(grab_chadsoft_tracks_hashmap);
    let tracks_chadsoft_arr_thread = std::thread::spawn(grab_chadsoft_tracks_array);
    let tracks_mkwpp_arr_thread = std::thread::spawn(grab_mkwpp_tracks_array);

    terminal::welcome_text();

    let path = std::path::Path::new("./config.cfg");
    if !path.exists() {
        files::create_config(path);
    } else {
        std::mem::drop(path);
    }
    
    let user_thread = std::thread::spawn(files::read_config);


    print!("| Chadsoft Track Hashmap");
    while !tracks_chadsoft_hash_thread.is_finished() {
        terminal::loading();
    };
    print!("\t[{}]\n| Chadsoft Track Array","✔".green());
    terminal::flush_stdout();
    let tracks_chadsoft_hash = tracks_chadsoft_hash_thread.join().unwrap().await;
    let all_chadsoft_links = tracks_chadsoft_hash.clone().into_keys().collect::<Vec<String>>();

    while !tracks_chadsoft_arr_thread.is_finished() {
        terminal::loading();
    };
    print!("\t\t[{}]\n| MKWPP Track Array","✔".green());
    terminal::flush_stdout();
    let tracks_chadsoft_arr = tracks_chadsoft_arr_thread.join().unwrap().await;

    while !tracks_mkwpp_arr_thread.is_finished() {
        terminal::loading();
    };
    print!("\t\t[{}]\n| User Data","✔".green());
    terminal::flush_stdout();
    let tracks_mkwpp_arr = tracks_mkwpp_arr_thread.join().unwrap().await;

    while !user_thread.is_finished() {
        terminal::loading();
    };
    let mut user = user_thread.join().unwrap();
    print!("\t\t\t[{}]\n","✔".green());
    terminal::flush_stdout();

    let command_list = ["q","quit","help","cfg","run"];

    loop {
        print!("\n>> ");
        terminal::flush_stdout();
        read_str!(input);
        let args: Vec<&str> = input.trim().split(" ").collect();
        let arg_0 = args.get(0).unwrap_or(&"").to_owned();
        match arg_0 {
            "q" | "quit" => {
                terminal::quit();
                break;
            },
            "help" => terminal::help_command(),
            "cfg" => {
                let key = match args.get(1) {
                    Some(key) => key.to_owned(),
                    None => {
                        println!("{}","Error! No CFG parameter found.".red());
                        continue;
                    }
                };
                match key {
                    "chadsoft" => {
                        let error = "Error! No Chadsoft url found.".red();
                        let url = match args.get(2) {
                            Some(key) => key.to_owned(),
                            None => {
                                println!("{error}");
                                continue;
                            }
                        };
                        if !url.contains("chadsoft.co.uk") {
                            println!("{error}");
                            continue;
                        }
                        if !url.contains("/time-trials/players/") {
                            println!("{error}");
                            continue;
                        };
                        files::write_config("CHADSOFTUSER".to_string(),url.split(".html").next().unwrap().split("/players/").last().unwrap().to_string().to_uppercase());
                        let user_thread = std::thread::spawn(files::read_config);
                        print!("{}","User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        print!("\t[{}]\n","✔".green());
                        println!("{} {} {} {}","Successfully saved".bright_blue(), "CHADSOFTUSER".bright_blue().bold(), "as".bright_blue(), user.chadsoft_id.bright_blue().bold());
                    },
                    "mkwpp" => {
                        let url = match args.get(2) {
                            Some(key) => key.to_owned(),
                            None => {
                                println!("{}","Error! No MKWPP url found.".red());
                                continue;
                            }
                        };
                        if !url.contains("mariokart64.com") {
                            println!("{}","Error! No MKWPP url found.".red());
                            continue;
                        }
                        if !url.contains("profile.php?pid=") {
                            println!("{}","Error! No MKWPP url found.".red());
                            continue;
                        };
                        files::write_config("MKWPPUSER".to_string(),url.split("profile.php?pid=").last().unwrap().to_string());
                        let user_thread = std::thread::spawn(files::read_config);
                        print!("{}","User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        print!("\t[{}]\n","✔".green());
                        println!("{} {} {} {}","Successfully saved".bright_blue(), "MKWPPUSER".bright_blue().bold(), "as".bright_blue(), user.mkwpp_id.bright_blue().bold());
                    },
                    "reload" => {
                        let user_thread = std::thread::spawn(files::read_config);
                        print!("{}","User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        print!("\t[{}]\n","✔".green());
                    },
                    _ => println!("{}","Error! No valid CFG parameter found.".red())
                }
            },
            "run" => {
                let mode = match args.get(1) {
                    Some(mode) => mode.to_owned(),
                    None => {
                        println!("{}","Error! No mode found.".red());
                        continue;
                    }
                };
                match mode {
                    _ => mkwpp_mode(user.mkwpp_id.clone(),user.chadsoft_id.clone(),tracks_chadsoft_hash.clone(),all_chadsoft_links.clone()).await
                }
            }
            _ => {
                println!("{} {}",arg_0.red().bold(), "is not a command!".red());
                let mut min: u8 = u8::MAX;
                let mut suggestion = "";
                for command in command_list {
                    let num = lev_dist(arg_0,command);
                    if num < min {
                        min = num;
                        suggestion = command;
                        if num < 1 {
                            break;
                        }
                    }
                }
                println!("Did you mean {}?",suggestion.bold());
            }
        }
    }
}

async fn grab_chadsoft_tracks_array() -> Vec<[String; 2]> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_array.json");
    let json: Vec<[String; 2]> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_mkwpp_tracks_array() -> Vec<[String; 2]> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/mkwpp_track_array.json");
    let json: Vec<[String; 2]> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_chadsoft_tracks_hashmap() -> HashMap<String,String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_mapping.json");
    let json: HashMap<String,String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_times_ctgp(chadsoft_id: String, track_hash: HashMap<String,String>) -> [HashMap<String,(i32,String)>; 2]{
    let url = format!("https://tt.chadsoft.co.uk/players/{}.json",chadsoft_id);
    let mut text = reqwest::get(&url).await.unwrap().text().await.unwrap();
    text.remove(0); // WTF Chadsoft. https://discord.com/channels/485882824881209345/485900922468433920/1102240594174148729 (The Bean Corner Discord).
    let json: HashMap<String,serde_json::Value> = serde_json::from_str(&text).unwrap();
    let mut times_3lap_map: HashMap<String,(i32,String)> = HashMap::default();
    let mut times_flap_map: HashMap<String,(i32,String)> = HashMap::default();
    let ghosts = json["ghosts"].as_array().unwrap();
    for ghost in ghosts {
        let track_link = ghost["_links"]["leaderboard"]["href"].as_str().unwrap().to_string();
        let track_name = match track_hash.get(&track_link) {
            Some(t_name) => t_name,
            None => continue
        };
        let ghost_time_3lap = sr::time_to_ms(ghost["finishTimeSimple"].as_str().unwrap().to_string());
        let ghost_time_flap = sr::time_to_ms(ghost["bestSplitSimple"].as_str().unwrap().to_string());
        let date = ghost["dateSet"].as_str().unwrap().split("T").next().unwrap().to_string();
        match times_3lap_map.get(track_name) {
            Some(inserted_time) => if inserted_time.0 > ghost_time_3lap {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, date.clone()));
            },
            None => {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, date.clone()));
            }
        };
        match times_flap_map.get(track_name) {
            Some(inserted_time) => if inserted_time.0 > ghost_time_flap {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, date));
            },
            None => {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, date));
            }
        };
    }
    return [times_3lap_map,times_flap_map];
}

async fn mkwpp_mode(mkwpp_id: String, chadsoft_id: String, track_hash: HashMap<String,String>, all_links: Vec<String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    if mkwpp_id.is_empty() {
        println!("{} {}","You must link your MKWPP profile with".red(),"` cfg mkwpp <mkwpp-url> `".red().bold());
        exit = true;
    }
    if exit {
        return;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { grab_times_ctgp(chadsoft_id, track_hash).await });
    
    
    
    chadsoft_times_thread.join().unwrap().await;
}