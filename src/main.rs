use std::{fs, io::{self, Read, Write}, path::Path};

use clap::Parser;

#[derive(Parser)]
#[command(about, version, author)]
struct LeetArgs {
    /// Source file path
    #[arg(short, long)]
    source: Option<String>,

    /// Destination file path
    #[arg(short, long)]
    destination: Option<String>,
}

fn main() {
    let args = LeetArgs::parse();

    let source = args.source
        .map(|s| Path::new(&s).to_owned());

    let destination = args.destination
        .map(|d| Path::new(&d).to_owned());

    let input = match source {
        Some(s) => {
            match fs::read_to_string(s.clone()) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to read from file: {:?}", s);
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
                
        },
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("leet -> No input given");
                std::process::exit(1);
            }

            let mut input = String::new();

            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to read from stdin");
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }

            input
        }
    };

    let output = input.chars()
        .map(|c| match c {
            '6' => '9',
            '9' => '6',
            _ => c,
        })
        .collect::<String>();

    match destination {
        Some(d) => {
            match fs::write(d.clone(), output) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to file: {:?}", d);
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        },
        None => {
            match io::stdout().write_all(output.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to write to stdout");
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
