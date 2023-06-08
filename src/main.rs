use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let tracks_hash_thread = std::thread::spawn(grab_tracks_hashmap);
    let tracks_arr_thread = std::thread::spawn(grab_tracks_array);
}

async fn grab_tracks_array() -> Vec<[String; 2]> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/cd_track_array.json");
    let json: Vec<[String; 2]> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}

async fn grab_tracks_hashmap() -> HashMap<String,String> {
    let json_string = reqwest::get("https://raw.githubusercontent.com/FallBackITA27/MKWPP-MKL-Local-Updater/main/cd_track_mapping.json");
    let json: HashMap<String,String> = serde_json::from_str(json_string.await.unwrap().text().await.unwrap().as_str()).unwrap();
    return json;
}