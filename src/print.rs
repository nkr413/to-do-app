pub mod print_func {	
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

	pub fn print_all() -> Result<()> {
		println!("all-data");
		let conn = Connection::open("base.db3")?;
		let mut list = conn.prepare("SELECT id, text, length FROM list")?;
		let data = list.query_map([], |row| {
			Ok(Type {
				id: row.get(0)?,
				text: row.get(1)?,
				length: row.get(2)?,
			})
		})?;

		let mut v = Vec::new();
		for i in data {v.push(i.unwrap());}

		println!("\n-- All Data --\n");
		for i in &v {println!("id: {:?} - {:?} ({:?})", i.id, i.text, i.length);}
		println!("\n--------------\n");

		Ok(())
	}

	pub fn print_one() -> Result<()> {
		pub fn get_list() -> Result<()> {
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

			Ok(())
		}

		println!("Which list do you want to see ? -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();

		Ok(())
	}
}