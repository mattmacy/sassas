use std::io::{Read, Write, BufRead, BufReader};
use std::io;
use std::collections::VecDeque;
use sval::*;

pub fn test(fp: Box<BufRead>, reg: bool, all: bool) -> io::Result<()> {
    unimplemented!();
    Ok(())
}

pub fn extract(
    fp: Box<BufRead>,
    out: Box<Write>,
    params: Option<&VecDeque<String>>,
) -> io::Result<()> {
    unimplemented!();
    Ok(())
}

pub fn assemble(file: &String, include: Vec<String>, reuse: bool) -> io::Result<KernelSection> {
    let mut kernel_sec = KernelSection::default();
    unimplemented!();
    Ok(kernel_sec)
}

pub fn preprocess(fp: Box<BufRead>, include: Vec<String>, debug: bool) -> io::Result<String> {
    let result = String::new();
    unimplemented!();
    Ok(result)
}
