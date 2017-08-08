use std::io::{Read, Write, BufRead, BufReader};
use std::io;
use std::collections::VecDeque;
use sval::*;

pub fn test(fp: Box<BufRead>, reg: bool, all: bool) -> io::Result<()> {
    Ok(())
}

pub fn extract(
    fp: Box<BufRead>,
    out: Box<Write>,
    params: Option<&VecDeque<String>>,
) -> io::Result<()> {
    Ok(())
}

pub fn assemble(file: &String, include: Vec<String>, reuse: bool) -> io::Result<KernelSection> {
    let mut kernel_sec = KernelSection::default();
    Ok(kernel_sec)
}
