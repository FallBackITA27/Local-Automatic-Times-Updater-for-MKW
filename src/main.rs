use core::panic;
use std::collections::HashMap;
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

    if cfg!(windows) { std::process::Command::new("chcp").arg("65001"); }
    terminal::welcome_text();

    let path = std::path::Path::new("./config.cfg");
    if !path.exists() {
        files::create_config(path);
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
    println!("\t\t\t[{}]","✔".green());

    let command_list = ["q","quit","help","cfg","run"];

    loop {
        print!("\n>> ");
        terminal::flush_stdout();
        read_str!(input);
        let args: Vec<&str> = input.trim().split(' ').collect();
        let arg_0 = args.first().unwrap_or(&"").to_owned();
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
                        print!("User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        println!("\t[{}]","✔".green());
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
                        print!("User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        println!("\t[{}]","✔".green());
                        println!("{} {} {} {}","Successfully saved".bright_blue(), "MKWPPUSER".bright_blue().bold(), "as".bright_blue(), user.mkwpp_id.bright_blue().bold());
                    },
                    "mkl" => {
                        let url = match args.get(2) {
                            Some(key) => key.to_owned(),
                            None => {
                                println!("{}","Error! No MKL url found.".red());
                                continue;
                            }
                        };
                        if !url.contains("www.mkleaderboards.com/mkw/players/") {
                            println!("{}","Error! No MKL url found.".red());
                            continue;
                        };
                        files::write_config("MKLUSER".to_string(),url.split("/players/").last().unwrap().to_string());
                        let user_thread = std::thread::spawn(files::read_config);
                        print!("User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        println!("\t[{}]","✔".green());
                        println!("{} {} {} {}","Successfully saved".bright_blue(), "MKWPPUSER".bright_blue().bold(), "as".bright_blue(), user.mkwpp_id.bright_blue().bold());
                    },
                    "reload" => {
                        let user_thread = std::thread::spawn(files::read_config);
                        print!("User Data");
                        terminal::flush_stdout();
                        while !user_thread.is_finished() {
                            terminal::loading();
                        };
                        user = user_thread.join().unwrap();
                        println!("\t[{}]","✔".green());
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
                    "mkwpp" => mkwpp_mode(user.mkwpp_id.clone(),user.chadsoft_id.clone(),tracks_chadsoft_hash.clone(),tracks_mkwpp_arr.clone()).await,
                    "mkl" => mkl_mode(user.mkl_id.clone(),user.chadsoft_id.clone(),tracks_chadsoft_hash.clone()).await,
                    _ => println!("You must select a mode with {} or {}!","` mkwpp `".bold(),"` mkl `".bold())
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

async fn grab_mkwpp_tracks_array() -> Vec<String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/mkwpp_track_array.json");
    let json: Vec<String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_chadsoft_tracks_hashmap() -> HashMap<String,String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_mapping.json");
    let json: HashMap<String,String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_times_ctgp(chadsoft_id: String, track_hash: HashMap<String,String>) -> [HashMap<String,(i32,String,String,String)>; 2] {
    let url = format!("https://tt.chadsoft.co.uk/players/{}.json",chadsoft_id);
    let mut text = reqwest::get(&url).await.unwrap().text().await.unwrap();
    text.remove(0); // WTF Chadsoft. https://discord.com/channels/485882824881209345/485900922468433920/1102240594174148729 (The Bean Corner Discord).
    let json: HashMap<String,serde_json::Value> = match serde_json::from_str(&text) {
        Ok(json) => json,
        Err(error) => {
            let err_str = error.to_string();
            if err_str.contains("The resource cannot be found.") {
                panic!("Chadsoft is down, it cannot load the data.");
            } else {
                panic!("{text}, {error}");
            }
        }
    };
    let mut times_3lap_map: HashMap<String,(i32,String,String,String)> = HashMap::default();
    let mut times_flap_map: HashMap<String,(i32,String,String,String)> = HashMap::default();
    let ghosts = json["ghosts"].as_array().unwrap();
    for ghost in ghosts {
        let track_link = ghost["_links"]["leaderboard"]["href"].as_str().unwrap().to_string();
        let track_name = match track_hash.get(&track_link) {
            Some(t_name) => t_name,
            None => continue
        };
        let time_string_3lap = ghost["finishTimeSimple"].as_str().unwrap().to_string();
        let ghost_time_3lap = sr::time_to_ms(time_string_3lap.clone());
        let time_string_flap = ghost["bestSplitSimple"].as_str().unwrap().to_string();
        let ghost_time_flap = sr::time_to_ms(time_string_flap.clone());
        let ghost_link = ghost["_links"]["item"]["href"].as_str().unwrap().replace("json", "html").to_string();
        let date = ghost["dateSet"].as_str().unwrap().split('T').next().unwrap().to_string();
        match times_3lap_map.get(track_name) {
            Some(inserted_time) => if inserted_time.0 > ghost_time_3lap {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, time_string_3lap, date.clone(), ghost_link.clone()));
            },
            None => {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, time_string_3lap, date.clone(), ghost_link.clone()));
            }
        };
        match times_flap_map.get(track_name) {
            Some(inserted_time) => if inserted_time.0 > ghost_time_flap {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, time_string_flap, date, ghost_link));
            },
            None => {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, time_string_flap, date, ghost_link));
            }
        };
    }
    return [times_3lap_map,times_flap_map];
}

async fn grab_times_mkwpp(mkwpp_id: String, track_arr: Vec<String>) -> [HashMap<String,i32>; 2] {
    let url = format!("https://www.mariokart64.com/mkw/profile.php?pid={}",mkwpp_id);
    let player_page_req = reqwest::get(&url);

    let mut skip = false;
    let mut flap = false;
    let mut track_arr_ind: u8 = 0;
    let mut times_3lap_map: HashMap<String,i32> = HashMap::default();
    let mut times_flap_map: HashMap<String,i32> = HashMap::default();

    let player_page = player_page_req.await.unwrap().text().await.unwrap();
    let mut split = player_page.split("table");
    let nosc_body = split.nth(20).unwrap();
    let sc_body = split.nth(3).unwrap();
    let mut split_rows_nosc = nosc_body.split("tr");
    split_rows_nosc.nth(2);
    for row in split_rows_nosc {
        if skip {
            skip = false;
            continue;
        }
        if row.contains("Totals") {
            break;
        }
        if row.contains("NT") {
            if flap {
                track_arr_ind+=1;
                flap = false;
            } else {
                flap = true;
            }
        }

        let time_split;
        if flap {
            time_split = row.split("</a").nth(0).unwrap();
        } else {
            time_split = row.split("</a").nth(1).unwrap();
        }
        let time_string = time_split.split('>').last().unwrap().replace('\'',":").replace('"',".");

        if flap {
            times_flap_map.insert(track_arr.get(track_arr_ind as usize).unwrap().to_owned(), sr::time_to_ms(time_string));
            track_arr_ind+=1;
            flap = false;
        } else {
            times_3lap_map.insert(track_arr.get(track_arr_ind as usize).unwrap().to_owned(), sr::time_to_ms(time_string));
            flap = true;
        }
        skip = true;
    }

    let mut split_rows_sc = sc_body.split("tr");
    split_rows_sc.nth(2);
    for row in split_rows_sc {
        if skip {
            skip = false;
            continue;
        }
        if row.contains("Totals") {
            break;
        }
        if row.contains("NT") {
            if flap {
                track_arr_ind+=1;
                flap = false;
            } else {
                flap = true;
            }
        }

        let time_split;
        if flap {
            time_split = row.split("</a").nth(0).unwrap();
        } else {
            time_split = row.split("</a").nth(1).unwrap();
        }
        let time_string = time_split.split('>').last().unwrap().replace('\'',":").replace('"',".");
        let track_name = track_arr.get(track_arr_ind as usize).unwrap().to_owned();
        let time = sr::time_to_ms(time_string);
        

        if flap {
            match times_flap_map.get(track_name.split('_').next().unwrap()) {
                Some(ng_time) => if ng_time < &time { times_flap_map.insert(track_name, time); },
                None => { times_flap_map.insert(track_name, time); }
            }
            track_arr_ind+=1;
            flap = false;
        } else {
            match times_3lap_map.get(track_name.split('_').next().unwrap()) {
                Some(ng_time) => if ng_time < &time { times_3lap_map.insert(track_name, time); },
                None => { times_3lap_map.insert(track_name, time); }
            }
            flap = true;
        }
        skip = true;
    }

    return [times_3lap_map, times_flap_map];
}

async fn mkwpp_mode(mkwpp_id: String, chadsoft_id: String, chadsoft_track_hash: HashMap<String,String>, mkwpp_track_arr: Vec<String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { grab_times_ctgp(chadsoft_id, chadsoft_track_hash).await });
    if mkwpp_id.is_empty() {
        println!("{} {}","You must link your MKWPP profile with".red(),"` cfg mkwpp <mkwpp-url> `".red().bold());
        exit = true;
    }
    let mkwpp_thread = std::thread::spawn( move || async { grab_times_mkwpp(mkwpp_id, mkwpp_track_arr).await });
    if exit {
        return;
    }

    print!("\nChadsoft PBs and Data");
    while !chadsoft_times_thread.is_finished() {
        terminal::loading();
    }
    println!("\t[{}]","✔".green());
    let ctgp_pbs = chadsoft_times_thread.join().unwrap().await;
    print!("MKW Players' Page PBs and Data");
    terminal::flush_stdout();
    while !mkwpp_thread.is_finished() {
        terminal::loading();
    }
    println!("\t[{}]\n","✔".green());
    terminal::flush_stdout();
    let mkwpp_pbs = mkwpp_thread.join().unwrap().await;

}

async fn grab_times_mkl(mkl_id: String) {
    let url = format!("https://www.mkleaderboards.com/mkw/players/{}",mkl_id);
    let web_page = reqwest::get(&url).await.unwrap().text().await.unwrap();
    if web_page.len() < 20 || web_page.contains("502 Bad Gateway") {
        panic!("MKLeaderboards is down. Try again later.");
    }
    println!("{web_page}");
    /*
    let mut bodies = web_page.split("tbody");
    let nosc_table = bodies.nth(1).unwrap();
    let glitch_table = bodies.nth(1).unwrap();
    let altsc_table = bodies.nth(1).unwrap();
    std::mem::drop(bodies);
    println!("NOSC Table:\n\n{nosc_table}");
    println!("GLITCH Table:\n\n{glitch_table}");
    println!("ALTSC Table:\n\n{altsc_table}");
    */
}

async fn mkl_mode(mkl_id: String, chadsoft_id: String, chadsoft_track_hash: HashMap<String,String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { grab_times_ctgp(chadsoft_id, chadsoft_track_hash).await });
    if mkl_id.is_empty() {
        println!("{} {}","You must link your MKL profile with".red(),"` cfg mkl <mkl-url> `".red().bold());
        exit = true;
    }
    let url = format!("https://www.mkleaderboards.com/mkw/players/{}",mkl_id);
    let mkl_times_thread = std::thread::spawn( move || async { grab_times_mkl(mkl_id).await });
    if exit {
        return;
    }
    /*
    println!("\nTo make this work, you have to trust this program with sensitive information.");
    println!("To submit to MKL, you need to give it your CSRF Token, which is basically your");
    println!("session Token for MKL. The program will not save this even locally.");
    println!("If you don't trust it, I invite you to check the source code, or just don't use it.");
    println!("Do you wanna continue? Y for yes, N for no.\n");
    print!(">> ");
    terminal::flush_stdout();
    read_str!(input);
    if !input.to_lowercase().starts_with("y") {
        return;
    }
    println!("\nTo find it, head to your profile page at {} while",url);
    println!("logged in, then: Open the page inspector with {} on Chromium based browsers","`CTRL+SHIFT+J`".bold());
    println!("or {} on Firefox","`CTRL+SHIFT+I`".bold());
    println!("\nIn the inspector you should see the text: {}","`<head>[•••]</head>`".bold());
    println!("If you can't see it, scroll to the top of the window.");
    println!("Click on it.\n");
    println!("Of the new lines that opened, the second line should be:");
    println!("{}{}{}\n","<meta name=\"csrf-token\" content=\"".bold(),"your-csrf-token-here".bold().red(),"\">".bold());
    println!("Copy it and paste it here.\n");
    print!(">> ");
    terminal::flush_stdout();

    read_str!(csrf_token);

    print!("\nChadsoft PBs and Data");
    while !chadsoft_times_thread.is_finished() {
        terminal::loading();
    }
    print!("\t\t\t[{}]","✔".green());
    let cdpbs = chadsoft_times_thread.join().unwrap().await.get(0).unwrap();
    print!("\nMKLeaderboards PBs and Data");
    terminal::flush_stdout();
    while !mkl_times_thread.is_finished() {
        terminal::loading();
    }
    print!("\t\t[{}]\n\n","✔".green());
    terminal::flush_stdout();*/
    mkl_times_thread.join().unwrap().await;
}