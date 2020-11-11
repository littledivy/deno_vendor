#![feature(box_patterns)]

mod analyze;
mod parser;

use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("denovendor")
        .version("1.0")
        .about("Vendor all your deno dependencies")
        .args(&[Arg::with_name("input")
            .help("Input source file")
            .required(true)])
        .get_matches();

    let file_name = matches.value_of("input").expect("Cannot read input file");
    let source = fs::read_to_string(&file_name).unwrap();
    let deps = analyze::analyze_dependencies(&file_name, &source);
    prepare();
    println!("{:?}", deps);
}

fn prepare() {
	fs::create_dir_all("vendor").expect("Could not create vendor/ directory");
}
