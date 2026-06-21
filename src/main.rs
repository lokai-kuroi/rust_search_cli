use clap::Parser;
use std::io::{self, IsTerminal};
use std::fs::{self};
use std::path::Path;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, long)]
    pattern: String,
    #[arg(long, default_value=".")]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    human_panic::setup_panic!();
    ctrlc::set_handler(move || {
        eprintln!("Shutting down the programm while something happend (ctrlc)");
        std::process::exit(130);
    }).expect("Error while setting the ctrlc handler");
    let args = CliArgs::parse();

    let mut to_check: Vec<String> = Vec::new();
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        let lines = io::stdin().lines();
        for line in lines {
            to_check.push(line?);
        }
    } else {
        for entry in fs::read_dir(args.path)? {
            let entry = entry?;
            let pathb = entry.path();
            let pathi = pathb.to_str().unwrap();
            to_check.push(pathi.to_string());
        }
    }
    
    while !to_check.is_empty() {
        let entry = &to_check.pop().unwrap();
        let entry_path = Path::new(&entry);
        if entry_path.is_dir() {
            for next_entry_line in fs::read_dir(&entry)? {
                let next_entry = next_entry_line?;
                let path_buffer = next_entry.path();
                let path_i = path_buffer.to_str().unwrap();
                to_check.push(path_i.to_string());
            }
        }
        if entry.contains(&args.pattern) {
            println!("{:#?}", entry);
        }

    }
    std::process::exit(exitcode::OK)
}
