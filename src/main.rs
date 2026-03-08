use colored::Colorize;
use std::env;
use std::fs;
use gethostname::gethostname;
use std::io::Write;
use std::io;
use rand::RngExt;

extern crate uptime_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _jokes_enabled: bool = false;
    // jokes array
    let dumb_jokes = [
        "brain just segfaulted",
        "Bailing out, you are on your own. Good luck",
        "try fastfetch instead",
        "i use lightfetch btw",
        "don't mind using neofetch, it's outdated",
        "The more if, then else elif"
    ];
    let mut rng = rand::rng();

    if args.len() > 1 {
        for i in 1..=args.len()-1 {
            if args[i] == "-h" || args[i] == "--help" {
                show_help();
                return;
            }

            else if args[i] == "-j" || args[i] == "--jokes" {
                _jokes_enabled = true;
            }
        }
    }
    let host = gethostname();
    let ver = fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();

    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let host = host.to_str().unwrap_or("unknown");
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let uptime = uptime_lib::get().expect("REASON").as_secs_f64();
    let hours: i32 = uptime as i32 / 3600;
    let remaining: i32 = uptime as i32 % 3600;
    let minutes: i32 = remaining as i32 / 60;
    let seconds: i32 = remaining as i32 % 60;
    let formatted_uptime = format!("{}h{}m{}s", hours, minutes, seconds);
    let terminal = env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let de_or_wm = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "unknown".to_string());

    println!("{}", format!("{user}@{host}").cyan().bold());
        
    for i in 1..=20 {
        print!("-");
        if i == 20 {
            print!("\n");
        }
        }

    io::stdout().flush().unwrap();

    println!("{}",
        format!("{} {}", "Distro/OS:".blue(), get_pretty_name().green()));
    println!("{}",
        format!("{} {}", "Kernel:".blue(), ver.green()));
    println!("{}",
        format!("{} {}", "Shell:".blue(), shell.green()));
    println!("{}",
        format!("{} {}", "Uptime:".blue(), formatted_uptime.green()));
    println!("{}",
        format!("{} {}", "Terminal:".blue(), terminal.green()));
    println!("{}",
        format!("{} {}", "DE/WM:".blue(), de_or_wm.green()));

    if _jokes_enabled {
        println!("{}", dumb_jokes[rng.random_range(0..dumb_jokes.len())]
            .yellow());
    }
}

// show help message
fn show_help() {
    println!("Usage: lightfetch [options]\n");
    println!("Options:");
    println!("  -h, --help  Show this message and exit.");
    println!("  -j, --joke  Shows a joke at the last line of system info.");
}

// get pretty name of distribution
fn get_pretty_name() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                return val.trim_matches(&['"', '\''][..]).to_string();
            }
        }
    }
    "Linux".to_string()
}
