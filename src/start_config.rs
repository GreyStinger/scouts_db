pub mod arg_to_env {
	use std::env;

	// Handle all possible None values in this function so that I
	// don't have to worry about explicitly handeling it anywhere else.
	fn set_defaults() {
		if env::var_os("IP_ADDRESS") == None {
			env::set_var("IP_ADDRESS", "127.0.0.1");
		}
		if env::var_os("PORT") == None {
			env::set_var("PORT", "8080");
		}
		if env::var_os("WORKERS") == None {
			env::set_var("WORKERS", "6");
		}
	}


	// Take in command line arguments, process those that are important
	// and drop those that aren't.
	pub fn run() {
		let args: Vec<String> = env::args().collect();
		let mut i: usize = 0;

		while i < args.len() {
			if args[i].get(0..1).unwrap() == "-" {
				match args[i].as_str() {
					"-ip" => env::set_var("IP_ADDRESS", args[i + 1].clone()),
					"-p" => env::set_var("PORT", args[i +1].clone()),
					"-w" => env::set_var("WORKERS", args[i + 1].clone()),
					_ => {}
				}
			}

			i += 1;
		}

		set_defaults();
	}

}