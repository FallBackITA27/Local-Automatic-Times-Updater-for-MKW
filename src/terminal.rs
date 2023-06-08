use std::io::Write;

#[cfg(any(target_os = "linux",target_os="mac_os"))]
pub fn clear() {
    let x = std::process::Command::new("clear").output().unwrap().stdout;
    print!("{}",String::from_utf8(x).unwrap());
}

#[cfg(target_os = "windows")]
pub fn clear() {
    let x = std::process::Command::new("cls").output().unwrap().stdout;
    print!("{}",String::from_utf8(x).unwrap());
}

pub fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn print_loading_dot() {
    print!(".");
    flush_stdout();
    std::thread::sleep(std::time::Duration::new(1,0));
}

fn move_cursor_back() {
    const BACKSPACE: char = 8u8 as char;
    print!("{}",BACKSPACE);
}

fn delete_written(num: u16) {
    for i in 0..num {
        move_cursor_back();
        print!(" ");
        move_cursor_back();
    }
}

pub fn loading() {
    print_loading_dot();
    print_loading_dot();
    print_loading_dot();
    delete_written(3);
    flush_stdout();
}