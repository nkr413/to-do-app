// PACKAGES
extern crate chrono;
use chrono::Utc;

extern crate rusqlite;
use rusqlite::{params, Connection, Result, NO_PARAMS};

use std::collections::HashMap;
// PACKAGES

#[derive(Debug)]
struct Type {
	id: i64,
	text: String
}


fn input(rsp: &str) {

	if rsp == "/add" {

		fn add_task() -> Result<()> {
			let mut resp = String::new();
			std::io::stdin()
				.read_line(&mut resp)
				.expect("Failes");

			let new_rsp = resp[0..resp.len() - 2].to_string();

			let conn = Connection::open("base.db3")?;

			let mut list = conn.prepare("SELECT id, text FROM lists")?;
			let data = list.query_map([], |row| {
				Ok(Type {
					id: row.get(0)?,
					text: row.get(1)?,
				})
			})?;

			//let mut v = Vec::new();

			for i in data {
				if new_rsp == i.unwrap().text {
					println!("true");
				} else {
					println!("false");
				}
			}

			//println!("{:?}", v);

			Ok(())
		}

		println!("Which list do you want to add the task to ? -->");
		add_task();
	}


	else if rsp == "/new-list" {
		fn create_list(s: &str) -> Result<()> {
			let conn = Connection::open("base.db3")?;

			conn.execute("CREATE TABLE IF NOT EXISTS lists (
				id  		INTEGER PRIMARY KEY,
				text  	TEXT NOT NULL)", NO_PARAMS
			)?;

			let mut list = conn.prepare("SELECT id, text FROM lists")?;
			let data = list.query_map([], |row| {
				Ok(Type {
					id: row.get(0)?,
					text: row.get(1)?,
				})
			})?;

			let mut id_int: i64 = 1;
			let mut ifhave: bool = false;
			let mut v = Vec::new();

			for i in data {v.push(i.unwrap());}

			for i in &v {
				if s == i.text { ifhave = true; }
				else { ifhave = false; }
			}

			if ifhave == false {
				v.push(Type {id: 0, text: s.to_string(),});

				for i in 0..v.len() {
					v[i].id = id_int;
					id_int += 1;
				}

				conn.execute("DELETE FROM lists", [])?;

				for i in v {
					conn.execute("INSERT INTO lists (id, text) values (?1, ?2)", params![i.id, i.text],)?;
				}

			} else {println!("This category already exists");}

			Ok(())
		}

		fn new_list() {
			let mut resp = String::new();
			std::io::stdin()
				.read_line(&mut resp)
				.expect("Failes");

			let new_rsp = resp[0..resp.len() - 2].to_string();

			create_list(&new_rsp);
		}

		println!("Write new list name (work chores) -->");
		new_list();
	}


	else if rsp == "/print" {
		fn print_data() -> Result<()> {
			let conn = Connection::open("base.db3")?;

			let mut list = conn.prepare("SELECT id, text FROM lists")?;
			let data = list.query_map([], |row| {
				Ok(Type {
					id: row.get(0)?,
					text: row.get(1)?,
				})
			})?;

			for i in data {
				println!("{:?}", i);
			}

			Ok(())
		}

		print_data();
	}

	else if rsp == "/exit" {
		println!("Exit !");
	}

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
		text  	TEXT NOT NULL)", NO_PARAMS
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
