pub mod arg_handler {
	// If I continue with this project.. I am sorry future me...
	// I really had no idea how to do this better T-T

	use std::collections::HashMap;

	// Function I wrote to print out hashmap keys and values cause things weren't working.
	fn print_maps(map: &HashMap<&str, String>) {
		for (key, value) in &*map {
			println!("{0:?} : {1:?}", key, value);
		}

		println!("print_maps ran");
	}

	//TODO Complete system that sorts through imputed arguments and puts them neatly into Hashed Maps
	pub fn sort(args: Vec<String>) -> HashMap<&'static str, String> {
		let mut hash_map: HashMap<&str, String> = HashMap::new();

		let mut i: usize = 0;

		while i < args.len() {
			if args[i].get(0..1).unwrap() == "-" {
				match args[i].as_str() {
					"-ip" => {hash_map.insert("ip_addr", args[i + 1].clone());},
					"-p" => {hash_map.insert("port", args[i + 1].clone());},
					"-w" => {hash_map.insert("workers", args[i + 1].clone());},
					_ => {}
				}
			}

			// println!("{:?}", args[i]);
			
			i += 1;
		}

		print_maps(&hash_map);

		hash_map
	}
}

mod routing {

}