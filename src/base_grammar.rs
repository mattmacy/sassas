 #![allow(non_upper_case_globals)]
use unsafe_lib::MutStrMap;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct InstrType {
    class: &'static str,
    lat: u8,
    blat: u8,
    rlat: u8,
    rhold: u8,
    tput: u8,
    dual: bool,
    reuse: bool,
}


#[derive(Clone, Debug)]
pub struct GrammarElt {
    itype: &'static InstrType,
    code: u64,
    rule: &'static str,
}

impl Default for GrammarElt {
    fn default() -> Self {
        GrammarElt {
            itype: &none,
            code: 0,
            rule: "",
        }
    }
}

static none: InstrType = InstrType {
    class: "",
    lat: 0,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 0,
    dual: false,
    reuse: false,
};
static s2rT: InstrType = InstrType {
    class: "s2r",
    lat: 2,
    blat: 25,
    rlat: 0,
    rhold: 0,
    tput: 1,
    dual: false,
    reuse: false,
};
static smemT: InstrType = InstrType {
    class: "mem",
    lat: 2,
    blat: 30,
    rlat: 2,
    rhold: 20,
    tput: 1,
    dual: true,
    reuse: false,
};
static gmemT: InstrType = InstrType {
    class: "mem",
    lat: 2,
    blat: 200,
    rlat: 4,
    rhold: 20,
    tput: 1,
    dual: true,
    reuse: false,
};
static x32T: InstrType = InstrType {
    class: "x32",
    lat: 6,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 1,
    dual: false,
    reuse: true,
};
static x64T: InstrType = InstrType {
    class: "x64",
    lat: 2,
    blat: 128,
    rlat: 0,
    rhold: 0,
    tput: 128,
    dual: false,
    reuse: true,
};
static shftT: InstrType = InstrType {
    class: "shift",
    lat: 6,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 2,
    dual: false,
    reuse: true,
};
static cmpT: InstrType = InstrType {
    class: "cmp",
    lat: 13,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 2,
    dual: false,
    reuse: true,
};
static qtrT: InstrType = InstrType {
    class: "qtr",
    lat: 8,
    blat: 0,
    rlat: 4,
    rhold: 0,
    tput: 1,
    dual: true,
    reuse: false,
};
static rroT: InstrType = InstrType {
    class: "rro",
    lat: 2,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 1,
    dual: false,
    reuse: false,
};
static voteT: InstrType = InstrType {
    class: "vote",
    lat: 2,
    blat: 0,
    rlat: 0,
    rhold: 0,
    tput: 1,
    dual: false,
    reuse: false,
};

fn hex(s: &str) -> u64 {
    unimplemented!();
    0
}

#[allow(non_snake_case)]
fn getP(val: &str, pos: usize) -> u64 {
    unimplemented!();
    0
}

#[allow(non_snake_case)]
fn getR(val: &str, pos: u32) -> u64 {
    unimplemented!();
    0
}

#[allow(non_snake_case)]
fn getC(val: &str) -> u64 {
    ((hex(val) >> 2) & 0x7fff) << 20
}

#[allow(non_snake_case)]
fn getF(val: &str, pos: u32, itype: char, trunc: u32) -> u64 {
    unimplemented!();
    0
}

#[allow(non_snake_case)]
fn getI(orig: &str, pos: u32, mask: u32) -> u64 {
    unimplemented!();
    0
}


pub fn build_operands() -> HashMap<&'static str, Box<Fn(&str) -> u64>> {
    let mut operands: HashMap<_, Box<Fn(&str) -> u64>> = HashMap::new();

    operands.insert("p0", Box::new(|s: &str| getP(s, 0)));
    operands.insert("p3", Box::new(|s: &str| getP(s, 3)));
    operands.insert("p12", Box::new(|s: &str| getP(s, 12)));
    operands.insert("p29", Box::new(|s: &str| getP(s, 29)));
    operands.insert("p39", Box::new(|s: &str| getP(s, 39)));
    operands.insert("p45", Box::new(|s: &str| getP(s, 45)));
    operands.insert("p48", Box::new(|s: &str| getP(s, 48)));
    operands.insert("p58", Box::new(|s: &str| getP(s, 58)));

    operands.insert("r0", Box::new(|s: &str| getR(s, 0)));
    operands.insert("r8", Box::new(|s: &str| getR(s, 8)));
    operands.insert("r20", Box::new(|s: &str| getR(s, 20)));
    operands.insert("r28", Box::new(|s: &str| getR(s, 28)));
    operands.insert("r39s20", Box::new(|s: &str| getR(s, 39)));
    operands.insert("r39", Box::new(|s: &str| getR(s, 39)));
    operands.insert("r39a", Box::new(|s: &str| getR(s, 39)));

    operands.insert("c20", Box::new(|s: &str| getC(s)));
    operands.insert("c39", Box::new(|s: &str| getC(s)));

    operands.insert("c34", Box::new(|s: &str| hex(s) << 34));
    operands.insert("c36", Box::new(|s: &str| hex(s) << 36));

    operands.insert("f20w32", Box::new(|s: &str| getF(s, 20, 'f', 0)));
    operands.insert("f20", Box::new(|s: &str| getF(s, 20, 'f', 12)));
    operands.insert("d20", Box::new(|s: &str| getF(s, 20, 'd', 44)));

    operands.insert("i8w4", Box::new(|s: &str| getI(s, 8, 0xf)));
    operands.insert("i20", Box::new(|s: &str| getI(s, 20, 0x7_ffff)));
    operands.insert("i20w6", Box::new(|s: &str| getI(s, 20, 0x3f)));
    operands.insert("i20w7", Box::new(|s: &str| getI(s, 20, 0x7f)));
    operands.insert("i20w8", Box::new(|s: &str| getI(s, 20, 0xff)));
    operands.insert("i20w12", Box::new(|s: &str| getI(s, 20, 0xfff)));
    operands.insert("i20w24", Box::new(|s: &str| getI(s, 20, 0xff_ffff)));
    operands.insert("i20w32", Box::new(|s: &str| getI(s, 20, 0xffff_ffff)));

    operands.insert("i31w4", Box::new(|s: &str| getI(s, 31, 0xf)));
    operands.insert("i34w13", Box::new(|s: &str| getI(s, 34, 0x1fff)));
    operands.insert("i36w20", Box::new(|s: &str| getI(s, 36, 0xf_ffff)));
    operands.insert("i39w8", Box::new(|s: &str| getI(s, 39, 0xff)));
    operands.insert("i28w8", Box::new(|s: &str| getI(s, 28, 0xff)));
    operands.insert("i28w20", Box::new(|s: &str| getI(s, 28, 0xf_ffff)));
    operands.insert("i48w20", Box::new(|s: &str| getI(s, 48, 0xff)));
    operands.insert("i51w5", Box::new(|s: &str| getI(s, 51, 0x1f)));
    operands.insert("i53w5", Box::new(|s: &str| getI(s, 53, 0x1f)));

    operands
}

pub fn build_grammar() -> MutStrMap<Vec<GrammarElt>> {
    let mut base_grammar = MutStrMap::new();

    base_grammar["ATOM"] = vec![
		GrammarElt {code : 0xed00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ATOM(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)(?:, (?^:(?<r39a>(?<r39>(?^:[a-zA-Z_]\w*)))(?<reuse3>\.reuse)?))?;)"#, itype : &gmemT		},
	];
    base_grammar["ATOMS"] = vec![
		GrammarElt {code : 0xec00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ATOMS(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)(?:, (?^:(?<r39a>(?<r39>(?^:[a-zA-Z_]\w*)))(?<reuse3>\.reuse)?))?;)"#, itype : &smemT		},
	];
    base_grammar["B2R"] = vec![
		GrammarElt {code : 0xf0b800010000ff00, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?B2R(?^:\.RESULT (?^:(?<r0>(?^:[a-zA-Z_]\w*)))(?:, (?^:(?<p45>(?^:P[0-6T])))|(?<nop45>)));)"#, itype : &x32T		},
	];
    base_grammar["BAR"] = vec![
		GrammarElt {code : 0xf0a8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BAR(?^:\.(?<mode>SYNC|ARV|RED)(?:\.(?<red>POPC|AND|OR))? (?:(?^:(?<i8w4>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?))(?:, (?:(?^:(?<i20w12>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)))?(?(<r20>)|(?<nor20>))(?(<red>), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])))|(?<nop39>)));)"#, itype : &gmemT		},
	];
    base_grammar["BFE"] = vec![
		GrammarElt {code : 0x5c01000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BFE(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT		},
	];
    base_grammar["BFI"] = vec![
		GrammarElt {code : 0x5bf0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BFI (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\])|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &shftT		},
	];
    base_grammar["BPT"] = vec![
		GrammarElt {code : 0xe3a00000000000c0, rule : r#"(?^:^(?^:(?<noPred>))?BPT\.TRAP (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["BRA"] = vec![
		GrammarElt {code : 0xe24000000000000f, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRA(?<U>\.U)? (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
		GrammarElt {code : 0xe240000000000002, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRA(?<U>\.U)? CC\.EQ, (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["BRK"] = vec![
        GrammarElt {
            code: 0xe34000000000000f,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRK;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["BRX"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRX[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CAL"] = vec![
		GrammarElt {code : 0xe260000000000040, rule : r#"(?^:^(?^:(?<noPred>))?CAL (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["CCTL"] = vec![
        GrammarElt {
            code: 0x5c88000000000000,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTL[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CCTLL"] = vec![
        GrammarElt {
            code: 0x5c88000000000000,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTLL[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CCTLT"] = vec![
        GrammarElt {
            code: 0x5c88000000000000,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTLT[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CONT"] = vec![
        GrammarElt {
            code: 0xe35000000000000f,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CONT;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CS2R"] = vec![
		GrammarElt {code : 0x50c8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CS2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:SR_(?<sr>\S+));)"#, itype : &x32T		},
	];
    base_grammar["CSET"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CSET[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["CSETP"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CSETP[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["DADD"] = vec![
		GrammarElt {code : 0x5c70000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DADD(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x64T		},
	];
    base_grammar["DEPBAR"] = vec![
		GrammarElt {code : 0xf0f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DEPBAR(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE)) (?^:(?<SB>SB0|SB1|SB2|SB3|SB4|SB5)), (?^:(?<i20w6>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &gmemT		},
		GrammarElt {code : 0xf0f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DEPBAR(?^: \{(?<db5>5)?,?(?<db4>4)?,?(?<db3>3)?,?(?<db2>2)?,?(?<db1>1)?,?(?<db0>0)?\});)"#, itype : &gmemT		},
	];
    base_grammar["DFMA"] = vec![
		GrammarElt {code : 0x5b70000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DFMA(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x64T		},
	];
    base_grammar["DMNMX"] = vec![
		GrammarElt {code : 0x5c50000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DMNMX (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["DMUL"] = vec![
		GrammarElt {code : 0x5c80000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DMUL(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x64T		},
	];
    base_grammar["DP2A"] = vec![
		GrammarElt {code : 0x53f9000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DP2A(?^:\.(?<mode>LO|HI))(?^:(?:\.(?<type1>U32|S32))?(?:\.(?<type2>U32|S32))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["DP4A"] = vec![
		GrammarElt {code : 0x53f8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DP4A(?^:(?:\.(?<type1>U32|S32))?(?:\.(?<type2>U32|S32))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["DSET"] = vec![
		GrammarElt {code : 0x5900000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DSET(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["DSETP"] = vec![
		GrammarElt {code : 0x5b80000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DSETP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["EXIT"] = vec![
        GrammarElt {
            code: 0xe30000000000000f,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?EXIT;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["F2F"] = vec![
		GrammarElt {code : 0x5ca8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?F2F(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?:\.(?<round>ROUND|FLOOR|CEIL|TRUNC))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT		},
	];
    base_grammar["F2I"] = vec![
		GrammarElt {code : 0x5cb0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?F2I(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<round>ROUND|FLOOR|CEIL|TRUNC))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT		},
	];
    base_grammar["FADD"] = vec![
		GrammarElt {code : 0x5c58000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FADD(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T		},
	];
    base_grammar["FADD32I"] = vec![
		GrammarElt {code : 0x800000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FADD32I(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?)));)"#, itype : &x32T		},
	];
    base_grammar["FCHK"] = vec![
		GrammarElt {code : 0x5c88000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FCHK\.DIVIDE (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["FCMP"] = vec![
		GrammarElt {code : 0x5ba0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FCMP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &cmpT		},
	];
    base_grammar["FFMA"] = vec![
		GrammarElt {code : 0x5980000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FFMA(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
		GrammarElt {code : 0x5980000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FFMA(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r39s20>(?<r20>(?^:[a-zA-Z_]\w*)))\|?(?:\.(?<r39part>H0|H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\]);)"#, itype : &x32T		},
	];
    base_grammar["FLO"] = vec![
		GrammarElt {code : 0x5c30000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FLO\.U32 (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &s2rT		},
	];
    base_grammar["FMNMX"] = vec![
		GrammarElt {code : 0x5c60000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMNMX(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT		},
	];
    base_grammar["FMUL"] = vec![
		GrammarElt {code : 0x5c68000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMUL(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T		},
	];
    base_grammar["FMUL32I"] = vec![
		GrammarElt {code : 0x1e00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMUL32I(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?)));)"#, itype : &x32T		},
	];
    base_grammar["FSET"] = vec![
		GrammarElt {code : 0x5800000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSET(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT		},
	];
    base_grammar["FSETP"] = vec![
		GrammarElt {code : 0x5bb0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSETP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["FSWZADD"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSWZADD[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["HADD2"] = vec![
		GrammarElt {code : 0x5d10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HADD2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["HFMA2"] = vec![
		GrammarElt {code : 0x5d00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HFMA2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["HMUL2"] = vec![
		GrammarElt {code : 0x5d08000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HMUL2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["HSETP2"] = vec![
		GrammarElt {code : 0x5d20000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HSETP2(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["I2F"] = vec![
		GrammarElt {code : 0x5cb8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?I2F(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT		},
	];
    base_grammar["I2I"] = vec![
		GrammarElt {code : 0x5ce0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?I2I(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT		},
	];
    base_grammar["IADD"] = vec![
		GrammarElt {code : 0x5c10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD(?^:(?<SAT>\.SAT)?)(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T		},
	];
    base_grammar["IADD3"] = vec![
		GrammarElt {code : 0x5cc0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD3(?^:(?:\.(?<type>X|RS|LS))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
	];
    base_grammar["IADD32I"] = vec![
		GrammarElt {code : 0x1c00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD32I(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["ICMP"] = vec![
		GrammarElt {code : 0x5b41000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ICMP(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &cmpT		},
	];
    base_grammar["IMAD"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMAD[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["IMADSP"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMADSP[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["IMNMX"] = vec![
		GrammarElt {code : 0x5c21000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMNMX(?^:(?<U32>\.U32)?)(?^:(?:\.(?<mode>XHI|XLO))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT		},
	];
    base_grammar["IMUL"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMUL[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["ISCADD"] = vec![
		GrammarElt {code : 0x5c18000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISCADD (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<i39w8>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
	];
    base_grammar["ISCADD32I"] = vec![
		GrammarElt {code : 0x1400000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISCADD32I (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))), (?^:(?<i53w5>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
	];
    base_grammar["ISET"] = vec![
		GrammarElt {code : 0x5b51000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISET(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?)(?^:(?<X>\.X)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT		},
	];
    base_grammar["ISETP"] = vec![
		GrammarElt {code : 0x5b61000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISETP(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?)(?^:(?<X>\.X)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["JCAL"] = vec![
		GrammarElt {code : 0xe220000000000040, rule : r#"(?^:^(?^:(?<noPred>))?JCAL (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["JMP"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?JMP[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["JMX"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?JMX[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["LD"] = vec![
		GrammarElt {code : 0x8000000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LD(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<p58>(?^:P[0-6T])));)"#, itype : &gmemT		},
	];
    base_grammar["LDC"] = vec![
		GrammarElt {code : 0xef90000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDC(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:c\[(?<c36>(?^:0[xX][0-9a-fA-F]+))\]\s*(?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]));)"#, itype : &gmemT		},
	];
    base_grammar["LDG"] = vec![
		GrammarElt {code : 0xeed0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDG(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &gmemT		},
	];
    base_grammar["LDL"] = vec![
		GrammarElt {code : 0xef40000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDL(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &gmemT		},
	];
    base_grammar["LDS"] = vec![
		GrammarElt {code : 0xef48000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDS(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &smemT		},
	];
    base_grammar["LEA"] = vec![
		GrammarElt {code : 0x5bd0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA (?^:(?<p48>(?^:P[0-6T]))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &cmpT		},
		GrammarElt {code : 0x5bd7000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<i39w8>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
		GrammarElt {code : 0x5bdf004000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA\.HI(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i28w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
		GrammarElt {code : 0xa07000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA\.HI(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i51w5>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
	];
    base_grammar["LOP"] = vec![
		GrammarElt {code : 0x5c40000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP(?^:\.(?<bool>AND|OR|XOR|PASS_B))(?^:(?:\.(?<z>NZ|Z) (?^:(?<p48>(?^:P[0-6T]))),|(?<noz>))) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?<INV>~)?(?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?))(?<INV>\.INV)?;)"#, itype : &x32T		},
	];
    base_grammar["LOP3"] = vec![
		GrammarElt {code : 0x5be7000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP3\.LUT (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i28w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
		GrammarElt {code : 0x3c00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP3\.LUT (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i48w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["LOP32I"] = vec![
		GrammarElt {code : 0x400000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP32I(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["MEMBAR"] = vec![
		GrammarElt {code : 0xef98000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MEMBAR(?^:\.(?<mode>CTA|GL|SYS));)"#, itype : &x32T		},
	];
    base_grammar["MOV"] = vec![
		GrammarElt {code : 0x5c98078000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MOV (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T		},
	];
    base_grammar["MOV32I"] = vec![
		GrammarElt {code : 0x10000000000f000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MOV32I (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?:(?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))));)"#, itype : &x32T		},
	];
    base_grammar["MUFU"] = vec![
		GrammarElt {code : 0x5080000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MUFU(?^:\.(?<func>COS|SIN|EX2|LG2|RCP|RSQ|RCP64H|RSQ64H)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?);)"#, itype : &qtrT		},
	];
    base_grammar["NOP"] = vec![
        GrammarElt {
            code: 0x50b0000000000f00,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?NOP;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["P2R"] = vec![
		GrammarElt {code : 0x38e8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?P2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), PR, (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w7>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["PBK"] = vec![
		GrammarElt {code : 0xe2a0000000000000, rule : r#"(?^:^(?^:(?<noPred>))?PBK (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["PCNT"] = vec![
		GrammarElt {code : 0xe2b0000000000000, rule : r#"(?^:^(?^:(?<noPred>))?PCNT (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["PEXIT"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PEXIT[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["POPC"] = vec![
		GrammarElt {code : 0x5c08000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?POPC (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &s2rT		},
	];
    base_grammar["PRET"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PRET[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["PRMT"] = vec![
		GrammarElt {code : 0x5bc0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PRMT(?^:(?:\.(?<mode>F4E|B4E|RC8|ECL|ECR|RC16))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\])|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &x32T		},
	];
    base_grammar["PSET"] = vec![
		GrammarElt {code : 0x5088000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PSET(?^:\.(?<bool2>AND|OR|XOR))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<p12not>\!)?(?<p12>(?^:P[0-6T]))), (?^:(?<p29not>\!)?(?<p29>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["PSETP"] = vec![
		GrammarElt {code : 0x5090000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PSETP(?^:\.(?<bool2>AND|OR|XOR))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<p12not>\!)?(?<p12>(?^:P[0-6T]))), (?^:(?<p29not>\!)?(?<p29>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT		},
	];
    base_grammar["R2B"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?R2B[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["R2P"] = vec![
		GrammarElt {code : 0x38f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?R2P PR, (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w7>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT		},
	];
    base_grammar["RED"] = vec![
		GrammarElt {code : 0xebf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RED(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT		},
	];
    base_grammar["RET"] = vec![
        GrammarElt {
            code: 0xe32000000000000f,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RET;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["RRO"] = vec![
		GrammarElt {code : 0x5c90000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RRO(?^:\.(?<func>SINCOS|EX2)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &rroT		},
	];
    base_grammar["S2R"] = vec![
		GrammarElt {code : 0xf0c8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?S2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:SR_(?<sr>\S+));)"#, itype : &s2rT		},
	];
    base_grammar["SEL"] = vec![
		GrammarElt {code : 0x5ca0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SEL (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &x32T		},
	];
    base_grammar["SHF"] = vec![
		GrammarElt {code : 0x5bf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHF\.L(?^:(?<W>\.W)?(?:\.(?<type>U64|S64))?(?<HI>\.HI)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
		GrammarElt {code : 0x5cf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHF\.R(?^:(?<W>\.W)?(?:\.(?<type>U64|S64))?(?<HI>\.HI)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["SHFL"] = vec![
		GrammarElt {code : 0xef10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHFL(?^:\.(?<mode>IDX|UP|DOWN|BFLY)) (?^:(?<p48>(?^:P[0-6T]))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?:(?^:(?<i20w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?:(?^:(?<i34w13>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &smemT		},
	];
    base_grammar["SHL"] = vec![
		GrammarElt {code : 0x5c48000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHL(?<W>\.W)? (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT		},
	];
    base_grammar["SHR"] = vec![
		GrammarElt {code : 0x5c29000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHR(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT		},
	];
    base_grammar["SSY"] = vec![
		GrammarElt {code : 0xe290000000000000, rule : r#"(?^:^(?^:(?<noPred>))?SSY (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T		},
	];
    base_grammar["ST"] = vec![
		GrammarElt {code : 0xa000000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ST(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<p58>(?^:P[0-6T])));)"#, itype : &gmemT		},
	];
    base_grammar["STG"] = vec![
		GrammarElt {code : 0xeed8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STG(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT		},
	];
    base_grammar["STL"] = vec![
		GrammarElt {code : 0xef50000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STL(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT		},
	];
    base_grammar["STS"] = vec![
		GrammarElt {code : 0xef58000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STS(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &smemT		},
	];
    base_grammar["SUATOM"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SUATOM[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["SULD"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SULD[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["SURED"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SURED[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["SUST"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SUST[^;]*;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["SYNC"] = vec![
        GrammarElt {
            code: 0xf0f800000000000f,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SYNC;)"#,
            itype: &x32T,
        },
    ];
    base_grammar["TEX"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TEX[^;]*;)"#,
            itype: &gmemT,
        },
    ];
    base_grammar["TEXS"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TEXS[^;]*;)"#,
            itype: &gmemT,
        },
    ];
    base_grammar["TLD"] = vec![
		GrammarElt {code : 0xdd38000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD\.B\.LZ\.(?^:(?<NODEP>NODEP\.)?(?:(?<reuse1>T)|(?<reuse2>P))) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:0[xX][0-9a-fA-F]+), \dD, (?^:(?<i31w4>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &gmemT		},
	];
    base_grammar["TLD4"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD4[^;]*;)"#,
            itype: &gmemT,
        },
    ];
    base_grammar["TLD4S"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD4S[^;]*;)"#,
            itype: &gmemT,
        },
    ];
    base_grammar["TLDS"] = vec![
		GrammarElt {code : 0xda0000000ff00000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLDS\.LZ\.(?^:(?<NODEP>NODEP\.)?(?:(?<reuse1>T)|(?<reuse2>P))) (?^:(?<r28>(?^:[a-zA-Z_]\w*))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i36w20>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))), \dD, (?^:(?<chnls>R|RGBA));)"#, itype : &gmemT		},
	];
    base_grammar["TXQ"] = vec![
        GrammarElt {
            code: 0x0,
            rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TXQ[^;]*;)"#,
            itype: &gmemT,
        },
    ];
    base_grammar["VABSDIFF"] = vec![
		GrammarElt {code : 0x5427000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VABSDIFF(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["VADD"] = vec![
		GrammarElt {code : 0x2044000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VADD(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["VMAD"] = vec![
		GrammarElt {code : 0x5f04000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMAD(?^:\.(?<sign1>[SU])(?<size1>16)\.(?<sign2>[SU])(?<size2>16)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
		GrammarElt {code : 0x5f04000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMAD(?^:\.(?<sign1>[SU])(?<size1>8|16)\.(?<sign2>[SU])(?<size2>8|16)(?<PO>\.PO)?(?<SHR_7>\.SHR_7)?(?<SHR_15>\.SHR_15)?(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["VMNMX"] = vec![
		GrammarElt {code : 0x3a44000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMNMX(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?:\.(?<MX>MX))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["VOTE"] = vec![
		GrammarElt {code : 0x50d8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VOTE(?^:\.(?<mode>ALL|ANY|EQ)) (?:(?^:(?<r0>(?^:[a-zA-Z_]\w*))), |(?<nor0>))(?^:(?<p45>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &voteT		},
	];
    base_grammar["VSET"] = vec![
		GrammarElt {code : 0x4004000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VSET(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT		},
	];
    base_grammar["XMAD"] = vec![
		GrammarElt {code : 0x5b00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<mode>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
		GrammarElt {code : 0x5900000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<mode>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r39s20>(?<r20>(?^:[a-zA-Z_]\w*)))\|?(?:\.(?<r39part>H0|H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\]);)"#, itype : &x32T		},
		GrammarElt {code : 0x5e00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<modec>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20partx>H0|H1|B0|B1|B2|B3))?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T		},
	];

    base_grammar
}
