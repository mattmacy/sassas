#![allow(unused_variables)]

extern crate clap;
extern crate memmap;
extern crate itertools;

use clap::{App, Arg, SubCommand};
mod elf;
mod cubin;
mod sval;
mod unsafe_lib;

#[derive(Debug, Clone)]
enum CmdArgs {
    List(String),
    Test(bool, bool, String),
    Extract(Option<String>, String),
    Pre(bool, String, Option<String>),
    Insert(bool, String, Option<String>),
    Error(&'static str),
}

fn parse_args() -> CmdArgs {
    let matches = App::new("sassas")
		.version("0.1")
		.author("Matt Macy")
		.about("sass assembler")
		.subcommand(
			SubCommand::with_name("list")
			.about("List kernels and symbols")
			.arg(Arg::with_name("cubin_file")
				.index(1)))
		.subcommand(
			SubCommand::with_name("test")
			.about(
				"Test a cubin or sass file to to see if the assembler can reproduce all of the \
				contained opcodes. Also useful for extending the missing grammar rules. \
				Defaults to only showing failures without --all. With the --reg flag it \
				will show register bank conflicts not hidden by reuse flags.")
			.arg(Arg::with_name("reg").long("reg").short("r").required(false))
			.arg(Arg::with_name("all").long("all").short("a").required(false))
			.arg(Arg::with_name("cubin_or_sass_file")
				.index(1)))
		.subcommand(
			SubCommand::with_name("extract")
			.about(
				"Extract a single kernel into an asm file from a cubin. \
				 Works much like cuobjdump but outputs in a format that \
				 can be re-assembled back into the cubin.")
			.arg(Arg::with_name("kernel_name").long("kernel").short("k").required(false).takes_value(true))
			.arg(Arg::with_name("cubin_or_sass_file").index(1)))
		.subcommand(
			SubCommand::with_name("pre")
			.about("Preprocess the asm: expand CODE sections, perform scheduling. Mainly \
					used for debugging purposes. Include the debug flag to print out \
					detailed scheduler info.")
			.arg(Arg::with_name("debug").long("debug").short("d").required(false).takes_value(true))
			.arg(Arg::with_name("asm_file").index(1))
			.arg(Arg::with_name("new_asm_file").index(2).required(false)))
		.subcommand(
			SubCommand::with_name("insert")
			.about("Insert the kernel asm back into the cubin.  Overwrite existing or create new cubin. \
					Optionally you can skip register reuse flag auto insertion.  This allows you to observe \
					performance without any reuse or you can use it to set the flags manually in your sass.")
			.arg(Arg::with_name("noreuse").long("noreuse").short("n").required(false).takes_value(true))
			.arg(Arg::with_name("cubin_file").index(1))
			.arg(Arg::with_name("new_cubin_file").index(2).required(false)))
			.get_matches();

    match matches.subcommand() {
        ("list", Some(sub_m)) => {
            if let Some(file) = sub_m.value_of("cubin_file") {
                CmdArgs::List(file.into())
            } else {
                CmdArgs::Error("<file>.cubin missing")
            }
        }
        ("test", Some(sub_m)) => {
            let (mut reg, mut all) = (false, false);
            if sub_m.is_present("reg") {
                reg = true;
            }
            if sub_m.is_present("all") {
                all = true;
            }
            if let Some(file) = sub_m.value_of("cubin_or_sass_file") {
                CmdArgs::Test(reg, all, file.into())
            } else {
                CmdArgs::Error("expected cubin or sass file!")
            }
        }
        ("extract", Some(sub_m)) => {
            let kernel_name = match sub_m.value_of("kernel_name") {
                Some(name) => Some(name.into()),
                None => None,
            };
            if let Some(file) = sub_m.value_of("cubin_or_sass_file") {
                CmdArgs::Extract(kernel_name, file.into())
            } else {
                CmdArgs::Error("expected cubin or sass file!")
            }
        }
        ("pre", Some(sub_m)) => {
            let mut debug = false;
            if sub_m.is_present("debug") {
                debug = true;
            }
            let asm_file = match sub_m.value_of("asm_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            let new_asm_file = match sub_m.value_of("new_asm_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            match asm_file {
                None => CmdArgs::Error("asm file reqired"),
                Some(file) => CmdArgs::Pre(debug, file, new_asm_file),
            }
        }
        ("insert", Some(sub_m)) => {
            let mut noreuse = false;
            if sub_m.is_present("noreuse") {
                noreuse = true;
            }
            let asm_file = match sub_m.value_of("asm_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            let new_asm_file = match sub_m.value_of("new_asm_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            match asm_file {
                None => CmdArgs::Error("asm file reqired"),
                Some(file) => CmdArgs::Insert(noreuse, file, new_asm_file),
            }
        }
        _ => {
            println!("{}", matches.usage());
            CmdArgs::Error("missing subcommand")
        }
    }
}

fn sass_list(file: String) {
    let bin = cubin::Cubin::new(file);
}
fn sass_test(reg: bool, all: bool, file: String) {}
fn sass_extract(kernel_name: Option<String>, file: String) {}
fn sass_pre(debug: bool, asm_file: String, new_asm_file: Option<String>) {}
fn sass_insert(noreuse: bool, asm_file: String, new_asm_file: Option<String>) {}




fn main() {
    let args = parse_args();
    match args {
        CmdArgs::List(file) => sass_list(file),
        CmdArgs::Test(reg, all, file) => sass_test(reg, all, file),
        CmdArgs::Extract(kernel, file) => sass_extract(kernel, file),
        CmdArgs::Pre(debug, asm_file, new_asm_file) => sass_pre(debug, asm_file, new_asm_file),
        CmdArgs::Insert(noreuse, asm_file, new_asm_file) => {
            sass_insert(noreuse, asm_file, new_asm_file)
        }
        CmdArgs::Error(err) => println!("Error: {}", err),
    }
}
