use std::{string, fmt::format, collections::HashMap};

pub fn time_to_ms(time: String) -> i32 {
    let mut colon_str_split = time.split(':');
    let mins_str = colon_str_split.next();
    if mins_str.is_none() {
        return -1
    }
    let seconds_and_ms_str = colon_str_split.last();
    if seconds_and_ms_str.is_none() {
        return -1
    }
    let mut dot_split = seconds_and_ms_str.unwrap().split('.');
    let sec_str = dot_split.next();
    if sec_str.is_none() {
        return -1
    }
    let ms_str = dot_split.last();
    if ms_str.is_none() {
        return -1
    }
    let mins: u32 = mins_str.unwrap().parse().unwrap();
    let sec: u32 = sec_str.unwrap().parse().unwrap();
    let ms: u32 = ms_str.unwrap().parse().unwrap();
    return (mins * 60000 + sec * 1000 + ms).try_into().unwrap();
}

pub fn ms_to_time(mut ms: i32) -> String {
    let mins = ms / 60000;
    ms %= 60000;
    let sec = ms / 1000;
    ms %= 1000;
    if mins > 0 {
        return format!("{}:{:02}.{:03}",mins,sec,ms);
    } else {
        return format!("{}.{:03}",sec,ms);
    }
}


fn filter_ctgp_hashmap(mut ctgp_hashmap: HashMap<String,(i32,String,String,String)>) -> HashMap<String,(i32,String,String,String)> {
    for (track_name, (time,_,_,_)) in ctgp_hashmap.clone() {
        if !track_name.contains('_') { continue }
        if track_name.contains("_sc") {
            match ctgp_hashmap.get(track_name.split('_').next().unwrap()) {
                Some((other_time,_,_,_)) => if time > *other_time {
                    ctgp_hashmap.remove(&track_name);
                },
                None => continue
            }
        } else if track_name.contains("_g") {
            if let Some((other_time,_,_,_)) = ctgp_hashmap.get(track_name.split('_').next().unwrap()) {
                if time > *other_time {
                    ctgp_hashmap.remove(&track_name);
                    continue;
                }
            }
            if let Some((other_time,_,_,_)) = ctgp_hashmap.get(&track_name.replace("_g","_sc")) {
                if time > *other_time {
                    ctgp_hashmap.remove(&track_name);
                    continue;
                }
            }
        }
    }
    return ctgp_hashmap;
}

pub async fn grab_times_ctgp(chadsoft_id: String, track_hash: HashMap<String,String>) -> [HashMap<String,(i32,String,String,String)>; 2] {
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
        let ghost_time_3lap = time_to_ms(time_string_3lap.clone());
        let time_string_flap = ghost["bestSplitSimple"].as_str().unwrap().to_string();
        let ghost_time_flap = time_to_ms(time_string_flap.clone());
        let ghost_link = ghost["_links"]["item"]["href"].as_str().unwrap().replace("json", "html").to_string();
        let date = ghost["dateSet"].as_str().unwrap().split('T').next().unwrap().to_string();
        match times_3lap_map.get(track_name) {
            Some((time,_,_,_)) => if time > &ghost_time_3lap {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, time_string_3lap, date.clone(), ghost_link.clone()));
            },
            None => {
                times_3lap_map.insert(track_name.to_owned().clone(), (ghost_time_3lap, time_string_3lap, date.clone(), ghost_link.clone()));
            }
        };
        match times_flap_map.get(track_name) {
            Some((time,_,_,_)) => if time > &ghost_time_flap {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, time_string_flap, date, ghost_link));
            },
            None => {
                times_flap_map.insert(track_name.to_owned().clone(), (ghost_time_flap, time_string_flap, date, ghost_link));
            }
        };
    }

    return [
        std::thread::spawn(move || { filter_ctgp_hashmap(times_3lap_map) }).join().unwrap(),
        std::thread::spawn(move || { filter_ctgp_hashmap(times_flap_map) }).join().unwrap()
    ];
}

pub fn date_to_full_date(date: String) -> String {
    let month_vector = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]; 
    let mut x = date.split("-");
    let year = x.next().unwrap();
    let month = month_vector.get(x.next().unwrap().parse::<usize>().unwrap()-1).unwrap();
    let day = x.next().unwrap();
    if day.starts_with("0") {
        return format!("{} {}, {}", month, day.chars().last().unwrap(), year);
    }
    return format!("{} {}, {}", month, day, year);
}

pub fn get_correct_abbreviation_mkwpp(track: String, mkwpp_combined_track_arr:  Vec<String>) -> String {
    let mut out = track.split('_').next().unwrap().to_uppercase();
    if track.starts_with('r') && track != "rr".to_string() {
        out.replace_range(0..1, "r");
    };
    if mkwpp_combined_track_arr.contains(&track) {
        out += " nosc";
    }
    return out;
}

pub fn compare_ctgp_mkwpp(mut ctgp_hashmap: HashMap<String, (i32, String, String, String)>, mkwpp_hashmap: HashMap<String, i32>) -> HashMap<String,(String, i32)> {
    // TODO: I should make this a better macro eventually.
    macro_rules! remove_superfluous_category {
        ($track: expr) => {
            // if glitch time > sc time is already handled in the ctgp filter.
            if ctgp_hashmap.contains_key(concat!($track,"_sc")) && ctgp_hashmap.contains_key(concat!($track,"_g")) {
                ctgp_hashmap.remove(concat!($track,"_sc"));
            }
        };
    }
    remove_superfluous_category!("mg");
    remove_superfluous_category!("cm");
    remove_superfluous_category!("gv");
    remove_superfluous_category!("rbc");
    let mut pbs_hashmap: HashMap<String,(String, i32)> = HashMap::default();
    for (mut track_name, (time, _, date, _)) in ctgp_hashmap {
        let originally = track_name.clone();
        if track_name.contains("_g") {
            if let Some(mkwpp_comparison) = mkwpp_hashmap.get(&track_name) {
                if mkwpp_comparison > &time {
                    pbs_hashmap.insert(originally, (date.clone(), time));
                    continue;
                }
                continue;
            }
        }
        track_name = track_name.replace("_g","_sc");
        if track_name.contains("_sc") {
            if let Some(mkwpp_comparison) = mkwpp_hashmap.get(&track_name) {
                if mkwpp_comparison > &time {
                    pbs_hashmap.insert(originally, (date.clone(), time));
                    continue;
                }
                continue;
            }
        }
        track_name = track_name.split('_').nth(0).unwrap().to_string();
        match mkwpp_hashmap.get(&track_name) {
            Some(mkwpp_comparison) => if mkwpp_comparison > &time {
                pbs_hashmap.insert(originally, (date.clone(), time));
            },
            None => {
                pbs_hashmap.insert(originally, (date.clone(), time));
            }
        }
    }
    return pbs_hashmap;
}

pub async fn grab_name_mkwpp(mkwpp_id: String) -> String {
    let url = format!("https://www.mariokart64.com/mkw/profile.php?pid={}",mkwpp_id);
    let player_page_req = reqwest::get(&url);
    let player_page = player_page_req.await.unwrap().text().await.unwrap();
    let mut split = player_page.split("MKW Profile: ").last().unwrap();
    return "Name: ".to_string() + split.split("&nbsp;").next().unwrap();
}

pub async fn grab_times_mkwpp(mkwpp_id: String, track_arr: Vec<String>) -> [HashMap<String,i32>; 2] {
    let url = format!("https://www.mariokart64.com/mkw/profile.php?pid={}",mkwpp_id);
    let player_page_req = reqwest::get(&url);

    let mut skip = false;
    let mut flap = false;
    let mut track_arr_ind: u8 = 0;
    let mut times_3lap_map: HashMap<String,i32> = HashMap::default();
    let mut times_flap_map: HashMap<String,i32> = HashMap::default();

    let player_page = player_page_req.await.unwrap().text().await.unwrap();
    let mut split = player_page.split("table");
    let sc_body = split.nth(20).unwrap();

    let nosc_body = split.nth(3).unwrap();
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
            times_flap_map.insert(track_arr.get(track_arr_ind as usize).unwrap().to_owned(), time_to_ms(time_string));
            track_arr_ind+=1;
            flap = false;
        } else {
            times_3lap_map.insert(track_arr.get(track_arr_ind as usize).unwrap().to_owned(), time_to_ms(time_string));
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
        let time = time_to_ms(time_string);
        

        if flap {
            match times_flap_map.get(track_name.split('_').next().unwrap()) {
                Some(ng_time) => if ng_time > &time { times_flap_map.insert(track_name, time); },
                None => { times_flap_map.insert(track_name, time); }
            }
            track_arr_ind+=1;
            flap = false;
        } else {
            match times_3lap_map.get(track_name.split('_').next().unwrap()) {
                Some(ng_time) => if ng_time > &time { times_3lap_map.insert(track_name, time); },
                None => { times_3lap_map.insert(track_name, time); }
            }
            flap = true;
        }
        skip = true;
    }

    return [
        times_3lap_map,
        times_flap_map
    ];
}

pub async fn grab_times_mkl(mkl_id: String) {
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