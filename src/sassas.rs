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
    let file = preprocess(file, include, false, Some(&mut regmap))?;
    let sg = SassGrammar::new();
    let vectors: Option<MutStrMap<Vec<String>>> = if let Some(s) = regmap.remove("__vectors") {
        Some(s.into())
    } else {
        None
    };
    let regbank: MutStrMap<String> = if let Some(s) = regmap.remove("__regbank") {
        s.into()
    } else {
        MutStrMap::new()
    };
    //let labels =
    let (mut instructs, mut branches) = (vec![AsmInstr::default()], Vec::new());
    let mut ctrl = vec![Vec::new()];
    let mut ctrl_idx = 0;
    let mut labels = MutStrMap::<u64>::new();
    for (linenum, line) in file.split("\n").enumerate() {

        /* XXX skip of not preprocess_line */

        let mut inst = AsmInstr::default();
        if sg.process_asm_line(&line, linenum, &mut inst) {
            if sg.no_dest.contains(&inst.op.as_str()) && inst.ctrl & 0x000e0 != 0x000e0 {
                panic!(
                    "It is illegal to set a Read-After-Write dependency on a memory store op (store ops don't write to a register)\n{}",
                    inst.inst
                );
            }
            if sg.jump_op.contains(&inst.op.as_str()) {
                branches.push(instructs.len());
            }
            ctrl[ctrl_idx].push(inst.ctrl);
            inst.ctrl_idx = ctrl_idx;
            if instructs.len() & 3 == 0 {
                ctrl.push(Vec::new());
                instructs.push(AsmInstr::default());
                ctrl_idx += 1;
            }
        } else if regex_match(r"^([a-zA-Z]\w*):", line) {
            let matches = regex_matches(r"^([a-zA-Z]\w*):", line);
            labels[&matches[0][1]] = instructs.len() as u64;
        } else {
            panic!("badly formed line at {}: {}", linenum, line);
        }
    }
    // add the final BRA op and align the number of instructions to a multiple of 8
    ctrl[ctrl_idx].push(0x007ff);
    instructs.push(AsmInstr {
        op: "BRA".into(),
        inst: "BRA 0xfffff8".into(),
        ..Default::default()
    });
    while instructs.len() & 7 != 0 {
        if instructs.len() & 3 == 0 {
            ctrl.push(Vec::new());
            instructs.push(AsmInstr::default());
            ctrl_idx += 1;
        }
        ctrl[ctrl_idx].push(0x007e0);
        instructs.push(AsmInstr {
            op: "NOP".into(),
            inst: "NOP".into(),
            ..Default::default()
        });
    }
    // remap labels
    for i in branches {
        let inst = instructs[i].inst.clone();
        let matches = regex_matches(r"(\w+);$", &inst);
        if matches.is_empty() || !labels.contains_key(&matches[0][1]) {
            panic!("instruction has invalid label: {}", inst);
        }
        instructs[i].jump = labels[&matches[0][1]];
        if sg.rel_offset.contains(&instructs[i].op.as_str()) {
            let inst = Regex::new(r"(\w+);$").unwrap().replace_all(
                &inst,
                |caps: &Captures| {
                    format!(
                        "0x{:06x}",
                        ((labels[&caps[1]] - hex(&caps[1]) - 1) * 8) & 0xffffff
                    )
                },
            );
            instructs[i].inst = (*inst).into();
        } else {
            let inst = Regex::new(r"(\w+);$").unwrap().replace_all(
                &inst,
                |caps: &Captures| {
                    format!("0x{:06x}", (labels[&caps[1]] * 8) & 0xffffff)
                },
            );
            instructs[i].inst = (*inst).into();
        }
    }
    // calculate optimal register reuse
    // This effects register bank decisions so do it before analyzing register use
    for (i, instr) in instructs.iter().enumerate() {
        if i & 3 == 0 {
            continue;
        }
        let (op, inst, ctrl) = (&instr.op, &instr.inst, instr.ctrl);
        let mut matched = false;

        for elt in &sg.grammar[op.as_str()] {
            let mut cap_data = HashMap::new();
            if !parse_instruct(&inst, &elt.rule, &mut cap_data) {
                continue;
            }
            matched = true;
            if reuse {

            } else if elt.itype.reuse {

            }
            break;
        }
        if !matched {
            for r in &sg.grammar[op.as_str()] {
                println!("rule: {:?}", r.rule);
            }
            panic!("Unable to encode: {}", inst);
        }

    }
    // Assign registers to requested banks if possible
    let mut keys = regbank.keys().collect::<Vec<_>>();
    keys.sort();
    for r in keys {}
    // calculate register live times and preferred banks for non-fixed registers.
    // LiveTime only half implemented...
    for (i, inst) in instructs.iter().enumerate() {}

    // assign unassigned registers
    // sort by most restricted, then most used, then name

    // for r in paired_banks


    // Now assign any remaining to first available
    let mut keys = regmap.keys().collect::<Vec<_>>();
    keys.sort();
    for r in keys {}

    // final pass to piece together control codes
    for (i, inst) in instructs.iter().enumerate() {}

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

        let mut inst = AsmInstr::default();
        if sg.process_asm_line(&line, linenum, &mut inst) {
            // match an instruction
            inst.first = first || (inst.ctrl & 0x1_f800) == 0;
            first = false;
            inst.order = ordered;
            if ordered != 0 {
                ordered += 1;
            }
            inst.force_stall = if regex_match(r"FORCE", &inst.comment) {
                inst.ctrl & 0xf
            } else {
                0
            };
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
        let op = inst.op.clone();
        // cap_data can capture immutable references so create local copy
        let inst_str: String = inst.inst.clone();
        for gram in grammar[op.as_str()].iter() {
            let mut cap_data = HashMap::new();
            if !parse_instruct(&inst_str, &gram.rule, &mut cap_data) {
                continue;
            }
            let mut src: Vec<String> = Vec::new();
            // copy over instruction types for easier access
            // XXX this creates a new regex instance
            inst.itypes = gram.itype.clone().into();
            inst.dual = if inst.dual_cnt { 1 } else { 0 };
            src.push(inst.pred_reg.clone());
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
