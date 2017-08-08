use std::io::{Read, Write, BufRead, BufReader};
use std::io;
use std::collections::VecDeque;

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
