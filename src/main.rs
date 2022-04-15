// PACKAGES
extern crate chrono;
use chrono::Utc;

extern crate rusqlite;
use rusqlite::{params, Connection, Result, NO_PARAMS};

// MODULES
mod print;
pub use crate::print::print_base;

mod list;
pub use crate::list::list_func;

mod task;
pub use crate::task::add;


fn input(rsp: &str) {
	if rsp == "/add" { add::add_task(); }

	else if rsp == "/new-list" { list_func::new_list(); }

	else if rsp == "/delete-list" { list_func::delete_list(); }

	else if rsp == "/print" { print_base::print_data(); }

	else if rsp == "/exit" { println!("Exit !"); }

	else { println!("none"); }
}

fn open_db() -> Result<()> {
	let conn = Connection::open("base.db3")?;

	conn.execute("CREATE TABLE IF NOT EXISTS base (
		id  		INTEGER PRIMARY KEY,
		text  	TEXT NOT NULL,
		status 	TEXT NOT NULL,
		list    TEXT NOT NULL)", NO_PARAMS,
	)?;

	conn.execute("CREATE TABLE IF NOT EXISTS lists (
		id  		INTEGER PRIMARY KEY,
		text  	TEXT NOT NULL)", NO_PARAMS,
	)?;

	Ok(())
}

fn main() {
	open_db();

	println!("Enter -> ");

	let mut resp = String::new();
	std::io::stdin()
		.read_line(&mut resp)
		.expect("Failes");

	input(&resp[0..&resp.len() - 2]);
}
