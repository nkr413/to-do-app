pub mod new {
	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	#[derive(Debug)]
	struct Type {
		id: i64,
		text: String
	}

	fn create_list(s: &str) -> Result<()> {
		let conn = Connection::open("base.db3")?;
		conn.execute("CREATE TABLE IF NOT EXISTS lists (
			id  		INTEGER PRIMARY KEY,
			text  	TEXT NOT NULL)", NO_PARAMS,
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
			if s == i.text {
				ifhave = true;
				break;
			}
			else { ifhave = false; }
		}

		if ifhave == false {
			v.push(Type {id: 0, text: s.to_string(),});

			for i in 0..v.len() {
				v[i].id = id_int;
				id_int += 1;
			}

			conn.execute("DELETE FROM lists", [])?;

			for i in v {conn.execute("INSERT INTO lists (id, text) values (?1, ?2)", params![i.id, i.text],)?;}

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
}