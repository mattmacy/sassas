#![allow(unused_variables)]

extern crate clap;
extern crate memmap;
extern crate itertools;
extern crate num;
extern crate regex;
extern crate dyon;
#[macro_use]
extern crate unborrow;

use regex::Regex;
use std::process::{Command, Stdio};
use clap::{App, Arg, SubCommand};
use std::io;
use cubin::Cubin;
use std::path::Path;
use std::collections::VecDeque;
use std::io::{Read, Write, BufRead, BufReader};
use std::fs::File;
use utils::KernelSection;


mod cubin;
mod maxas;
mod sassas_grammar;
mod utils;

#[derive(Debug, Clone)]
enum CmdArgs {
    List(String),
    Test(bool, bool, String),
    Extract(Option<String>, String),
    Pre(bool, String, Option<String>),
    Insert(bool, String, String, Option<String>),
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
            .arg(Arg::with_name("asm_file").index(1))
            .arg(Arg::with_name("cubin_file").index(2))
			.arg(Arg::with_name("new_cubin_file").required(false).index(3)))
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
            let reg = sub_m.is_present("reg");
            let all = sub_m.is_present("all");
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
            let debug = sub_m.is_present("debug");
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
            let noreuse = sub_m.is_present("noreuse");
            let asm_file = match sub_m.value_of("asm_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            let cubin_file = sub_m.value_of("cubin_file").unwrap().into();
            let new_cubin_file = match sub_m.value_of("new_cubin_file") {
                Some(file) => Some(file.into()),
                None => None,
            };
            match asm_file {
                None => CmdArgs::Error("asm file reqired"),
                Some(file) => CmdArgs::Insert(noreuse, file, cubin_file, new_cubin_file),
            }
        }
        _ => {
            println!("{}", matches.usage());
            CmdArgs::Error("missing subcommand")
        }
    }
}

fn do_cuobjdump(arch: u32, file: &String, kernel: &Option<String>) -> io::Result<Box<BufRead>> {
    let kernelcmd = match kernel {
        &Some(ref kernel_name) => format!(" -fun {}", kernel_name),
        &None => String::from(""),
    };
    let mut child = Command::new("cuobjdump")
        .arg(format!(" -arch sm_{}", arch))
        .arg(format!(" -sass {}", file))
        .arg(kernelcmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("cuobjdump failed");
    let fh = child.stdout.take().unwrap();
    let mut fp = Box::new(BufReader::new(fh));
    let buf =
        String::from_utf8(fp.fill_buf()?.to_vec()).expect("failed to convert output to string");
    if buf.contains("cuobjdump fatal") {
        println!("{}", buf);
        std::process::exit(1);
    }

    Ok(fp)
}

fn sass_list(file: &String) -> io::Result<()> {
    let bin = cubin::Cubin::new(file)?;
    let (arch, class, addr_size) = (bin.arch, bin.class, bin.addr_size);
    let kernels = bin.list_kernels();
    let symbols = bin.list_symbols();
    println!(
        "{}: arch:sm_{} machine:{}bit address_size:{}bit",
        file,
        arch,
        class,
        addr_size
    );
    for (k, v) in kernels.iter() {
        let ker: KernelSection = v.clone().into();
        println!(
            "Kernel: {} (Linkage: {:?}, Params: {}, Size: {}, Registers: {}, SharedMem: {}, Barriers: {}",
            k,
            ker.linkage,
            ker.param_cnt,
            ker.size,
            ker.reg_cnt,
            ker.shared_size,
            ker.bar_cnt
        )
    }
    for (k, _) in symbols.iter() {
        println!("Symbol: {}", k)
    }
    Ok(())
}

fn sass_test(reg: bool, all: bool, file: &String) -> io::Result<()> {
    let fp: Box<BufRead> = if Cubin::is_elf(file)? {
        let bin = cubin::Cubin::new(file)?;
        let arch = bin.arch;
        do_cuobjdump(arch, file, &None)?
    } else {
        let fh = File::open(file)?;
        Box::new(BufReader::<File>::new(fh))
    };
    maxas::test(fp, reg, all)?;
    Ok(())
}

fn sass_extract_cubin(
    kernel_name: &Option<String>,
    cubin_file: &String,
    asm_file: &Option<String>,
) -> io::Result<()> {
    let bin = cubin::Cubin::new(cubin_file)?;
    let arch = bin.arch;
    let kernels = bin.list_kernels();
    let first_kernel = kernels.keys().nth(0).unwrap().clone();
    let kernel_name = kernel_name.clone().unwrap_or(first_kernel);
    let kernel: &KernelSection = kernels
        .get(&kernel_name)
        .expect(&format!("bad kernel: {}", kernel_name))
        .into();
    let fp = do_cuobjdump(arch, cubin_file, &Some(kernel_name.clone()))?;
    let mut out = match *asm_file {
        Some(ref x) => {
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(io::stdout()) as Box<Write>,
    };
    out.write_fmt(format_args!(
        "# Kernel: {}\n# Arch: sm_{}\n",
        kernel_name,
        arch
    ))?;
    out.write_fmt(
        format_args!("# InsCnt: {}", kernel.instr_cnt),
    )?;
    out.write_fmt(format_args!("# RegCnt: {}", kernel.reg_cnt))?;
    out.write_fmt(
        format_args!("# SharedSize: {}", kernel.shared_size),
    )?;
    out.write_fmt(format_args!("# BarCnt: {}", kernel.bar_cnt))?;
    let params = if kernel.map.contains_key("Params") {
        let params: &VecDeque<String> = (&kernel.map["Params"]).into();
        let paramstr = params
            .iter()
            .map(|s| format!("#\t{}\n", s))
            .collect::<String>();
        out.write_all(paramstr.as_bytes())?;
        Some(params)
    } else {
        None
    };
    out.write_all(b"#\n# Instructions:\n\n")?;
    maxas::extract(fp, out, params)?;
    Ok(())
}

fn sass_extract_sass(sass_file: &String, asm_file: &Option<String>) -> io::Result<()> {
    let out = match *asm_file {
        Some(ref x) => {
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(io::stdout()) as Box<Write>,
    };
    let fh = File::open(sass_file)?;
    let fp = Box::new(BufReader::<File>::new(fh));
    maxas::extract(fp, out, None)?;
    Ok(())
}

fn sass_extract(
    kernel_name: &Option<String>,
    input_file: &String,
    asm_file: &Option<String>,
) -> io::Result<()> {
    if Cubin::is_elf(input_file)? {
        sass_extract_cubin(kernel_name, input_file, asm_file)?;
    } else {
        sass_extract_sass(input_file, asm_file)?;
    }
    Ok(())
}

fn sass_insert(
    noreuse: bool,
    sass_file: &String,
    cubin_file: &String,
    new_cubin_file: &Option<String>,
) -> io::Result<()> {
    let out = match *new_cubin_file {
        Some(ref x) => {
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(io::stdout()) as Box<Write>,
    };
    let fh = File::open(sass_file)?;
    let mut fp = Box::new(BufReader::<File>::new(fh));
    let buf =
        String::from_utf8(fp.fill_buf()?.to_vec()).expect("failed to convert input file to string");
    let re = Regex::new(r"^# Kernel: (\w+)").unwrap();
    let kernel_match = &re.captures_iter(&buf).nth(0);
    let kernel_name = match *kernel_match {
        None => {
            println!("No kernel found");
            ::std::process::exit(1)
        }
        Some(ref name) => &name[1],
    };
    let new_cubin_file = new_cubin_file.clone().unwrap_or(cubin_file.clone());
    let include = Vec::new();
    let mut kernel = maxas::assemble(sass_file, include, !noreuse)?;
    let mut cubin = Cubin::new(cubin_file)?;
    kernel.map.insert(
        "Kernel",
        cubin.get_kernel(&kernel_name.into())?.into(),
    );
    cubin.modify_kernel(&kernel)?;
    cubin.write(&new_cubin_file)?;

    println!("kernel_name: {}", kernel_name);
    Ok(())
}

fn sass_pre(debug: bool, sass_file: &String, new_asm_file: &Option<String>) -> io::Result<()> {
    /* XXX process -D(\w+) */

    let mut out = match *new_asm_file {
        Some(ref x) => {
            if x == sass_file {
                println!("source and destination should not be the same");
                ::std::process::exit(1)
            }
            let path = Path::new(x);
            Box::new(File::create(&path).unwrap()) as Box<Write>
        }
        None => Box::new(io::stdout()) as Box<Write>,
    };
    let fh = File::open(sass_file)?;
    let fp = Box::new(BufReader::<File>::new(fh));
    let include = Vec::new();
    let result = maxas::preprocess(fp, &include, debug, None)?;
    out.write_all(result.as_bytes())?;
    Ok(())
}

fn main() {
    let args = parse_args();
    match args {
        CmdArgs::List(ref file) => sass_list(file),
        CmdArgs::Test(reg, all, ref file) => sass_test(reg, all, file),
        CmdArgs::Extract(ref kernel, ref file) => sass_extract(kernel, file, &None),
        CmdArgs::Pre(debug, ref asm_file, ref new_asm_file) => {
            sass_pre(debug, asm_file, new_asm_file)
        }
        CmdArgs::Insert(noreuse, ref asm_file, ref cubin_file, ref new_cubin_file) => {
            sass_insert(noreuse, asm_file, cubin_file, new_cubin_file)
        }
        CmdArgs::Error(err) => {
            println!("Error: {}", err);
            Ok(())
        }
    };
}
