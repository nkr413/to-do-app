pub mod help_func {
	use std::fs::File;
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	struct Content {
		title: String,
		description: String
	}

	pub fn commands_list() {
		let file = File::open("commands.json").expect("file not found");
		let base: Vec<Content> = serde_json::from_reader(file).expect("error while reading");
		
		println!("\n--- All Commands ---\n");
		for i in base { println!("{}\n --> {}\n", i.title, i.description); }
	}
}