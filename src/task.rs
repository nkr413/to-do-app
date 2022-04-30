pub mod task_func {
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

	// RETURN DATABASE <LIST>
	fn list_db_data() -> Result<Vec<Type>> {
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

		Ok(v)
	}

	// RETURN DATABASE <BASE>
	fn base_db_data() -> Result<Vec<Note>> {
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

		let mut v = Vec::new();
		for i in data {v.push(i.unwrap());}

		Ok(v)
	}


	fn update_len(d: &Vec<Type>, s: &str) -> Result<()> {
		let mut v = Vec::new();

		for i in d {
			if i.text == s {
				v.push(Type {
					id: i.id,
					text: i.text.to_string(),
					length: i.length + 1,
				});
			}
			else { v.push(Type { id: i.id, text: i.text.to_string(), length: i.length, }); }
		}

		let conn = Connection::open("base.db3")?;
		conn.execute("DELETE FROM list", [])?;

		for i in &v {conn.execute("INSERT INTO list (id, text, length) values (?1, ?2, ?3)", params![i.id, i.text, i.length],)?;}
		
		Ok(())
	}

	fn write_task(s: &str) -> Result<()> {
		println!("Write the task you want to add to the {:?} category -->", &s);
		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();

		let conn = Connection::open("base.db3")?;
		conn.execute("CREATE TABLE IF NOT EXISTS base (
			id  		INTEGER PRIMARY KEY,
			text  	TEXT NOT NULL,
			status 	TEXT NOT NULL,
			list    TEXT NOT NULL)", NO_PARAMS,
		)?;

		let mut list = conn.prepare("SELECT id, text, status, list FROM base")?;
		let data = list.query_map([], |row| {
			Ok(Note {
				id: row.get(0)?,
				text: row.get(1)?,
				status: row.get(2)?,
				list: row.get(3)?,
			})
		})?;

		let mut id_int: i64 = 1;
		let mut v = Vec::new();
		for i in data { v.push(i.unwrap()); }

		v.push(Note {
			id: 0,
			text: rsp.to_string(),
			status: "false".to_string(),
			list: s.to_string()
		});

		for i in 0..v.len() {
			v[i].id = id_int;
			id_int += 1;
		}

		conn.execute("DELETE FROM base", [])?;
		for i in v {conn.execute("INSERT INTO base (id, text, status, list) values (?1, ?2, ?3, ?4)", params![i.id, i.text, i.status, i.list],)?;}

		println!("Task added !");

		Ok(())
	}

	pub fn add_task() -> Result<()> {
		println!("Which list do you want to add the task to ? -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let new_rsp = resp[0..resp.len() - 2].to_string();

		let mut ifhave: bool = false;
		let mut v = list_db_data().unwrap();
		
		for i in &v {
			if new_rsp == i.text {
				ifhave = true;
				break;
			}
			else { ifhave = false; }
		}

		if ifhave == true {
			update_len(&v, &new_rsp);
			write_task(&new_rsp);
		}
		else { println!("There is no such category");}

		Ok(())
	}

	pub fn delete_note() {
		fn delete(list_name: &str, list_db: &Vec<Type>, base_db: &Vec<Note>) -> Result<()> {
			println!("Write the <ID> of the note -->");

			let mut resp = String::new();
			std::io::stdin()
				.read_line(&mut resp)
				.expect("Failes");

			let rsp: i64 = resp[0..resp.len() - 2].to_string().trim().parse().unwrap();
			let mut new_base = Vec::new();
			let mut ifhave: bool = false;
			let mut id_int: i64 = 1;

			for i in base_db {
				if i.id != rsp {
					ifhave = true;

					new_base.push(Note {
						id: id_int,
						text: i.text.to_string(),
						status: i.status.to_string(),
						list: i.list.to_string()
					});

					id_int += 1;
				}
			}

			if ifhave == false {
				println!("No such identifier was found");
				Ok(())
			}
			else {
				let mut new_list = Vec::new();
				
				for i in list_db {
					if i.text == list_name {
						new_list.push(Type {
							id: i.id,
							text: i.text.to_string(),
							length: i.length - 1,
						});
					} else { new_list.push(Type {id: i.id, text: i.text.to_string(), length: i.length,}); }
				}

				let conn = Connection::open("base.db3")?;
				conn.execute("DELETE FROM base", [])?;
				conn.execute("DELETE FROM list", [])?;

				for i in &new_list {conn.execute("INSERT INTO list (id, text, length) values (?1, ?2, ?3)", params![i.id, i.text, i.length],)?;}
				for i in &new_base {conn.execute("INSERT INTO base (id, text, status, list) values (?1, ?2, ?3, ?4)", params![i.id, i.text, i.status, i.list],)?;}

				println!("Note deleted !");

				Ok(())
			}
		}

		println!("From which list do you want to delete a note ? -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();
		let mut ifhave: bool = false;

		let v = list_db_data().unwrap();
		let b = base_db_data().unwrap();
		let mut new_base = Vec::new();

		for i in &v {
			if rsp == i.text {
				ifhave = true;
				break;
			} else { ifhave = false; }
		}

		if ifhave == true {
			for i in &b {
				if i.list == rsp { new_base.push(i); }
				else { continue; }
			}

			println!("\n--- Select the note <ID> to delete it from <{}> ---\n", rsp);
			for i in &new_base { println!("id: {:?} - {}", i.id, i.text); }
			println!("\n--------------\n");

			delete(&rsp, &v, &b);

		} else { println!("List not found !"); }
	}


	fn done(list_name: &str) -> Result<()> {
		println!("\nEnter the ID of the note you want to mark as completed -->\n");

		let v = base_db_data().unwrap();

		for i in v {
			if i.list == list_name {
				if i.status == "true".to_string() {println!("{:?}. (✅) -> {:?}", i.id, i.text);}
				else {println!("{:?}. (❌) -> {:?}", i.id, i.text);}
			}
		}
		println!("\n--------------\n");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let old_rsp = resp[0..resp.len() - 2].to_string();
		let rsp: i64 = old_rsp.trim().parse().unwrap();
		let mut ifhave: bool = false;
		let mut new_base = Vec::new();
		let b = base_db_data().unwrap();

		for i in b {
			if i.id == rsp {
				ifhave = true;

				new_base.push(Note {
					id: i.id,
					text: i.text.to_string(),
					status: "true".to_string(),
					list: i.list.to_string()
				});
			} else {
				new_base.push(Note {id: i.id, text: i.text.to_string(), status: i.status.to_string(), list: i.list.to_string()});
			}
		}

		if ifhave == true {
			let conn = Connection::open("base.db3")?;
			conn.execute("DELETE FROM base", [])?;

			for i in &new_base {conn.execute("INSERT INTO base (id, text, status, list) values (?1, ?2, ?3, ?4)", params![i.id, i.text, i.status, i.list],)?;}

			println!("Note marked as 'completed' !");
		} else {println!("Incorrect ID !");}

		Ok(())
	}

	pub fn done_note() {
		println!("Select a list -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();

		let mut ifhave: bool = false;
		let mut v = list_db_data().unwrap();
		
		for i in &v {
			if rsp == i.text {
				ifhave = true;
				break;
			}
			else { ifhave = false; }
		}

		if ifhave == true {done(&rsp);}
		else {println!("There is no such category");}
	}
}