use std::io::{Read, Write, BufRead, BufReader};
use std::{io, path, fs, ops};
use std::collections::{HashMap, VecDeque};
use unsafe_lib::MutStrMap;

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
fn regex_matches<'t>(expr: &str, line: &'t str) -> Option<Vec<Captures<'t>>> {
    if regex_match(expr, line) {
        Some(Regex::new(expr).unwrap().captures_iter(line).collect())
    } else {
        None
    }
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

fn get_range(bounds: &Vec<&str>) -> Result<ops::Range<usize>, ::std::num::ParseIntError> {
    let range = if bounds.len() == 1 {
        let start: usize = bounds[0].parse()?;
        start..start + 1
    } else {
        let (start, stop): (usize, usize) = (bounds[0].parse()?, bounds[1].parse()?);
        start..stop + 1
    };
    Ok(range)
}

fn set_register_map_<'a>(
    regmap: &mut MutStrMap<SVal>,
    regtext: &'a str,
    remove_regmap: bool,
) -> Result<&'a str, ::std::num::ParseIntError> {
    let reg1_re = r"^(\w+)<((?:\d+(?:\s*\-\s*\d+)?\s*\|?\s*)+)>(\w*)(?:\[([0-3])\])?$";
    let reg2_re = r"^(\w+)(?:\[([0-3])\])?$";
    /* XXX  -- look up in regMap */
    let mut vectors: MutStrMap<SVal> = match regmap.get("__vectors") {
        Some(v) => v.clone().into(),
        None => MutStrMap::new(),
    };
    let mut regbank: MutStrMap<String> = match regmap.get("__regbank") {
        Some(v) => v.clone().into(),
        None => MutStrMap::new(),
    };
    /* XXX  -- look up in regMap */

    let mut vectors = HashMap::<String, Vec<String>>::new();
    let mut aliases: HashMap<String, String> = HashMap::new();

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
        let kv = line.split(|c| c == ':' || c == '~' || c == '=')
            .collect::<Vec<&str>>();
        let (reg_nums, reg_names) = (kv[0], kv[1]);
        let mut num_list = Vec::<usize>::new();
        for num in reg_nums.split(r",") {
            let bounds = num.split(r"-").map(|s| s.trim()).collect::<Vec<&str>>();
            let mut range_list = get_range(&bounds)?.collect();
            num_list.append(&mut range_list);
        }
        let mut name_list = Vec::<String>::new();
        for name in reg_names.split(",") {
            let name = name.trim();
            if regex_match(reg1_re, name) {
                let caps = &regex_matches(reg1_re, name).unwrap()[0];
                let (name1, name2) = (&caps[1], &caps[3]);
                let bank = if caps.len() > 4 { Some(&caps[4]) } else { None };
                for s in (&caps[2]).split("|") {
                    let bounds = s.split("-").collect::<Vec<&str>>();
                    let range = get_range(&bounds)?;
                    for r in range.map(|v| format!("{}{}{}", name1, v, name2)) {
                        if !aliases.contains_key(&r) {
                            aliases.insert(r.clone(), format!("{}{}", name1, name2));
                        }
                        name_list.push(r.clone());
                        if bank.is_none() {
                            continue;
                        }
                        if auto {
                            regbank.insert(r, bank.unwrap().into());
                        } else {
                            println!("Cannot request a bank for a fixed register range: {}", name);
                        }
                    }
                }
            } else if regex_match(reg2_re, name) {
                let caps = &regex_matches(reg2_re, name).unwrap()[0];
                name_list.push(caps[1].into());
                if caps.len() <= 2 {
                    continue;
                }
                if auto {
                    // help out the type checker :-/
                    let (a, b): (String, String) = (caps[1].into(), caps[2].into());
                    regbank.insert(a, b);
                } else {
                    println!("Cannot request a bank for a fixed register range: {}", name);
                }
            } else {
                panic!("Bad register name: '{}' at: {}", name, line);
            }
        }
        if (!share && num_list.len() < name_list.len()) || (share && num_list.len() > 1) {
            panic!("Mismatched register mapping at: {}", &line);
        }
        let mut i = 0;
        while i < num_list.len() - 1 {
            if num_list[i] != num_list[i + 1] {
                break;
            }
            i += 1;
        }
        let ascending = i + 1 == num_list.len();
        for n in 0..name_list.len() {
            let n_name: &String = (&name_list[n]).into();
            if regmap.contains_key(n_name) {
                panic!("register defined twice: {}", n_name)
            }
            if auto {
                regmap.insert(n_name.clone(), num_list.clone().into());
            } else if share {
                regmap.insert(n_name.clone(), format!("R{}", num_list[0]).into());
            } else {
                regmap.insert(n_name.clone(), format!("R{}", num_list[0]).into());
                if ascending && num_list[n] & 1 != 0 {
                    continue;
                }
                let end = if num_list[n] & 2 != 0 || n + 3 > name_list.len() {
                    n + 1
                } else {
                    n + 3
                };
                if end > name_list.len() {
                    continue;
                }
                vectors.insert(name_list[n].clone().into(), name_list[n..end].to_vec());
                if !aliases.contains_key(&name_list[n]) ||
                    regmap.contains_key(&aliases[&name_list[n]])
                {
                    continue;
                }
                let alias_name = aliases[&name_list[n]].clone();
                unborrow!(regmap.insert(alias_name.clone(), regmap[n_name].clone()));
                unborrow!(vectors.insert(alias_name, vectors[n_name].clone()));
                aliases.remove(n_name);
            }
        }
    }

    /* XXX */
    if remove_regmap { Ok("") } else { Ok(regtext) }
}
fn set_register_map<'a>(
    regmap: &mut MutStrMap<SVal>,
    regtext: &'a str,
    remove_regmap: bool,
) -> &'a str {
    let result = set_register_map_(regmap, regtext, remove_regmap);
    match result {
        Ok(r) => r,
        Err(e) => {
            panic!("parse failed: {:?}", e);
        }
    }
}
fn scheduler(block: &str, count: usize, regmap: &MutStrMap<SVal>, debug: bool) -> String {
    /* XXX replaceXMADs*/
    "".into()
}

pub fn preprocess(
    mut fp: Box<BufRead>,
    include: &Vec<String>,
    debug: bool,
    regmap: Option<MutStrMap<SVal>>,
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
        None => (MutStrMap::new(), false),
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
            format!("{}\n", scheduler(&caps[1], count, &regmap, debug))
        },
    );

    Ok((*file).into())
}
