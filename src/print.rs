pub mod print_base {
	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	#[derive(Debug)]
	struct Type {
		id: i64,
		text: String
	}

	pub fn print_data() -> Result<()> {
		let conn = Connection::open("base.db3")?;

		let mut list = conn.prepare("SELECT id, text FROM lists")?;
		let data = list.query_map([], |row| {
			Ok(Type {
				id: row.get(0)?,
				text: row.get(1)?,
			})
		})?;

		for i in data { println!("{:?}", i); }

		Ok(())
	}
}