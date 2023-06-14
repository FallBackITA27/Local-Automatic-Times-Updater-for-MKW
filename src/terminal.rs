use colored::Colorize;
use std::io::Write;

#[cfg(target_os="linux")]
fn clear() {
    print!("{}",String::from_utf8(std::process::Command::new("clear").output().unwrap().stdout).unwrap());
}

#[cfg(target_os="windows")]
fn clear() { }

pub fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn print_loading_dot() {
    print!(".");
    flush_stdout();
    std::thread::sleep(std::time::Duration::new(1,0));
}

fn move_cursor_back(num: u8) {
    const BACKSPACE: char = 8u8 as char;
	for _ in 0..num {
		print!("{}",BACKSPACE);
	}
}

fn delete_written(num: u8) {
	move_cursor_back(num);
    for _ in 0..num {
        print!(" ");
    }
}

pub fn loading() {
    print_loading_dot();
    print_loading_dot();
    print_loading_dot();
    delete_written(3);
    flush_stdout();
}

fn slow_print(string: String) {
	for character in string.chars() {
		print!("{character}");
		flush_stdout();
		std::thread::sleep(std::time::Duration::new(0,10000000));
	}
}

pub fn welcome_text() {
    clear();
	slow_print("\n\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n".to_string());
	slow_print("||\tWelcome to the Automatic Times Updater for Mario Kart Wii!\t\t".to_string());
	move_cursor_back(1);
	slow_print(format!("||\n||\tWrite {} to start if you don't know what you're doing.\t\t","` help `".bold()));
	move_cursor_back(1);
	slow_print(format!("||\n||\tWrite {} or {} to exit the program.\t\t\t\t","` q `".bold(),"` quit `".bold()));
	move_cursor_back(1);
    slow_print("||\n||\t\t\t\t\t\t\t\t\t\t".to_string());
	move_cursor_back(1);
	slow_print(format!("||\n||\t{} {}{}\t\t\t\t\t\t\t","Written by".purple(),"FalB".purple().bold(),".".purple()));
	move_cursor_back(1);
	slow_print("||\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n\n".to_string());
	println!("TEST BUILD");
	println!("\u{001B}[31mTEST\u{001B}[0m ASSS");
	println!("\\033[31mTEST\\033[0m ASSS");
	println!("\033[31mTEST\033[0m ASSS");
}

pub fn help_command() {
	let empty_line = "|\t\t\t\t\t\t\t\t\t\t|";
	let separator_line = "|-------------------------------------------------------------------------------|";
	println!("\n=========== GENERIC COMMANDS ====================================================");
	println!("| {}\t\t\t\tDisplays this screen\t\t\t\t|","help".bold());
	println!("| {}\t\t\t\tExits the program\t\t\t\t|","quit".bold());
	println!("| {}\t\t\t\tAlias of {}\t\t\t\t|","q".bold(),"` quit `".bold());
	println!("| {} <cfg-option>\t\tModifies the config file\t\t\t|","cfg".bold());
	println!("| {} <run-option>\t\tStarts the Automatic Updater\t\t\t|","run".bold());
	println!("{empty_line}");
	println!("===========   CFG OPTIONS    ====================================================");
	println!("| All commands in this tab are preceded by {}\t\t\t\t\t|", "cfg".bold());
	println!("{separator_line}");
	println!("| {} <chadsoft-url>\tSets up your CTGP profile\t\t\t|","chadsoft".bold());
	println!("| {} <mkwpp-url>\t\tSets up your MKWPP profile\t\t\t|","mkwpp".bold());
	println!("| {} <mkl-url>\t\t\tSets up your MKL profile\t\t\t|","mkl".bold());
	println!("| {}\t\t\tReloads the config\t\t\t\t|","reload".bold());
	println!("{empty_line}");
	println!("===========   RUN OPTIONS    ====================================================");
	println!("| All commands in this tab are preceded by {}\t\t\t\t\t|", "run".bold());
	println!("{separator_line}");
	println!("| {} [mode]\t\t\tRuns the updater for the MKWPP. {}\t\t|","mkwpp".bold(),"(WIP)".red());
	println!("| {}\t\t\t\tRuns the updater for MKL.\t\t\t|","mkl".bold());
	println!("{empty_line}");
	println!("=================================================================================");
}

pub fn quit() {
    clear();
    println!("{}","bye bye!".green());
}

