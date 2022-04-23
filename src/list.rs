pub mod list_func {
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	#[derive(Debug)]
	struct Type {
		id: i64,
		text: String,
		length: i64
	}

	#[derive(Debug)]
	struct Note {
		id: i64,
		text: String,
		status: String,
		list: String
	}

	fn create_list(s: &str) -> Result<()> {
		let conn = Connection::open("base.db3")?;
		conn.execute("CREATE TABLE IF NOT EXISTS list (
			id  		INTEGER PRIMARY KEY,
			text  	TEXT NOT NULL,
			length 	INTEGER NOT NULL)", NO_PARAMS,
		)?;

		let mut list = conn.prepare("SELECT id, text, length FROM list")?;
		let data = list.query_map([], |row| {
			Ok(Type {
				id: row.get(0)?,
				text: row.get(1)?,
				length: row.get(2)?,
			})
		})?;

		let mut id_int: i64 = 1;
		let mut ifhave: bool = false;
		let mut v = Vec::new();

		for i in data {v.push(i.unwrap());}

		for i in &v {
			if s == i.text {
				ifhave = true;
				break; } else { ifhave = false; }
		}

		if ifhave == false {
			v.push(Type {id: 0, text: s.to_string(), length: 0,});

			for i in 0..v.len() {
				v[i].id = id_int;
				id_int += 1;
			}

			conn.execute("DELETE FROM list", [])?;

			for i in v {conn.execute("INSERT INTO list (id, text, length) values (?1, ?2, ?3)", params![i.id, i.text, i.length],)?;}

			println!("Category created !");

		} else {println!("This category already exists");}

		Ok(())
	}

	pub fn new_list() {
		println!("Write new list name (work chores) -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let new_rsp = resp[0..resp.len() - 2].to_string();

		create_list(&new_rsp);
	}


	pub fn delete_list() -> Result<()> {

		fn delete_notes(rsp: &str) -> Result<()> {
			let conn = Connection::open("base.db3")?;
			let mut list = conn.prepare("SELECT id, text, status, list FROM base")?;
			let data = list.query_map([], |row| {
				Ok(Note {
					id: row.get(0)?,
					text: row.get(1)?,
					status: row.get(2)?,
					list: row.get(3)?,
				})
			})?;

			let mut ifhave: bool = false;
			let mut id_int: i64 = 1;
			let mut base = Vec::new();

			for i in data {
				if i.as_ref().unwrap().list == rsp { ifhave = true; }
				else { base.push(i.unwrap()); }
			}

			if ifhave == true {
				for i in 0..base.len() {
					base[i].id = id_int;
					id_int += 1;
				}
			}

			conn.execute("DELETE FROM base", [])?;
			for i in &base {conn.execute("INSERT INTO base (id, text, status, list) values (?1, ?2, ?3, ?4)", params![i.id, i.text, i.status, i.list],)?;}

			Ok(())
		}

		println!("Which list do you want to delete ? -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();

		let conn = Connection::open("base.db3")?;
		let mut list = conn.prepare("SELECT id, text, length FROM list")?;
		let data = list.query_map([], |row| {
			Ok(Type {
				id: row.get(0)?,
				text: row.get(1)?,
				length: row.get(2)?,
			})
		})?;

		let mut ifhave: bool = false;
		let mut id_int: i64 = 1;
		let mut base = Vec::new();

		for i in data {
			if i.as_ref().unwrap().text == rsp { ifhave = true; }
			else { base.push(i.unwrap()); }
		}

		if ifhave == true {
			for i in 0..base.len() {
				base[i].id = id_int;
				id_int += 1;
			}

			delete_notes(&rsp);

			let con = Connection::open("base.db3")?;
			con.execute("DELETE FROM list", [])?;
			for i in &base { con.execute("INSERT INTO list (id, text, length) values (?1, ?2, ?3)", params![i.id, i.text, i.length],)?; }

			println!("List deleted !");
		} else { println!("Element not found !"); }

		Ok(())
	}
}