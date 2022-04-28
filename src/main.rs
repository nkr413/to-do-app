#![allow(warnings)]

// PACKAGES
//extern crate chrono;
//use chrono::Utc;

extern crate rusqlite;
use rusqlite::{params, Connection, Result, NO_PARAMS};

// MODULES
mod print;
pub use crate::print::print_func;

mod list;
pub use crate::list::list_func;

mod task;
pub use crate::task::task_func;

mod help;
pub use crate::help::help_func;


fn input() {
	println!("Enter -> ");

	let mut resp = String::new();
	std::io::stdin()
		.read_line(&mut resp)
		.expect("Failes");

	let rsp = &resp[0..&resp.len() - 2].to_string();

	if rsp == "/add" { task_func::add_task(); }

	if rsp == "/del" { task_func::delete_note(); }

	else if rsp == "/new-list" { list_func::new_list(); }

	else if rsp == "/del-list" { list_func::delete_list(); }

	else if rsp == "/print-all" { print_func::print_all(); }

	else if rsp == "/print" { print_func::print_one(); }

	else if rsp == "/clear" { clearscreen::clear().unwrap(); }
	else if rsp == "/cls" { clearscreen::clear().unwrap(); }

	else if rsp == "/help" { help_func::commands_list(); }

	else if rsp == "/exit" {
		println!("Exit !");
		return;
	}

	else { println!("none"); }

	input();
}

fn open_db() -> Result<()> {
	let conn = Connection::open("base.db3")?;

	conn.execute("CREATE TABLE IF NOT EXISTS base (
		id  		INTEGER 	PRIMARY KEY,
		text  	TEXT 			NOT NULL,
		status 	TEXT 			NOT NULL,
		list    TEXT 			NOT NULL)", NO_PARAMS,
	)?;

	conn.execute("CREATE TABLE IF NOT EXISTS list (
		id  		INTEGER		PRIMARY KEY,
		text  	TEXT 			NOT NULL,
		length 	INTEGER 	NOT NULL)", NO_PARAMS,
	)?;

	Ok(())
}

fn main() {
	open_db();
	input();
}
