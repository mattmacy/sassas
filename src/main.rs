#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};


enum CmdArgs {
	List(String),
	Other
}

fn parse_args() -> CmdArgs {
	let matches = App::new("sassas")
		.version("0.1")
		.author("Matt Macy")
		.about("sass assembler")
		.arg_from_usage("[list] -l, --list <FILE> 'List kernels and symbols'")
		.get_matches();

	let cubin_file = value_t!(matches.value_of("list"), String).ok();
	let mut result;
	result = match cubin_file {
		Some(file) => CmdArgs::List(file),
		None => CmdArgs::Other
	};
	result
}

fn main() {
	let args = parse_args();
	match args {
		CmdArgs::List(file) => println!("list file {}", file),
		CmdArgs::Other => println!("other"),
	}
}
