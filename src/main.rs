// PACKAGES
extern crate chrono;
use chrono::Utc;

extern crate serde;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
// PACKAGES

fn input(rsp: &str) {

	if rsp == "/start" {
		println!("- Start");
	}

	else if rsp == "/print" {
		println!("- Print");
	}

	else if rsp == "/file" {
    let file = fs::File::open("./data.json").expect("file should open read only");

    let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");

    let first = json.get("users").expect("file should have FirstName key");

    println!("{:?}", first[0]);
	}

	else { println!("none"); }
}

fn main() {
  println!("Enter -> ");

	let mut resp = String::new();
	std::io::stdin()
		.read_line(&mut resp)
		.expect("Failes");

	input(&resp[0..&resp.len() - 2]);
}
