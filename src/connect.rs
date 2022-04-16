pub mod conn {
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	#[derive(Debug)]
	struct Type {
		id: i64,
		text: String
	}

	#[derive(Debug)]
	struct Note {
		id: i64,
		text: String,
		status: String,
		list: String
	}

	pub fn get_list_db() -> Result<()> {
		let conn = Connection::open("base.db3")?;
		let mut list = conn.prepare("SELECT id, text FROM lists")?;
		let data = list.query_map([], |row| {
			Ok(Type {
				id: row.get(0)?,
				text: row.get(1)?,
			})
		})?;

		let mut v = Vec::new();
		for i in data {v.push(i.unwrap());}

		println!("{:?}", v);
		Ok(())
	}

	pub fn get_note_db() -> Result<()> {
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
}