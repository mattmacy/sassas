use std::io::{Read, Write, BufRead, BufReader};
use std::{io, path, fs};
use std::collections::{HashMap, VecDeque};

use regex::{Regex, Captures};
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

fn include_file_(include: &Vec<String>, file: &str) -> io::Result<String> {
    let includestr = include
        .iter()
        .map(|s| format!("{}/", s))
        .collect::<String>();
    let path = path::PathBuf::from(&includestr).join(file);
    let fh = fs::File::open(path)?;
    let mut fp = Box::new(BufReader::new(fh));
    Ok(String::from_utf8(fp.fill_buf()?.to_vec()).expect(
        "failed to convert input file",
    ))
}

fn include_file(include: &Vec<String>, file: &str) -> String {
    let result = include_file_(include, file);
    match result {
        Ok(r) => r,
        Err(e) => {
            println!("include failed: {:?}", e);
            ::std::process::exit(1)
        }
    }
}

fn regex_strip(expr: &str, line: &str) -> String {
    (*Regex::new(expr).unwrap().replace_all(line, "")).into()
}

fn regex_match(expr: &str, line: &str) -> bool {
    Regex::new(expr).unwrap().is_match(line)
}

fn set_constmap<'a>(constmap: &mut HashMap<String, String>, consttext: &'a str) -> &'a str {
    for line in consttext.split('\n') {
        // strip comments
        let line = regex_strip(r"(?:#|//).*", &line);
        // skip blank lines
        if !regex_match(r"\S", &line) {
            continue;
        }

        let kv = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>();

        constmap.insert(kv[0].into(), kv[1].into());
    }
    ""
}
fn constmap_lookup(constmap: &HashMap<String, String>, key: &str) -> String {
    if constmap.contains_key(key) {
        constmap[key].clone()
    } else {
        key.into()
    }
}

fn set_register_map<'a>(
    regmap: &mut HashMap<String, Vec<String>>,
    regtext: &'a str,
    remove_regmap: bool,
) -> &'a str {

    for line in regtext.split("\n") {
        let line = line.trim();
        // strip comments
        let line = regex_strip(r"?:#|//).*", &line);
        // skip blank lines
        if !regex_match(r"\S", &line) {
            continue;
        }
        let auto = regex_match(r"~", &line);
        let share = regex_match(r"=", &line);
        let kv = line.split(r"[:~=]").collect::<Vec<&str>>();
        let (reg_nums, reg_names) = (kv[0], kv[1]);
        //let mut num_list = Vec::new();
        for num in reg_nums.split(r"\s*,\s*") {
            let bounds = num.split(r"\s*\-\s*").collect::<Vec<&str>>();
            let (start, stop) = (bounds[0], bounds[1]);

        }
    }

    /* XXX */
    if remove_regmap { "" } else { regtext }
}

fn schedule_blocks(
    block: &str,
    count: usize,
    regmap: &HashMap<String, Vec<String>>,
    debug: bool,
) -> String {
    /* XXX */
    "".into()
}

pub fn preprocess(
    mut fp: Box<BufRead>,
    include: &Vec<String>,
    debug: bool,
    regmap: Option<HashMap<String, Vec<String>>>,
) -> io::Result<String> {
    let file = String::from_utf8(fp.fill_buf()?.to_vec()).expect("failed to convert input file");
    let comment_re = r#"^[\t ]*<COMMENT>.*?^\s*</COMMENT>\n?"#;
    let include_re = r#"^[\t ]*<INCLUDE\s+file="([^"]+)"\s*/?>\n?"#;
    let code_re = r"^[\t ]*<CODE(\d*)>(.*?)^\s*<\/CODE\1>\n?";
    let constmap_re = r"^[\t ]*<CONSTANT_MAPPING>(.*?)^\s*</CONSTANT_MAPPING>\n?";
    let regmap_re = r"^[\t ]*<REGISTER_MAPPING>(.*?)^\s*</REGISTER_MAPPING>\n?";
    let schedule_re = r"^[\t ]*<SCHEDULE_BLOCK>(.*?)^\s*</SCHEDULE_BLOCK>\n?";
    let inline_re = r"\[(\+|\-)(.+?)\1\]";
    let (mut regmap, remove_regmap) = match regmap {
        Some(r) => (r, true),
        None => (HashMap::new(), false),
    };
    let mut constmap = HashMap::new();

    let file = Regex::new(include_re).unwrap().replace_all(
        &file,
        |caps: &Captures| {
            format!("{}\n", include_file(include, &caps[1]))
        },
    );
    let file = regex_strip(comment_re, &file);
    // XXX implement Inline and Code

    let file = Regex::new(constmap_re).unwrap().replace_all(
        &file,
        |caps: &Captures| {
            format!("{}\n", set_constmap(&mut constmap, &caps[1]))
        },
    );

    let lines = file.split("\n");
    let mut linesnew = Vec::new();

    // replace constants with values
    let re = Regex::new(r"(\w+(?:\[\d+\])?)").unwrap();
    for line in lines {
        let line = re.replace_all(&file, |caps: &Captures| {
            format!("{}", constmap_lookup(&constmap, &caps[1]))
        });
        linesnew.push(line);
    }
    let file = linesnew
        .iter()
        .map(|l| format!("{}\n", l))
        .collect::<String>();

    let file = Regex::new(regmap_re).unwrap().replace_all(
        &file,
        |caps: &Captures| {
            format!(
                "{}\n",
                set_register_map(&mut regmap, &caps[1], remove_regmap)
            )
        },
    );
    let mut count = 0;
    let file = Regex::new(schedule_re).unwrap().replace_all(
        &file,
        |caps: &Captures| {
            count += 1;
            format!("{}\n", schedule_blocks(&caps[1], count, &regmap, debug))
        },
    );

    Ok((*file).into())
}
