use std::io::{Read, Write, BufRead, BufReader};
use std::{io, path, fs, ops};
use std::collections::{HashMap, VecDeque};
use utils::{regex_strip, regex_matches, regex_match, SVal, KernelSection, MutStrMap};

use regex::{Regex, Captures};
use sassas_grammar::*;

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

pub fn assemble(file: &String, include: &Vec<String>, reuse: bool) -> io::Result<KernelSection> {
    let mut kernel_sec = KernelSection::default();
    let mut regmap = MutStrMap::new();
    let file = preprocess(file, include, false, Some(&mut regmap));

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
    let mut vectors: MutStrMap<Vec<String>> = match regmap.get("__vectors") {
        Some(v) => v.clone().into(),
        None => MutStrMap::new(),
    };
    let mut regbank: MutStrMap<String> = match regmap.get("__regbank") {
        Some(v) => v.clone().into(),
        None => MutStrMap::new(),
    };
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
                let caps = &regex_matches(reg1_re, name)[0];
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
                            regbank.insert(&r, bank.unwrap().into());
                        } else {
                            println!("Cannot request a bank for a fixed register range: {}", name);
                        }
                    }
                }
            } else if regex_match(reg2_re, name) {
                let caps = &regex_matches(reg2_re, name)[0];
                name_list.push(caps[1].into());
                if caps.len() <= 2 {
                    continue;
                }
                if auto {
                    // help out the type checker :-/
                    let b: String = caps[2].into();
                    regbank.insert(&caps[1], b);
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
                regmap.insert(n_name, num_list.clone().into());
            } else if share {
                regmap.insert(n_name, format!("R{}", num_list[0]).into());
            } else {
                regmap.insert(n_name, format!("R{}", num_list[0]).into());
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
                vectors.insert(&name_list[n], name_list[n..end].to_vec());
                if !aliases.contains_key(&name_list[n]) ||
                    regmap.contains_key(&aliases[&name_list[n]])
                {
                    continue;
                }
                {
                    let alias_name = &aliases[&name_list[n]];
                    unborrow!(regmap.insert(alias_name, regmap[n_name].clone()));
                    unborrow!(vectors.insert(alias_name, vectors[n_name].clone()));
                }
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

fn preprocess_line(line: &str) -> bool {
    unimplemented!();
    false
}

fn scheduler(
    sg: &SassGrammar,
    block: &str,
    count: usize,
    regmap: &MutStrMap<SVal>,
    debug: bool,
) -> String {
    let src_reg = vec!["r8", "r20", "r39", "p12", "p29", "p39", "X"];
    let dest_reg = vec!["r0", "p0", "p3", "p45", "p48", "CC"];
    let itypes = vec!["class", "lat", "rlat", "tput", "dual"];
    let mut regops = Vec::new();
    regops.append(&mut src_reg.clone());
    regops.append(&mut dest_reg.clone());

    let vectors: &MutStrMap<SVal> = (&regmap["__vectors"]).into();
    let mut linenum = 0;
    let (mut comments, mut ordered, mut first, mut instrs) = (Vec::new(), 0, true, Vec::new());
    for line in block.split("\n") {
        linenum += 1;
        if !preprocess_line(&line) {
            if regex_match(r"\S", line) {
                comments.push(line)
            }
            continue;
        }
        let mut inst = MutStrMap::new();
        if sg.process_asm_line(&line, linenum, &mut inst) {
            // match an instruction
            let ctrl: u32 = inst["ctrl"].clone().into();
            inst["first"] = (first || (ctrl & 0x1_f800) == 0).into();
            first = false;
            inst["exeTime"] = 0.into();
            inst["order"] = ordered.into();
            if ordered != 0 {
                ordered += 1;
            }
            let comment: String = inst["comment"].clone().into();
            inst["force_stall"] = (if regex_match(r"FORCE", &comment) {
                                       ctrl & 0xf
                                   } else {
                                       0
                                   }).into();
            instrs.push(inst);
        } else if regex_match(r"^([a-zA-Z]\w*):", line) {
            // match a label
            panic!(
                "SCHEDULE_BLOCK's cannot contain labels. block: {} line: {}",
                count,
                linenum
            );
        } else if regex_match(r"^<ORDERED>", line) {
            // open an ORDERED block
            if ordered != 0 {
                panic!(" <ORDERED> tags cannot be nested!")
            }
            ordered = 1;
        } else if regex_match(r"^</ORDERED>", line) {
            // open an ORDERED block
            if ordered == 0 {
                panic!("missing opening <ORDERED> tag!")
            }
            ordered = 0;
        } else {
            panic!(
                "badly formed line at block: {} line: {}: {}",
                count,
                linenum,
                line
            );
        }
    }
    let grammar = &sg.grammar;
    for mut inst in instrs {
        let mut matched = false;
        // disambiguate for the type checker :-/
        let op: String = inst["op"].clone().into();
        // cap_data can capture immutable references so create local copy
        let inst_str: String = inst["inst"].clone().into();
        for gram in grammar[op.as_str()].iter() {
            let mut cap_data = HashMap::new();
            if !parse_instruct(&inst_str, &gram.rule, &mut cap_data) {
                continue;
            }
            let mut src: Vec<String> = Vec::new();
            // copy over instruction types for easier access
            // XXX this creates a new regex instance
            inst["itypes"] = gram.itype.clone().into();
            inst["dual"] = (if inst["dualCnt"].clone().into() { 1 } else { 0 }).into();
            src.push(inst["predReg"].clone().into());
        }
    }

    /* XXX replaceXMADs*/
    "".into()
}

pub fn preprocess(
    file: &String,
    include: &Vec<String>,
    debug: bool,
    regmap: Option<&mut MutStrMap<SVal>>,
) -> io::Result<String> {
    let comment_re = r#"^[\t ]*<COMMENT>.*?^\s*</COMMENT>\n?"#;
    let include_re = r#"^[\t ]*<INCLUDE\s+file="([^"]+)"\s*/?>\n?"#;
    let code_re = r"^[\t ]*<CODE(\d*)>(.*?)^\s*<\/CODE\1>\n?";
    let constmap_re = r"^[\t ]*<CONSTANT_MAPPING>(.*?)^\s*</CONSTANT_MAPPING>\n?";
    let regmap_re = r"^[\t ]*<REGISTER_MAPPING>(.*?)^\s*</REGISTER_MAPPING>\n?";
    let schedule_re = r"^[\t ]*<SCHEDULE_BLOCK>(.*?)^\s*</SCHEDULE_BLOCK>\n?";
    let inline_re = r"\[(\+|\-)(.+?)\1\]";
    let sg = SassGrammar::new();
    let mut rtmp;
    let (mut regmap, remove_regmap) = match regmap {
        Some(r) => (r, true),
        None => {
            rtmp = MutStrMap::new();
            (&mut rtmp, false)
        }
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
            format!("{}\n", scheduler(&sg, &caps[1], count, &regmap, debug))
        },
    );

    Ok((*file).into())
}
