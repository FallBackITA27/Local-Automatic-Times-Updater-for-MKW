// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, io::Write, str::FromStr};
use colored::Colorize;
use serde_json::Value;

mod jsondata;
mod files;
mod sr;

#[allow(clippy::needless_return)]
#[deny(clippy::needless_borrow)]

#[tauri::command]
async fn check_for_update() -> u8 {
    let req = reqwest::get("https://api.github.com/repos/FallBackITA27/Local-Automatic-Times-Updater-for-MKW/releases").await.unwrap().text().await.unwrap();
    let x: Vec<Value> = serde_json::from_str(&req).unwrap();
    if x.len() == 1 {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        files::read_config,
        save_chadsoft_user,
        save_mkl_user,
        save_mkwpp_user,
        mkwpp_mode
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn save_chadsoft_user(data: String) -> String {
    if !data.contains("chadsoft.co.uk") || !data.contains("/time-trials/players/") {
        return String::from_str("Error! No Chadsoft url found.").unwrap();
    }
    files::write_config(String::from_str("CHADSOFTUSER").unwrap(), data.split(".html").next().unwrap().split("/players/").last().unwrap().to_string().to_uppercase());
    return String::from_str("ok").unwrap();
}

#[tauri::command]
fn save_mkwpp_user(data: String) -> String {
    if !data.contains("mariokart64.com") || !data.contains("profile.php?pid=") {
        return String::from_str("Error! No MKWPP url found.").unwrap();
    }
    files::write_config(String::from_str("MKWPPUSER").unwrap(), data.split("profile.php?pid=").last().unwrap().to_string());
    return String::from_str("ok").unwrap();
}

#[tauri::command]
fn save_mkl_user(data: String) -> String {
    if !data.contains("www.mkleaderboards.com/mkw/players/") {
        return String::from_str("Error! No MKL url found.").unwrap();
    }
    files::write_config(String::from_str("MKLUSER").unwrap(), data.split("/players/").last().unwrap().to_string());
    return String::from_str("ok").unwrap();
}

/*
    "mkl" => mkl_mode(user.mkl_id.clone(),user.chadsoft_id.clone(),tracks_chadsoft_hash.clone()).await,
*/

#[tauri::command]
async fn mkwpp_mode(mkwpp_id: String, chadsoft_id: String) -> Vec<String> {
    let mut popups: Vec<String> = vec![];
    let mut exit = false;
    if chadsoft_id.is_empty() {
        popups.push(String::from_str("You must link your Chadsoft account").unwrap());
        exit = true;
    }
    // Don't you dare remove this line.
    if chadsoft_id == String::from_str("F6/8757434F0AA9F0").unwrap() {
        popups.push(String::from_str("Sorry, but this program isn't meant for homophobes or transphobes. Glad you know who I am now :) --FalB").unwrap());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { sr::grab_times_ctgp(chadsoft_id).await });
    if mkwpp_id.is_empty() {
        popups.push(String::from_str("You must link your MKWPP profile").unwrap());
        exit = true;
    }
    if exit {
        return popups;
    }

    let mkwpp_for_name = mkwpp_id.clone();
    let mkwpp_thread = std::thread::spawn( move || async { sr::grab_times_mkwpp(mkwpp_id).await });
    let mkwpp_grab_name = std::thread::spawn( move || async { sr::grab_name_mkwpp(mkwpp_for_name).await });
    let mkwpp_combined_track_arr_thread = std::thread::spawn(jsondata::grab_mkwpp_combined_tracks_array); 

    let ctgp_pbs = match chadsoft_times_thread.join().unwrap().await {
        Ok(pbs) => pbs,
        Err(error) => {
            popups.push(error);
            return popups;
        }
    };
    let mkwpp_pbs = mkwpp_thread.join().unwrap().await;

    let ctgp_3lap = ctgp_pbs.get(0).unwrap().to_owned();
    let ctgp_flap = ctgp_pbs.get(1).unwrap().to_owned();
    let mkwpp_3lap = mkwpp_pbs.get(0).unwrap().to_owned();
    let mkwpp_flap = mkwpp_pbs.get(1).unwrap().to_owned();
    
    let final_3lap_pbs_thread = std::thread::spawn( move || { sr::compare_ctgp_mkwpp(ctgp_3lap, mkwpp_3lap) });
    let final_flap_pbs_thread = std::thread::spawn( move || { sr::compare_ctgp_mkwpp(ctgp_flap, mkwpp_flap) });

    let final_3lap_pbs = final_3lap_pbs_thread.join().unwrap();

    let mut empty_3lap = false;

    if final_3lap_pbs.is_empty() {
        popups.push(String::from_str("You have no unsubmitted 3lap PBs").unwrap());
        empty_3lap = true;
    }

    let final_flap_pbs = final_flap_pbs_thread.join().unwrap();

    if final_flap_pbs.is_empty() {
        popups.push(String::from_str("You have no unsubmitted Flap PBs").unwrap());
        if empty_3lap {
            popups.push(String::from_str("Exited early because there are no new PBs").unwrap());
            return popups;
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

    let mkwpp_combined_track_arr = mkwpp_combined_track_arr_thread.join().unwrap().await;
    let mkwpp_name = mkwpp_grab_name.join().unwrap().await;

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
    let mut output_file = std::fs::File::create("./output.txt").unwrap();
    output_file.write_all(output.as_bytes()).unwrap();
    popups.push(String::from_str("Updated! The output has been written to ./output.txt").unwrap());
    return popups;
}

async fn mkl_mode(mkl_id: String, chadsoft_id: String, chadsoft_track_hash: HashMap<String,String>) {
    let mut exit = false;
    if chadsoft_id.is_empty() {
        println!("{} {}","You must link your Chadsoft account with".red(),"` cfg chadsoft <chadsoft-url> `".red().bold());
        exit = true;
    }
    let chadsoft_times_thread = std::thread::spawn( move || async { sr::grab_times_ctgp(chadsoft_id).await });
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