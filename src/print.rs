pub mod print_func {	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	pub fn print_all() -> Result<()> {
		println!("all-data");


		Ok(())
	}

	pub fn print_one() -> Result<()> {
		println!("Which list do you want to see ? -->");

		let mut resp = String::new();
		std::io::stdin()
			.read_line(&mut resp)
			.expect("Failes");

		let rsp = resp[0..resp.len() - 2].to_string();

		Ok(())
	}
}