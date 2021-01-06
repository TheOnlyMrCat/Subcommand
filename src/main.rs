use std::io::{self, BufReader, BufRead};

fn main() {
	let cmd_name = std::env::args_os().nth(1).unwrap();
	let cmd_string = cmd_name.to_string_lossy().into_owned();
	eprint!("scmd {}>", cmd_string);
	
	for line in BufReader::new(io::stdin()).lines() {
		match line {
			Ok(line) => {
				if line.is_empty() {
					break;
				}
				std::process::Command::new(&cmd_name)
					.args(&line.split(' ').collect::<Vec<_>>())
					.status().unwrap();
				eprint!("scmd {}>", cmd_string);
			},
			Err(e) => {
				eprintln!("{}", e);
				break;
			}
		}
	}
}
