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