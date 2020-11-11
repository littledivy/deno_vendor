#![feature(box_patterns)]

mod analyze;
mod parser;

use clap::{App, Arg};
use std::fs;
use std::path::Path;
use url::Url;

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
    let vendor_path = Path::new("vendor");
    for dep in deps {
	    if let Some(url) = parse_url(dep.clone()) {
	    	let loc = vendor_path.join(url.host_str().unwrap().to_owned() + url.path());
	    	let src = fetch(dep.clone());
	    	fs::write(loc, src);
	    }   
    }
}

fn parse_url(dep: String) -> Option<Url> {
	 match Url::parse(&dep) {
	 	Err(_) => None,
	 	Ok(u) => Some(u),
	 }
}

fn prepare() {
	fs::create_dir_all("vendor").expect("Could not create vendor/ directory");
}

fn fetch(dep: String) -> String {
	 let resp = reqwest::blocking::get(&dep).expect("Cannot fetch dependency")
        .text().unwrap();
}