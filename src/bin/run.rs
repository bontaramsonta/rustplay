extern crate clap;

use clap::{App, Arg};
use std::io::{self, Write};
use std::process::{exit, Command};

fn check_cargo_watch_installed() {
    let output = Command::new("cargo")
        .args(&["install", "cargo-watch", "--list"])
        .output()
        .expect("Failed to check for cargo-watch installation");

    let installed_binaries = String::from_utf8_lossy(&output.stdout);

    if !installed_binaries.contains("cargo-watch") {
        println!("cargo-watch is not installed. Please install it with 'cargo install cargo-watch' first.");
        exit(1);
    }
}

fn main() {
    check_cargo_watch_installed();
    let matches = App::new("Cargo Watch Runner")
        .version("1.0")
        .author("bontaramsonta")
        .about("Runs a Cargo Watch command")
        .usage("cargo-watch-runner [OPTIONS] <filepath>")
        .arg(
            Arg::with_name("filepath")
                .help("Path to the main.rs file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("input_path")
                .help("Path to the input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("output_path")
                .help("Path to the output file")
                .takes_value(true),
        )
        .get_matches();

    let filepath = matches.value_of("filepath").unwrap();
    let input_path = matches.value_of("input").unwrap_or("in.txt");
    let output_path = matches.value_of("output").unwrap_or("out.txt");

    let command = format!("cargo-watch -w {filepath} -w {input_path} -cqs 'cargo -q run {filepath} < {input_path} > {output_path} 2>&1'");

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to run command");

    io::stdout().write_all(&output.stdout).unwrap();
}
