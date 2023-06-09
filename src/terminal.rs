use colored::Colorize;
use std::io::Write;

fn clear() {
    println!("\x1b[2J\x1b[H");
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

fn slow_print(string: String) {
	for character in string.chars() {
		print!("{character}");
		flush_stdout();
		std::thread::sleep(std::time::Duration::new(0,10000000));
	}
}

pub fn welcome_text() {
    clear();
	slow_print("\n\n-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n".to_string());
	slow_print("|\tWelcome to the Automatic Times Updater for Mario Kart Wii!\t\t|\n".to_string());
	slow_print(format!("|\tWrite {} to start if you don't know what you're doing.\t\t|\n","` help `".bold()));
	slow_print(format!("|\tWrite {} or {} to exit the program.\t\t\t\t|\n","` q `".bold(),"` quit `".bold()));
    slow_print("|\t\t\t\t\t\t\t\t\t|\n".to_string());
	slow_print(format!("|\t{} {}{}\t\t\t\t\t\t|\n","Written by".purple(),"FalB".purple().bold().on_bright_magenta(),".".purple()));
	slow_print("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n\n".to_string());
}

pub fn help_command() {
	let empty_line = "|\t\t\t\t\t\t\t\t\t|";
	let separator_line = "|-------------------------------------------------------------------------------|";
	println!("\n=========== GENERIC COMMANDS ====================================================");
	println!("| {}\t\t\t\tDisplays this screen\t\t\t|","help".bold());
	println!("| {}\t\t\t\tExits the program\t\t\t|","quit".bold());
	println!("| {}\t\t\t\tAlias of {}\t\t\t|","q".bold(),"` quit `".bold());
	println!("| {} <cfg-option>\t\tModifies the config file\t\t|","cfg".bold());
	println!("| {} <run-option>\t\tStarts the Automatic Updater\t\t|","run".bold());
	println!("{empty_line}");
	println!("===========   CFG OPTIONS    ====================================================");
	println!("| All commands in this tab are preceded by {}\t\t\t\t\t|", "cfg".bold());
	println!("{separator_line}");
	println!("| {} <chadsoft-url>\tSets up your CTGP profile\t\t|","chadsoft".bold());
	println!("| {} <mkwpp-url>\t\tSets up your MKWPP profile\t\t|","mkwpp".bold());
	println!("| {}\t\t\tReloads the config\t\t\t|","reload".bold());
	println!("{empty_line}");
	println!("===========   RUN OPTIONS    ====================================================");
	println!("| All commands in this tab are preceded by {}\t\t\t\t\t|", "run".bold());
	println!("{separator_line}");
	println!("| {} <mode>\t\t\tRuns the updater for the MKWPP. The\t|","mkwpp".bold());
	println!("| \t\t\t\tmode can be either flap, 3lap, or not\t|");
	println!("| \t\t\t\tbe set for both. {}\t\t\t|","(WIP)".red());
	println!("{empty_line}");
	println!("=================================================================================");
}

pub fn quit() {
    clear();
    println!("{}","bye bye!".green());
}

