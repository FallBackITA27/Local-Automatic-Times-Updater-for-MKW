/*
    NOTE: A lot of functions should be moved to sr.rs. I am not in the vein of doing it right now, I will probably one of these days.
*/

use core::panic;
use std::{collections::HashMap, io::Write};
use colored::Colorize;
use iced::Application;

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

struct IcedApp;

impl iced::Application for IcedApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (IcedApp, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("LOCMKWUPD")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "Test".into()
    }
}


#[tokio::main]
async fn main() {
    let tracks_chadsoft_hash_thread = std::thread::spawn(grab_chadsoft_tracks_hashmap);
    let tracks_chadsoft_arr_thread = std::thread::spawn(grab_chadsoft_tracks_array);
    let tracks_mkwpp_arr_thread = std::thread::spawn(grab_mkwpp_tracks_array);
    let tracks_mkwpp_arr_combined_thread = std::thread::spawn(grab_mkwpp_combined_tracks_array);

    IcedApp::run(iced::Settings::default()).unwrap();

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
    print!("\t[{}]\n| Chadsoft Track Array","√".green());
    terminal::flush_stdout();
    let tracks_chadsoft_hash = tracks_chadsoft_hash_thread.join().unwrap().await;
    let all_chadsoft_links = tracks_chadsoft_hash.clone().into_keys().collect::<Vec<String>>();

    while !tracks_chadsoft_arr_thread.is_finished() {
        terminal::loading();
    };
    print!("\t\t[{}]\n| MKWPP Track Array","√".green());
    terminal::flush_stdout();
    let tracks_chadsoft_arr = tracks_chadsoft_arr_thread.join().unwrap().await;

    while !tracks_mkwpp_arr_thread.is_finished() && !tracks_mkwpp_arr_combined_thread.is_finished() {
        terminal::loading();
    };
    print!("\t\t[{}]\n| User Data","√".green());
    terminal::flush_stdout();
    let tracks_mkwpp_arr = tracks_mkwpp_arr_thread.join().unwrap().await;
    let tracks_mkwpp_combined_arr = tracks_mkwpp_arr_combined_thread.join().unwrap().await;

    while !user_thread.is_finished() {
        terminal::loading();
    };
    let mut user = user_thread.join().unwrap();
    println!("\t\t\t[{}]","√".green());

    let command_list = ["q","quit","help","cfg","run"];
/*
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
                        println!("\t[{}]","√".green());
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
                        println!("\t[{}]","√".green());
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
                        println!("\t[{}]","√".green());
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
                        println!("\t[{}]","√".green());
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
                    "mkwpp" => mkwpp_mode(user.mkwpp_id.clone(),user.chadsoft_id.clone(),tracks_chadsoft_hash.clone(),tracks_mkwpp_arr.clone(), tracks_mkwpp_combined_arr.clone()).await,
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
    }*/
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

async fn grab_mkwpp_combined_tracks_array() -> Vec<String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/mkwpp_combined.json");
    let json: Vec<String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_chadsoft_tracks_hashmap() -> HashMap<String,String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/json/cd_track_mapping.json");
    let json: HashMap<String,String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn mkwpp_mode(mkwpp_id: String, chadsoft_id: String, chadsoft_track_hash: HashMap<String,String>, mkwpp_track_arr: Vec<String>, mkwpp_combined_track_arr:  Vec<String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { sr::grab_times_ctgp(chadsoft_id, chadsoft_track_hash).await });
    if mkwpp_id.is_empty() {
        println!("{} {}","You must link your MKWPP profile with".red(),"` cfg mkwpp <mkwpp-url> `".red().bold());
        exit = true;
    }
    let mkwpp_for_name = mkwpp_id.clone();
    let mkwpp_thread = std::thread::spawn( move || async { sr::grab_times_mkwpp(mkwpp_id, mkwpp_track_arr).await });
    let mkwpp_grab_name = std::thread::spawn( move || async { sr::grab_name_mkwpp(mkwpp_for_name).await });
    if exit {
        return;
    }

    print!("\nChadsoft PBs and Data");
    while !chadsoft_times_thread.is_finished() {
        terminal::loading();
    }
    terminal::flush_stdout();
    println!("\t[{}]","√".green());
    let ctgp_pbs = chadsoft_times_thread.join().unwrap().await;
    print!("MKW Players' Page PBs and Data");
    terminal::flush_stdout();
    while !mkwpp_thread.is_finished() && !mkwpp_grab_name.is_finished() {
        terminal::loading();
    }
    println!("\t[{}]\n","√".green());
    terminal::flush_stdout();
    let mkwpp_pbs = mkwpp_thread.join().unwrap().await;
    let mkwpp_name = mkwpp_grab_name.join().unwrap().await;

    let ctgp_3lap = ctgp_pbs.get(0).unwrap().to_owned();
    let ctgp_flap = ctgp_pbs.get(1).unwrap().to_owned();
    let mkwpp_3lap = mkwpp_pbs.get(0).unwrap().to_owned();
    let mkwpp_flap = mkwpp_pbs.get(1).unwrap().to_owned();
    
    let final_3lap_pbs_thread = std::thread::spawn( move || { sr::compare_ctgp_mkwpp(ctgp_3lap, mkwpp_3lap) });
    let final_flap_pbs_thread = std::thread::spawn( move || { sr::compare_ctgp_mkwpp(ctgp_flap, mkwpp_flap) });

    print!("Comparing 3lap Times");
    terminal::flush_stdout();
    while !final_3lap_pbs_thread.is_finished() {
        terminal::loading();
    }
    print!("\t\t[{}]\nComparing Flap Times","√".green());
    terminal::flush_stdout();
    let final_3lap_pbs = final_3lap_pbs_thread.join().unwrap();
    let mut empty_3lap = false;
    if final_3lap_pbs.is_empty() {
        println!("You have no unsubmitted 3lap PBs");
        empty_3lap = true;
    }
    while !final_flap_pbs_thread.is_finished() {
        terminal::loading();
    }
    println!("\t\t[{}]","√".green());
    let final_flap_pbs = final_flap_pbs_thread.join().unwrap();
    if final_flap_pbs.is_empty() {
        println!("You have no unsubmitted Flap PBs");
        if empty_3lap {
            println!("Stopping");
            return;
        }
    }
    let mut final_pbs: Vec<(bool, String, String, i32)> = vec![];
    for (track_name, (date, time)) in final_3lap_pbs {
        final_pbs.push((false,track_name,date,time));
    }
    for (track_name, (date, time)) in final_flap_pbs {
        final_pbs.push((true,track_name,date,time));
    }
    final_pbs.sort_by(|a,b| a.0.cmp(&b.0));
    final_pbs.sort_by(|a,b| a.1.cmp(&b.1));
    final_pbs.sort_by(|a,b| a.2.cmp(&b.2));
    let mut output = String::new();
    let mut last_date = String::new();
    let mut last_track_name = String::new();
    for (is_flap, track_name, date, time) in final_pbs {
        if last_date != date {
            if is_flap {
                output += &format!("\n\n\nDate: {}\n{}\n\n{} flap {}",
                    sr::date_to_full_date(date.clone()),
                    mkwpp_name,
                    sr::get_correct_abbreviation_mkwpp(track_name.clone(),mkwpp_combined_track_arr.clone()),
                    sr::ms_to_time(time)
                );
            } else {
                output += &format!("\n\n\nDate: {}\n{}\n\n{} {}",
                    sr::date_to_full_date(date.clone()),
                    mkwpp_name,
                    sr::get_correct_abbreviation_mkwpp(track_name.clone(),mkwpp_combined_track_arr.clone()),
                    sr::ms_to_time(time)
                );
            }
        } else {
            if is_flap {
                if track_name == last_track_name {
                    output += &format!(" / {}",sr::ms_to_time(time));
                } else {
                    output += &format!("\n{} flap {}",sr::get_correct_abbreviation_mkwpp(track_name.clone(),mkwpp_combined_track_arr.clone()),sr::ms_to_time(time));
                }
            } else {
                output += &format!("\n{} {}",sr::get_correct_abbreviation_mkwpp(track_name.clone(),mkwpp_combined_track_arr.clone()),sr::ms_to_time(time));
            }
        }
        last_track_name = track_name;
        last_date = date;
    }
    println!("The output has been written to ./output.txt");
    let mut output_file = std::fs::File::create("./output.txt").unwrap();
    output_file.write_all(output.as_bytes()).unwrap();
}

async fn mkl_mode(mkl_id: String, chadsoft_id: String, chadsoft_track_hash: HashMap<String,String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { sr::grab_times_ctgp(chadsoft_id, chadsoft_track_hash).await });
    if mkl_id.is_empty() {
        println!("{} {}","You must link your MKL profile with".red(),"` cfg mkl <mkl-url> `".red().bold());
        exit = true;
    }
    let url = format!("https://www.mkleaderboards.com/mkw/players/{}",mkl_id);
    let mkl_times_thread = std::thread::spawn( move || async { sr::grab_times_mkl(mkl_id).await });
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
    print!("\t\t\t[{}]","√".green());
    let cdpbs = chadsoft_times_thread.join().unwrap().await.get(0).unwrap();
    print!("\nMKLeaderboards PBs and Data");
    terminal::flush_stdout();
    while !mkl_times_thread.is_finished() {
        terminal::loading();
    }
    print!("\t\t[{}]\n\n","√".green());
    terminal::flush_stdout();*/
    mkl_times_thread.join().unwrap().await;
}