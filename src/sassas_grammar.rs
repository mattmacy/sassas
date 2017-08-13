 #![allow(non_upper_case_globals)]
use std::collections::HashMap;
use utils::{MutMap, MutStrMap, regex_strip, regex_matches, regex_match, regex_replace, re_matches,
            re_match_names, SVal};
use regex::Regex;

static flagsstr: &str = "\
    BFE, BFI, FLO, IADD, IADD3, ICMP, IMNMX, ISCADD, ISET, ISETP, LEA, LOP, LOP3, MOV, PRMT, SEL, SHF, SHL, SHR, XMAD\
    0x0100000000000000 neg\
    \
    FADD, FCMP, FFMA, FMNMX, FMUL, FSET, FSETP, DADD, DFMA, DMNMX, DMUL, DSET, DSETP\
    0x0100000000000000 neg\
    \
    PSET, PSETP\
    0x0000000000008000 p12not\
    0x0000000100000000 p29not\
    \
    FMNMX, FSET, FSETP, DMNMX, DSET, DSETP, IMNMX, ISET, ISETP, SEL, PSET, PSETP, BAR, VOTE\
    0x0000040000000000 p39not\
    \
    IADD, IADD3, XMAD, LEA, IMNMX\
    0x0000800000000000 CC\
    \
    IADD32I\
    0x0010000000000000 CC\
    \
    LEA\
    0x0000000000000000 X\
    \
    SHF\
    0x0004000000000000 W\
    0x0001000000000000 HI\
    \
    SHF: type\
    0x0000004000000000 U64\
    0x0000006000000000 S64\
    \
    SHR, IMNMX, ISETP, ISET, ICMP, BFE\
    0x0001000000000000 U32\
    \
    SHL\
    0x0000008000000000 W\
    \
    SHFL\
    0x0000000010000000 i20w8\
    0x0000000020000000 i34w13\
    \
    SHFL: mode\
    0x0000000000000000 IDX\
    0x0000000040000000 UP\
    0x0000000080000000 DOWN\
    0x00000000c0000000 BFLY\
    \
    IMNMX: mode\
    0x0000080000000000 XLO\
    0x0000180000000000 XHI\
    \
    ISETP, ISET, ICMP: cmp\
    0x0002000000000000 LT\
    0x0004000000000000 EQ\
    0x0006000000000000 LE\
    0x0008000000000000 GT\
    0x000a000000000000 NE\
    0x000c000000000000 GE\
    \
    ISETP, ISET, PSETP, PSET: bool\
    0x0000000000000000 AND\
    0x0000200000000000 OR\
    0x0000400000000000 XOR\
    \
    PSETP, PSET: bool2\
    0x0000000000000000 AND\
    0x0000000001000000 OR\
    0x0000000002000000 XOR\
    \
    ISETP, ISET\
    0x0000080000000000 X\
    \
    LOP: bool\
    0x0000000000000000 AND\
    0x0000020000000000 OR\
    0x0000040000000000 XOR\
    0x0000060000000000 PASS_B\
    \
    LOP:\
    0x0000010000000000 INV\
    \
    LOP: z\
    0x0000200000000000 Z\
    0x0000300000000000 NZ\
    \
    LOP\
    0x0007000000000000 noz\
    \
    LOP32I: bool\
    0x0000000000000000 AND\
    0x0020000000000000 OR\
    0x0040000000000000 XOR\
    \
    PRMT: mode\
    0x0001000000000000 F4E\
    0x0002000000000000 B4E\
    0x0003000000000000 RC8\
    0x0004000000000000 ECL\
    0x0005000000000000 ECR\
    0x0006000000000000 RC16\
    \
    XMAD: type1\
    0x0000000000000000 U16\
    0x0001000000000000 S16\
    \
    XMAD: type2\
    0x0000000000000000 U16\
    0x0002000000000000 S16\
    \
    XMAD: mode\
    0x0000002000000000 MRG\
    0x0000001000000000 PSL\
    0x0008000000000000 CHI\
    0x0004000000000000 CLO\
    0x000c000000000000 CSFU\
    0x0004001000000000 PSL.CLO\
    \
    XMAD: modec\
    0x0004000000000000 CLO\
    0x0008000000000000 CHI\
    0x000c000000000000 CSFU\
    0x0040000000000000 X\
    0x0080000000000000 PSL\
    0x0100000000000000 MRG\
    0x0084000000000000 PSL.CLO\
    \
    XMAD\
    0x0010000000000000 CBCC\
    \
    XMAD: r8part\
    0x0000000000000000 H0\
    0x0020000000000000 H1\
    \
    XMAD: r20part\
    0x0000000000000000 H0\
    0x0000000800000000 H1\
    \
    XMAD: r20partx\
    0x0000000000000000 H0\
    0x0010000000000000 H1\
    \
    XMAD: r39part\
    0x0000000000000000 H0\
    0x0010000000000000 H1\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: r8part\
    0x0000000000000000 B0\
    0x0000001000000000 B1\
    0x0000002000000000 B2\
    0x0000003000000000 B3\
    0x0000001000000000 H1\
    0x0000000000000000 H0\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: r20part\
    0x0000000000000000 B0\
    0x0000000010000000 B1\
    0x0000000020000000 B2\
    0x0000000030000000 B3\
    0x0000000010000000 H1\
    0x0000000000000000 H0\
    \
    VMAD\
    0x0040000000000000 r8neg\
    0x0020000000000000 r39neg\
    0x0008000000000000 SHR_7\
    0x0010000000000000 SHR_15\
    0x0060000000000000 PO\
    0x0080000000000000 SAT\
    \
    VMNMX\
    0x0100000000000000 MX\
    \
    VADD, VABSDIFF, VMNMX\
    0x0080000000000000 SAT\
    0x0040000000000000 UD\
    0x0040000000000000 SD\
    \
    VSET: cmp\
    0x0040000000000000 LT\
    0x0080000000000000 EQ\
    0x00c0000000000000 LE\
    0x0100000000000000 GT\
    0x0140000000000000 NE\
    0x0180000000000000 GE\
    \
    VADD, VSET: mode\
    0x0020000000000000 ACC\
    0x0028000000000000 MIN\
    0x0030000000000000 MAX\
    0x0000000000000000 MRG_16H\
    0x0008000000000000 MRG_16L\
    0x0010000000000000 MRG_8B0\
    0x0000000000000000 MRG_8B1\
    0x0018000000000000 MRG_8B2\
    0x0000000000000000 MRG_8B3\
    \
    VABSDIFF: mode\
    0x0003000000000000 ACC\
    0x000b000000000000 MIN\
    0x0013000000000000 MAX\
    0x0023000000000000 MRG_16H\
    0x002b000000000000 MRG_16L\
    0x0033000000000000 MRG_8B0\
    0x0000000000000000 MRG_8B1\
    0x003b000000000000 MRG_8B2\
    0x0000000000000000 MRG_8B3\
    \
    VMNMX: mode\
    0x0020000000000000 ACC\
    0x0028000000000000 MIN\
    0x0030000000000000 MAX\
    0x0000000000000000 MRG_16H\
    0x0008000000000000 MRG_16L\
    0x0010000000000000 MRG_8B0\
    0x0000000000000000 MRG_8B1\
    0x0018000000000000 MRG_8B2\
    0x0000000000000000 MRG_8B3\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: sign1\
    0x0000000000000000 U\
    0x0001000000000000 S\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: sign2\
    0x0000000000000000 U\
    0x0002000000000000 S\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: size1\
    0x0000000000000000 8\
    0x0000004000000000 16\
    0x0000006000000000 32\
    \
    VMAD, VADD, VABSDIFF, VMNMX, VSET: size2\
    0x0000000000000000 8\
    0x0000000040000000 16\
    0x0000000060000000 32\
    \
    IADD3: type\
    0x0001000000000000 X\
    0x0000002000000000 RS\
    0x0000004000000000 LS\
    \
    IADD3: r8part\
    0x0000000000000000 H0\
    0x0000001000000000 H1\
    \
    IADD3: r20part\
    0x0000000080000000 H0\
    \
    IADD3: r39part\
    0x0000000200000000 H0\
    \
    IADD3\
    0x0008000000000000 r8neg\
    0x0004000000000000 r20neg\
    0x0002000000000000 r39neg\
    \
    IADD\
    0x0000080000000000 X\
    0x0004000000000000 SAT\
    \
    IADD, ISCADD\
    0x0002000000000000 r8neg\
    0x0001000000000000 r20neg\
    \
    IADD32I\
    0x0100000000000000 r8neg\
    0x0020000000000000 X\
    \
    DEPBAR: SB\
    0x0000000000000000 SB0\
    0x0000000004000000 SB1\
    0x0000000008000000 SB2\
    0x000000000c000000 SB3\
    0x0000000010000000 SB4\
    0x0000000014000000 SB5\
    \
    DEPBAR: cmp\
    0x0000000020000000 LE\
    \
    DEPBAR\
    0x0000000000000001 db0\
    0x0000000000000002 db1\
    0x0000000000000004 db2\
    0x0000000000000008 db3\
    0x0000000000000010 db4\
    0x0000000000000020 db5\
    \
    F2F, F2I, I2F, I2I: destWidth\
    0x0000000000000000 8\
    0x0000000000000100 16\
    0x0000000000000200 32\
    0x0000000000000300 64\
    \
    F2F, F2I, I2F, I2I: srcWidth\
    0x0000000000000000 8\
    0x0000000000000400 16\
    0x0000000000000800 32\
    0x0000000000000c00 64\
    \
    F2F, F2I, I2F, I2I: destSign\
    0x0000000000000000 F\
    0x0000000000000000 U\
    0x0000000000001000 S\
    \
    F2F, F2I, I2F, I2I: srcSign\
    0x0000000000000000 F\
    0x0000000000000000 U\
    0x0000000000002000 S\
    \
    F2I, I2F, I2I: r20part\
    0x0000000000000000 H0\
    0x0000040000000000 H1\
    0x0000000000000000 B0\
    0x0000020000000000 B1\
    0x0000040000000000 B2\
    0x0000060000000000 B3\
    \
    F2F: r20part\
    0x0000000000000000 H0\
    0x0000020000000000 H1\
    \
    F2F: round\
    0x0000040000000000 ROUND\
    0x0000048000000000 FLOOR\
    0x0000050000000000 CEIL\
    0x0000058000000000 TRUNC\
    \
    F2I: round\
    0x0000000000000000 ROUND\
    0x0000008000000000 FLOOR\
    0x0000010000000000 CEIL\
    0x0000018000000000 TRUNC\
    \
    HADD2, HMUL2, HFMA2: r8part\
    0x0001000000000000 H0_H0\
    0x0001800000000000 H1_H1\
    0x0000800000000000 F32\
    \
    HADD2, HMUL2, HFMA2: r20part\
    0x0000000020000000 H0_H0\
    0x0000000030000000 H1_H1\
    \
    HFMA2: r39part\
    0x0000000800000000 F32\
    0x0000001000000000 H0_H0\
    0x0000001800000000 H1_H1\
    \
    HADD2, HMUL2, HFMA2\
    0x0000000080000000 r20neg\
    0x0000000040000000 r39neg\
    \
    HADD2, HMUL2, HFMA2: mode\
    0x0002000000000000 F32\
    0x0004000000000000 MRG_H0\
    0x0006000000000000 MRG_H1\
    \
    HADD2, HMUL2\
    0x0000008000000000 FTZ\
    \
    HFMA2\
    0x0000002000000000 FTZ\
    \
    HFMA2\
    0x0000004000000000 FMZ\
    \
    HADD2, HMUL2, HFMA2\
    0x0000000100000000 SAT\
    \
    FADD, DADD, FMUL, DMUL, F2F, I2F: rnd\
    0x0000000000000000 RN\
    0x0000008000000000 RM\
    0x0000010000000000 RP\
    0x0000018000000000 RZ\
    \
    DFMA: rnd\
    0x0000000000000000 RN\
    0x0004000000000000 RM\
    0x0008000000000000 RP\
    0x000c000000000000 RZ\
    \
    FFMA: rnd\
    0x0000000000000000 RN\
    0x0008000000000000 RM\
    0x0010000000000000 RP\
    0x0018000000000000 RZ\
    \
    FFMA\
    0x0020000000000000 FTZ\
    \
    F2F, F2I, FADD, FMUL, FMNMX\
    0x0000100000000000 FTZ\
    \
    FADD32I\
    0x0080000000000000 FTZ\
    \
    FMUL32I\
    0x0020000000000000 FTZ\
    \
    FSET\
    0x0080000000000000 FTZ\
    \
    FSETP, FCMP\
    0x0000800000000000 FTZ\
    \
    FADD, FFMA, FMUL, F2F, I2I\
    0x0004000000000000 SAT\
    \
    FADD, DADD, FMNMX, DMNMX, MUFU\
    0x0001000000000000 r8neg\
    \
    FADD, DADD, FMNMX, DMNMX, RRO, F2F, F2I, I2F, I2I\
    0x0000200000000000 r20neg\
    \
    FMUL, DMUL, FFMA, DFMA\
    0x0001000000000000 r20neg\
    \
    FFMA, DFMA\
    0x0002000000000000 r39neg\
    \
    FADD, DADD, FMNMX, DMNMX\
    0x0000400000000000 r8abs\
    \
    FADD, DADD, FMNMX, DMNMX, F2F, F2I, I2F, I2I\
    0x0002000000000000 r20abs\
    \
    FSETP, DSETP, FSET, DSET\
    0x0000080000000000 r8neg\
    0x0000000000000040 r20neg\
    0x0000000000000080 r8abs\
    0x0000100000000000 r20abs\
    \
    RRO: func\
    0x0000000000000000 SINCOS\
    0x0000008000000000 EX2\
    \
    MUFU: func\
    0x0000000000000000 COS\
    0x0000000000100000 SIN\
    0x0000000000200000 EX2\
    0x0000000000300000 LG2\
    0x0000000000400000 RCP\
    0x0000000000500000 RSQ\
    0x0000000000600000 RCP64H\
    0x0000000000700000 RSQ64H\
    \
    FSETP, DSETP, FSET, DSET, FCMP: cmp\
    0x0001000000000000 .LT\
    0x0002000000000000 .EQ\
    0x0003000000000000 .LE\
    0x0004000000000000 .GT\
    0x0004000000000000\
    0x0005000000000000 .NE\
    0x0006000000000000 .GE\
    0x0007000000000000 .NUM\
    0x0008000000000000 .NAN\
    0x0009000000000000 .LTU\
    0x000a000000000000 .EQU\
    0x000b000000000000 .LEU\
    0x000c000000000000 .GTU\
    0x000d000000000000 .NEU\
    0x000e000000000000 .GEU\
    \
    FSETP, DSETP, FSET, DSET: bool\
    0x0000000000000000 AND\
    0x0000200000000000 OR\
    0x0000400000000000 XOR\
    \
    HSETP2: cmp\
    0x0000002800000000 .NE\
    \
    HSETP2: bool\
    0x0000000000000000 AND\
    \
    S2R: sr\
    0x0000000000000000 LANEID\
    0x0000000000200000 VIRTCFG\
    0x0000000000300000 VIRTID\
    0x0000000002100000 TID.X\
    0x0000000002200000 TID.Y\
    0x0000000002300000 TID.Z\
    0x0000000002500000 CTAID.X\
    0x0000000002600000 CTAID.Y\
    0x0000000002700000 CTAID.Z\
    0x0000000003800000 EQMASK\
    0x0000000003900000 LTMASK\
    0x0000000003a00000 LEMASK\
    0x0000000003b00000 GTMASK\
    0x0000000003c00000 GEMASK\
    \
    CS2R: sr\
    0x0000000005000000 CLOCKLO\
    0x0000000005100000 CLOCKHI\
    0x0000000005200000 GLOBALTIMERLO\
    0x0000000005300000 GLOBALTIMERHI\
    \
    B2R\
    0x0000e00000000000 nop45\
    \
    BAR\
    0x0000100000000000 i8w4\
    0x0000080000000000 nor20\
    0x0000038000000000 nop39\
    \
    BAR: mode\
    0x0000000000000000 SYNC\
    0x0000000100000000 ARV\
    0x0000000200000000 RED\
    \
    BAR: red\
    0x0000000000000000 POPC\
    0x0000000800000000 AND\
    0x0000001000000000 OR\
    \
    MEMBAR: mode\
    0x0000000000000000 CTA\
    0x0000000000000100 GL\
    0x0000000000000200 SYS\
    \
    VOTE: mode\
    0x0000000000000000 ALL\
    0x0001000000000000 ANY\
    0x0002000000000000 EQ\
    \
    VOTE\
    0x00000000000000ff nor0\
    \
    BRA\
    0x0000000000000080 U\
    \
    TLDS: chnls\
    0x0010000000000000 RGBA\
    \
    TLDS\
    0x0002000000000000 NODEP\
    \
    LD, ST, LDG, STG, LDS, STS, LDL, STL, LDC, RED, ATOM, ATOMS\
    0x000000000000ff00 nor8\
    \
    LD, ST: type\
    0x0000000000000000 .U8\
    0x0020000000000000 .S8\
    0x0040000000000000 .U16\
    0x0060000000000000 .S16\
    0x0080000000000000\
    0x0080000000000000 .32\
    0x00a0000000000000 .64\
    0x00c0000000000000 .128\
    \
    LD, ST: cache\
    0x0100000000000000 CG\
    0x0200000000000000 CS\
    0x0300000000000000 CV\
    0x0300000000000000 WT\
    \
    LDG, STG, LDS, STS, LDL, STL, LDC: type\
    0x0000000000000000 .U8\
    0x0001000000000000 .S8\
    0x0002000000000000 .U16\
    0x0003000000000000 .S16\
    0x0004000000000000\
    0x0004000000000000 .32\
    0x0005000000000000 .64\
    0x0006000000000000 .128\
    \
    LDG, STG: cache\
    0x0000400000000000 CG\
    0x0000800000000000 CI\
    0x0000800000000000 CS\
    0x0000c00000000000 CV\
    0x0000c00000000000 WT\
    \
    LDL: cache\
    0x0000200000000000 CI\
    \
    LDC: cache\
    0x0000100000000000 IL\
    \
    LDG, STG, LDS, STS, LDL, STL, LDC\
    0x0000200000000000 E\
    \
    LDS\
    0x0000100000000000 U\
    \
    RED: type\
    0x0000000000000000\
    0x0000000000100000 .S32\
    0x0000000000200000 .U64\
    0x0000000000300000 .F32.FTZ.RN\
    0x0000000000400000 .F16x2.FTZ.RN\
    0x0000000000500000 .S64\
    \
    RED: mode\
    0x0000000000000000 ADD\
    0x0000000000800000 MIN\
    0x0000000001000000 MAX\
    0x0000000001800000 INC\
    0x0000000002000000 DEC\
    0x0000000002800000 AND\
    0x0000000003000000 OR\
    0x0000000003800000 XOR\
    \
    ATOM: type\
    0x0000000000000000\
    0x0002000000000000 .S32\
    0x0004000000000000 .U64\
    0x0006000000000000 .F32.FTZ.RN\
    0x0008000000000000 .F16x2.FTZ.RN\
    0x000a000000000000 .S64\
    0x0002000000000000 .64\
    \
    ATOM, RED\
    0x0001000000000000 E\
    \
    ATOM: mode\
    0x0000000000000000 ADD\
    0x0010000000000000 MIN\
    0x0020000000000000 MAX\
    0x0030000000000000 INC\
    0x0040000000000000 DEC\
    0x0050000000000000 AND\
    0x0060000000000000 OR\
    0x0070000000000000 XOR\
    0x0080000000000000 EXCH\
    0x03f0000000000000 CAS\
    \
    ATOMS: type\
    0x0000000000000000\
    0x0000000010000000 .S32\
    0x0000000020000000 .U64\
    0x0000000030000000 .S64\
    0x0010000000000000 .64\
    \
    ATOMS: mode\
    0x0000000000000000 ADD\
    0x0010000000000000 MIN\
    0x0020000000000000 MAX\
    0x0030000000000000 INC\
    0x0040000000000000 DEC\
    0x0050000000000000 AND\
    0x0060000000000000 OR\
    0x0070000000000000 XOR\
    0x0080000000000000 EXCH\
    0x0240000000000000 CAS\
    \
    DP4A, DP2A: type1\
    0x0000000000000000 U32\
    0x0002000000000000 S32\
    \
    DP4A, DP2A: type2\
    0x0000000000000000 U32\
    0x0000800000000000 S32\
    \
    DP2A: mode\
    0x0000000000000000 LO\
    0x0004000000000000 HI\
";


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
pub struct GrammarEltBase {
    itype: &'static InstrType,
    code: u64,
    rule: &'static str,
}
impl Default for GrammarEltBase {
    fn default() -> Self {
        GrammarEltBase {
            itype: &none,
            code: 0,
            rule: "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct GrammarElt {
    itype: &'static InstrType,
    code: u64,
    rule: Regex,
}
impl<'a> From<&'a GrammarEltBase> for GrammarElt {
    fn from(src: &'a GrammarEltBase) -> Self {
        GrammarElt {
            itype: src.itype,
            code: src.code,
            rule: Regex::new(src.rule).unwrap(),
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
    match s.parse() {
        Ok(val) => val,
        Err(err) => panic!("err: {}", err),
    }
}

#[allow(non_snake_case)]
fn getP(val: &str, pos: usize) -> u64 {
    let matches = regex_matches(r"^P(\d+|T)$", val);
    if matches.len() == 0 {
        panic!("Bad predicate name found: {}", val);
    }
    let p = &matches[0][1];
    if p == "T" { 7 << pos } else { hex(p) << pos }
}

#[allow(non_snake_case)]
fn getR(val: &str, pos: u32) -> u64 {
    let caps = regex_matches(r"^R(\d+|Z)$", val);
    if caps.len() == 0 || hex(&caps[0][1]) >= 255 {
        panic!("Bad register name found: {}", val);
    }
    if hex(&caps[0][1]) >= 255 {
        panic!("Bad register name found: {}", val);
    }
    let val = hex(&caps[0][1]);
    if val == 'Z' as u64 {
        0xff << pos
    } else {
        val << pos
    }
}

#[allow(non_snake_case)]
fn getC(val: &str) -> u64 {
    ((hex(val) >> 2) & 0x7fff) << 20
}

#[allow(non_snake_case)]
fn getF(val: &str, pos: u32, itype: char, trunc: u32) -> u64 {
    let val = if regex_match(r"^0x[0-9a-zA-Z]+", val) {
        hex(val)
    } else if regex_match(r"INF", val) {
        if trunc == 0 {
            0x7f800000
        } else if itype == 'f' {
            0x7f800
        } else {
            0x7ff00
        }
    } else {
        /* XXX -- what is the end result of unpacking as 'L' vs 'Q' ? */
        let val = hex(val);
        if trunc != 0 {
            (val >> trunc) & 0x7ffff
        } else {
            val
        }
    };
    val << pos
}

/* XXX -- revisit WRT sign extension handling XXX */
#[allow(non_snake_case)]
fn getI(orig: &str, pos: u32, mask: i64) -> u64 {
    let neg = regex_match(r"^\-", orig);

    let val = if neg {
        regex_strip(r"-", orig)
    } else {
        orig.into()
    };
    let val = if regex_match(r"^(\d+)[xX]<([^>]+)>", &val) {
        panic!(" implement dyon parsing!");
    /*
        # allow any perl expression and multiply result by leading decimal.
        # also allow global scalar varibles in the expression.
        my $mul = $1;
        my $exp = $2;
        # strip leading zeros (don't interpret numbers as octal)
        $exp =~ s/(?<!\d)0+(?=[1-9])//g;
        my @globals = $exp =~ m'\$\w+'g;
        my $our = @globals ? ' our (' . join(',',@globals) . ');' : '';
        $val = $mul * eval "package MaxAs::MaxAs::CODE;$our $exp";
        */
    } else if regex_match(r"^0x[0-9a-zA-Z]+", &val) {
        hex(&val) as i64
    } else {
        panic!("bad immediate {}", orig);
    };
    let val = if neg {
        (-val & mask)
    } else {
        if val & mask != val {
            panic!(
                "Immediate value out of range(0x{:x}): 0x{:x} ({})",
                mask,
                val,
                orig
            );
        }
        val
    };
    (val << pos) as u64
}


pub fn build_operands<'a>() -> HashMap<&'a str, Box<Fn(&str) -> u64>> {
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

pub fn build_grammar() -> HashMap<&'static str, Vec<GrammarElt>> {
    let mut base_grammar = MutMap::new();

    base_grammar.insert("ATOM", vec![
        GrammarEltBase {code : 0xed00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ATOM(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)(?:, (?^:(?<r39a>(?<r39>(?^:[a-zA-Z_]\w*)))(?<reuse3>\.reuse)?))?;)"#, itype : &gmemT        },
    ]);
    base_grammar.insert("ATOMS", vec![
        GrammarEltBase {code : 0xec00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ATOMS(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)(?:, (?^:(?<r39a>(?<r39>(?^:[a-zA-Z_]\w*)))(?<reuse3>\.reuse)?))?;)"#, itype : &smemT       },
    ]);
    base_grammar.insert("B2R", vec![
        GrammarEltBase {code : 0xf0b800010000ff00, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?B2R(?^:\.RESULT (?^:(?<r0>(?^:[a-zA-Z_]\w*)))(?:, (?^:(?<p45>(?^:P[0-6T])))|(?<nop45>)));)"#, itype : &x32T        },
    ]);
    base_grammar.insert("BAR", vec![
        GrammarEltBase {code : 0xf0a8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BAR(?^:\.(?<mode>SYNC|ARV|RED)(?:\.(?<red>POPC|AND|OR))? (?:(?^:(?<i8w4>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?))(?:, (?:(?^:(?<i20w12>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)))?(?(<r20>)|(?<nor20>))(?(<red>), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])))|(?<nop39>)));)"#, itype : &gmemT     },
    ]);
    base_grammar.insert("BFE", vec![
        GrammarEltBase {code : 0x5c01000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BFE(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT        },
    ]);
    base_grammar.insert("BFI", vec![
        GrammarEltBase {code : 0x5bf0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BFI (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\])|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &shftT      },
    ]);
    base_grammar.insert("BPT", vec![
        GrammarEltBase {code : 0xe3a00000000000c0, rule : r#"(?^:^(?^:(?<noPred>))?BPT\.TRAP (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T     },
    ]);
    base_grammar.insert("BRA", vec![
        GrammarEltBase {code : 0xe24000000000000f, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRA(?<U>\.U)? (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T        },
        GrammarEltBase {code : 0xe240000000000002, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRA(?<U>\.U)? CC\.EQ, (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T        },
    ]);
    base_grammar.insert(
        "BRK",
        vec![
            GrammarEltBase {
                code: 0xe34000000000000f,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRK;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "BRX",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?BRX[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("CAL", vec![
        GrammarEltBase {code : 0xe260000000000040, rule : r#"(?^:^(?^:(?<noPred>))?CAL (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T       },
    ]);
    base_grammar.insert(
        "CCTL",
        vec![
            GrammarEltBase {
                code: 0x5c88000000000000,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTL[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "CCTLL",
        vec![
            GrammarEltBase {
                code: 0x5c88000000000000,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTLL[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "CCTLT",
        vec![
            GrammarEltBase {
                code: 0x5c88000000000000,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CCTLT[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "CONT",
        vec![
            GrammarEltBase {
                code: 0xe35000000000000f,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CONT;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("CS2R", vec![
        GrammarEltBase {code : 0x50c8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CS2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:SR_(?<sr>\S+));)"#, itype : &x32T      },
    ]);
    base_grammar.insert(
        "CSET",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CSET[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "CSETP",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?CSETP[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("DADD", vec![
        GrammarEltBase {code : 0x5c70000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DADD(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x64T     },
    ]);
    base_grammar.insert("DEPBAR", vec![
        GrammarEltBase {code : 0xf0f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DEPBAR(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE)) (?^:(?<SB>SB0|SB1|SB2|SB3|SB4|SB5)), (?^:(?<i20w6>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &gmemT     },
        GrammarEltBase {code : 0xf0f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DEPBAR(?^: \{(?<db5>5)?,?(?<db4>4)?,?(?<db3>3)?,?(?<db2>2)?,?(?<db1>1)?,?(?<db0>0)?\});)"#, itype : &gmemT     },
    ]);
    base_grammar.insert("DFMA", vec![
        GrammarEltBase {code : 0x5b70000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DFMA(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x64T        },
    ]);
    base_grammar.insert("DMNMX", vec![
        GrammarEltBase {code : 0x5c50000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DMNMX (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT      },
    ]);
    base_grammar.insert("DMUL", vec![
        GrammarEltBase {code : 0x5c80000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DMUL(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x64T     },
    ]);
    base_grammar.insert("DP2A", vec![
        GrammarEltBase {code : 0x53f9000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DP2A(?^:\.(?<mode>LO|HI))(?^:(?:\.(?<type1>U32|S32))?(?:\.(?<type2>U32|S32))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T        },
    ]);
    base_grammar.insert("DP4A", vec![
        GrammarEltBase {code : 0x53f8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DP4A(?^:(?:\.(?<type1>U32|S32))?(?:\.(?<type2>U32|S32))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T     },
    ]);
    base_grammar.insert("DSET", vec![
        GrammarEltBase {code : 0x5900000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DSET(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT       },
    ]);
    base_grammar.insert("DSETP", vec![
        GrammarEltBase {code : 0x5b80000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?DSETP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<d20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT     },
    ]);
    base_grammar.insert(
        "EXIT",
        vec![
            GrammarEltBase {
                code: 0xe30000000000000f,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?EXIT;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("F2F", vec![
        GrammarEltBase {code : 0x5ca8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?F2F(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?:\.(?<round>ROUND|FLOOR|CEIL|TRUNC))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT       },
    ]);
    base_grammar.insert("F2I", vec![
        GrammarEltBase {code : 0x5cb0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?F2I(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<round>ROUND|FLOOR|CEIL|TRUNC))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT     },
    ]);
    base_grammar.insert("FADD", vec![
        GrammarEltBase {code : 0x5c58000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FADD(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T      },
    ]);
    base_grammar.insert("FADD32I", vec![
        GrammarEltBase {code : 0x800000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FADD32I(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?)));)"#, itype : &x32T        },
    ]);
    base_grammar.insert("FCHK", vec![
        GrammarEltBase {code : 0x5c88000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FCHK\.DIVIDE (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T       },
    ]);
    base_grammar.insert("FCMP", vec![
        GrammarEltBase {code : 0x5ba0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FCMP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &cmpT        },
    ]);
    base_grammar.insert("FFMA", vec![
        GrammarEltBase {code : 0x5980000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FFMA(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T     },
        GrammarEltBase {code : 0x5980000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FFMA(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r39s20>(?<r20>(?^:[a-zA-Z_]\w*)))\|?(?:\.(?<r39part>H0|H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\]);)"#, itype : &x32T     },
    ]);
    base_grammar.insert("FLO", vec![
        GrammarEltBase {code : 0x5c30000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FLO\.U32 (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &s2rT     },
    ]);
    base_grammar.insert("FMNMX", vec![
        GrammarEltBase {code : 0x5c60000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMNMX(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT     },
    ]);
    base_grammar.insert("FMUL", vec![
        GrammarEltBase {code : 0x5c68000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMUL(?^:(?<FTZ>\.FTZ)?)(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T      },
    ]);
    base_grammar.insert("FMUL32I", vec![
        GrammarEltBase {code : 0x1e00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FMUL32I(?^:(?<FTZ>\.FTZ)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?)));)"#, itype : &x32T       },
    ]);
    base_grammar.insert("FSET", vec![
        GrammarEltBase {code : 0x5800000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSET(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT      },
    ]);
    base_grammar.insert("FSETP", vec![
        GrammarEltBase {code : 0x5bb0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSETP(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:(?<FTZ>\.FTZ)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT     },
    ]);
    base_grammar.insert(
        "FSWZADD",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?FSWZADD[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("HADD2", vec![
        GrammarEltBase {code : 0x5d10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HADD2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T       },
    ]);
    base_grammar.insert("HFMA2", vec![
        GrammarEltBase {code : 0x5d00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HFMA2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T      },
    ]);
    base_grammar.insert("HMUL2", vec![
        GrammarEltBase {code : 0x5d08000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HMUL2(?^:(?:\.(?<mode>F32|MRG_H0|MRG_H1))?(?^:(?<FTZ>\.FTZ)?))(?^:(?<FMZ>\.FMZ)?)(?^:(?<FTZ>\.FTZ)?)(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &x32T       },
    ]);
    base_grammar.insert("HSETP2", vec![
        GrammarEltBase {code : 0x5d20000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?HSETP2(?^:(?<cmp>\.LT|\.EQ|\.LE|\.GT|\.NE|\.GE|\.NUM|\.NAN|\.LTU|\.EQU|\.LEU|\.GTU|\.NEU|\.GEU|))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<f20>(?:(?<neg>\-)|\+|)(?i:inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT       },
    ]);
    base_grammar.insert("I2F", vec![
        GrammarEltBase {code : 0x5cb8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?I2F(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?:\.(?<rnd>RN|RM|RP|RZ))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT     },
    ]);
    base_grammar.insert("I2I", vec![
        GrammarEltBase {code : 0x5ce0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?I2I(?^:\.(?<destSign>F|U|S)(?<destWidth>8|16|32|64)\.(?<srcSign>F|U|S)(?<srcWidth>8|16|32|64))(?^:(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &qtrT     },
    ]);
    base_grammar.insert("IADD", vec![
        GrammarEltBase {code : 0x5c10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD(?^:(?<SAT>\.SAT)?)(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T     },
    ]);
    base_grammar.insert("IADD3", vec![
        GrammarEltBase {code : 0x5cc0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD3(?^:(?:\.(?<type>X|RS|LS))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T     },
    ]);
    base_grammar.insert("IADD32I", vec![
        GrammarEltBase {code : 0x1c00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IADD32I(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T      },
    ]);
    base_grammar.insert("ICMP", vec![
        GrammarEltBase {code : 0x5b41000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ICMP(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &cmpT       },
    ]);
    base_grammar.insert(
        "IMAD",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMAD[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "IMADSP",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMADSP[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("IMNMX", vec![
        GrammarEltBase {code : 0x5c21000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMNMX(?^:(?<U32>\.U32)?)(?^:(?:\.(?<mode>XHI|XLO))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT     },
    ]);
    base_grammar.insert(
        "IMUL",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?IMUL[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("ISCADD", vec![
        GrammarEltBase {code : 0x5c18000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISCADD (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<i39w8>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT     },
    ]);
    base_grammar.insert("ISCADD32I", vec![
        GrammarEltBase {code : 0x1400000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISCADD32I (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))), (?^:(?<i53w5>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT      },
    ]);
    base_grammar.insert("ISET", vec![
        GrammarEltBase {code : 0x5b51000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISET(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?)(?^:(?<X>\.X)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &shftT      },
    ]);
    base_grammar.insert("ISETP", vec![
        GrammarEltBase {code : 0x5b61000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ISETP(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?<U32>\.U32)?)(?^:(?<X>\.X)?)(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT     },
    ]);
    base_grammar.insert("JCAL", vec![
        GrammarEltBase {code : 0xe220000000000040, rule : r#"(?^:^(?^:(?<noPred>))?JCAL (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T      },
    ]);
    base_grammar.insert(
        "JMP",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?JMP[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "JMX",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?JMX[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("LD", vec![
        GrammarEltBase {code : 0x8000000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LD(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<p58>(?^:P[0-6T])));)"#, itype : &gmemT        },
    ]);
    base_grammar.insert("LDC", vec![
        GrammarEltBase {code : 0xef90000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDC(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:c\[(?<c36>(?^:0[xX][0-9a-fA-F]+))\]\s*(?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]));)"#, itype : &gmemT       },
    ]);
    base_grammar.insert("LDG", vec![
        GrammarEltBase {code : 0xeed0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDG(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &gmemT      },
    ]);
    base_grammar.insert("LDL", vec![
        GrammarEltBase {code : 0xef40000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDL(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &gmemT      },
    ]);
    base_grammar.insert("LDS", vec![
        GrammarEltBase {code : 0xef48000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LDS(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]);)"#, itype : &smemT      },
    ]);
    base_grammar.insert("LEA", vec![
        GrammarEltBase {code : 0x5bd0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA (?^:(?<p48>(?^:P[0-6T]))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &cmpT     },
        GrammarEltBase {code : 0x5bd7000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<i39w8>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT        },
        GrammarEltBase {code : 0x5bdf004000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA\.HI(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i28w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT      },
        GrammarEltBase {code : 0xa07000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LEA\.HI(?^:(?<X>\.X)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i51w5>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT       },
    ]);
    base_grammar.insert("LOP", vec![
        GrammarEltBase {code : 0x5c40000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP(?^:\.(?<bool>AND|OR|XOR|PASS_B))(?^:(?:\.(?<z>NZ|Z) (?^:(?<p48>(?^:P[0-6T]))),|(?<noz>))) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?<INV>~)?(?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?))(?<INV>\.INV)?;)"#, itype : &x32T      },
    ]);
    base_grammar.insert("LOP3", vec![
        GrammarEltBase {code : 0x5be7000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP3\.LUT (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i28w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T        },
        GrammarEltBase {code : 0x3c00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP3\.LUT (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?), (?^:(?<i48w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T        },
    ]);
    base_grammar.insert("LOP32I", vec![
        GrammarEltBase {code : 0x400000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?LOP32I(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T      },
    ]);
    base_grammar.insert("MEMBAR", vec![
        GrammarEltBase {code : 0xef98000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MEMBAR(?^:\.(?<mode>CTA|GL|SYS));)"#, itype : &x32T        },
    ]);
    base_grammar.insert("MOV", vec![
        GrammarEltBase {code : 0x5c98078000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MOV (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &x32T      },
    ]);
    base_grammar.insert("MOV32I", vec![
        GrammarEltBase {code : 0x10000000000f000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MOV32I (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?:(?^:(?<i20w32>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<f20w32>(?:\-|\+|)(?i:(?^:0[xX][0-9a-fA-F]+)|inf\s*|\d+(?:\.\d+(?:e[\+\-]\d+)?)?))));)"#, itype : &x32T       },
    ]);
    base_grammar.insert("MUFU", vec![
        GrammarEltBase {code : 0x5080000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?MUFU(?^:\.(?<func>COS|SIN|EX2|LG2|RCP|RSQ|RCP64H|RSQ64H)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?);)"#, itype : &qtrT       },
    ]);
    base_grammar.insert(
        "NOP",
        vec![
            GrammarEltBase {
                code: 0x50b0000000000f00,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?NOP;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("P2R", vec![
        GrammarEltBase {code : 0x38e8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?P2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), PR, (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w7>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T     },
    ]);
    base_grammar.insert("PBK", vec![
        GrammarEltBase {code : 0xe2a0000000000000, rule : r#"(?^:^(?^:(?<noPred>))?PBK (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T       },
    ]);
    base_grammar.insert("PCNT", vec![
        GrammarEltBase {code : 0xe2b0000000000000, rule : r#"(?^:^(?^:(?<noPred>))?PCNT (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T      },
    ]);
    base_grammar.insert(
        "PEXIT",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PEXIT[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("POPC", vec![
        GrammarEltBase {code : 0x5c08000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?POPC (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &s2rT        },
    ]);
    base_grammar.insert(
        "PRET",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PRET[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("PRMT", vec![
        GrammarEltBase {code : 0x5bc0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PRMT(?^:(?:\.(?<mode>F4E|B4E|RC8|ECL|ECR|RC16))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\])|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &x32T        },
    ]);
    base_grammar.insert("PSET", vec![
        GrammarEltBase {code : 0x5088000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PSET(?^:\.(?<bool2>AND|OR|XOR))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<p12not>\!)?(?<p12>(?^:P[0-6T]))), (?^:(?<p29not>\!)?(?<p29>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT       },
    ]);
    base_grammar.insert("PSETP", vec![
        GrammarEltBase {code : 0x5090000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?PSETP(?^:\.(?<bool2>AND|OR|XOR))(?^:\.(?<bool>AND|OR|XOR|PASS_B)) (?^:(?<p3>(?^:P[0-6T]))), (?^:(?<p0>(?^:P[0-6T]))), (?^:(?<p12not>\!)?(?<p12>(?^:P[0-6T]))), (?^:(?<p29not>\!)?(?<p29>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &cmpT     },
    ]);
    base_grammar.insert(
        "R2B",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?R2B[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("R2P", vec![
        GrammarEltBase {code : 0x38f0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?R2P PR, (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i20w7>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &shftT       },
    ]);
    base_grammar.insert("RED", vec![
        GrammarEltBase {code : 0xebf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RED(?^:(?<E>\.E)?(?:\.(?<mode>ADD|MIN|MAX|INC|DEC|AND|OR|XOR|EXCH|CAS))(?<type>|\.S32|\.U64|\.F(?:16x2|32)\.FTZ\.RN|\.S64|\.64)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i28w20>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT        },
    ]);
    base_grammar.insert(
        "RET",
        vec![
            GrammarEltBase {
                code: 0xe32000000000000f,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RET;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert("RRO", vec![
        GrammarEltBase {code : 0x5c90000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?RRO(?^:\.(?<func>SINCOS|EX2)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?);)"#, itype : &rroT       },
    ]);
    base_grammar.insert("S2R", vec![
        GrammarEltBase {code : 0xf0c8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?S2R (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:SR_(?<sr>\S+));)"#, itype : &s2rT       },
    ]);
    base_grammar.insert("SEL", vec![
        GrammarEltBase {code : 0x5ca0000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SEL (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &x32T       },
    ]);
    base_grammar.insert("SHF", vec![
        GrammarEltBase {code : 0x5bf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHF\.L(?^:(?<W>\.W)?(?:\.(?<type>U64|S64))?(?<HI>\.HI)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT      },
        GrammarEltBase {code : 0x5cf8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHF\.R(?^:(?<W>\.W)?(?:\.(?<type>U64|S64))?(?<HI>\.HI)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT      },
    ]);
    base_grammar.insert("SHFL", vec![
        GrammarEltBase {code : 0xef10000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHFL(?^:\.(?<mode>IDX|UP|DOWN|BFLY)) (?^:(?<p48>(?^:P[0-6T]))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?:(?^:(?<i20w8>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?:(?^:(?<i34w13>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)))|(?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?));)"#, itype : &smemT      },
    ]);
    base_grammar.insert("SHL", vec![
        GrammarEltBase {code : 0x5c48000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHL(?<W>\.W)? (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT     },
    ]);
    base_grammar.insert("SHR", vec![
        GrammarEltBase {code : 0x5c29000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SHR(?^:(?<U32>\.U32)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3))?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?));)"#, itype : &shftT        },
    ]);
    base_grammar.insert("SSY", vec![
        GrammarEltBase {code : 0xe290000000000000, rule : r#"(?^:^(?^:(?<noPred>))?SSY (?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &x32T       },
    ]);
    base_grammar.insert("ST", vec![
        GrammarEltBase {code : 0xa000000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?ST(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<p58>(?^:P[0-6T])));)"#, itype : &gmemT        },
    ]);
    base_grammar.insert("STG", vec![
        GrammarEltBase {code : 0xeed8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STG(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT      },
    ]);
    base_grammar.insert("STL", vec![
        GrammarEltBase {code : 0xef50000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STL(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &gmemT      },
    ]);
    base_grammar.insert("STS", vec![
        GrammarEltBase {code : 0xef58000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?STS(?^:(?<E>\.E)?(?<U>\.U)?(?:\.(?<cache>CG|CI|CS|CV|IL|WT))?)(?^:(?<type>\.U8|\.S8|\.U16|\.S16||\.32|\.64|\.128)) (?^:\[(?:(?<r8>(?^:[a-zA-Z_]\w*))|(?<nor8>))(?:\s*\+?\s*(?^:(?<i20w24>\-?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))))?\]), (?^:(?<r0>(?^:[a-zA-Z_]\w*)));)"#, itype : &smemT      },
    ]);
    base_grammar.insert(
        "SUATOM",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SUATOM[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "SULD",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SULD[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "SURED",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SURED[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "SUST",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SUST[^;]*;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "SYNC",
        vec![
            GrammarEltBase {
                code: 0xf0f800000000000f,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?SYNC;)"#,
                itype: &x32T,
            },
        ],
    );
    base_grammar.insert(
        "TEX",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TEX[^;]*;)"#,
                itype: &gmemT,
            },
        ],
    );
    base_grammar.insert(
        "TEXS",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TEXS[^;]*;)"#,
                itype: &gmemT,
            },
        ],
    );
    base_grammar.insert("TLD", vec![
        GrammarEltBase {code : 0xdd38000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD\.B\.LZ\.(?^:(?<NODEP>NODEP\.)?(?:(?<reuse1>T)|(?<reuse2>P))) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:0[xX][0-9a-fA-F]+), \dD, (?^:(?<i31w4>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+)));)"#, itype : &gmemT        },
    ]);
    base_grammar.insert(
        "TLD4",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD4[^;]*;)"#,
                itype: &gmemT,
            },
        ],
    );
    base_grammar.insert(
        "TLD4S",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLD4S[^;]*;)"#,
                itype: &gmemT,
            },
        ],
    );
    base_grammar.insert("TLDS", vec![
        GrammarEltBase {code : 0xda0000000ff00000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TLDS\.LZ\.(?^:(?<NODEP>NODEP\.)?(?:(?<reuse1>T)|(?<reuse2>P))) (?^:(?<r28>(?^:[a-zA-Z_]\w*))), (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<i36w20>(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))), \dD, (?^:(?<chnls>R|RGBA));)"#, itype : &gmemT        },
    ]);
    base_grammar.insert(
        "TXQ",
        vec![
            GrammarEltBase {
                code: 0x0,
                rule: r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?TXQ[^;]*;)"#,
                itype: &gmemT,
            },
        ],
    );
    base_grammar.insert("VABSDIFF", vec![
        GrammarEltBase {code : 0x5427000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VABSDIFF(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT     },
    ]);
    base_grammar.insert("VADD", vec![
        GrammarEltBase {code : 0x2044000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VADD(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT     },
    ]);
    base_grammar.insert("VMAD", vec![
        GrammarEltBase {code : 0x5f04000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMAD(?^:\.(?<sign1>[SU])(?<size1>16)\.(?<sign2>[SU])(?<size2>16)) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T        },
        GrammarEltBase {code : 0x5f04000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMAD(?^:\.(?<sign1>[SU])(?<size1>8|16)\.(?<sign2>[SU])(?<size2>8|16)(?<PO>\.PO)?(?<SHR_7>\.SHR_7)?(?<SHR_15>\.SHR_15)?(?<SAT>\.SAT)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT       },
    ]);
    base_grammar.insert("VMNMX", vec![
        GrammarEltBase {code : 0x3a44000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VMNMX(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?:\.(?<MX>MX))?)(?^:(?<SAT>\.SAT)?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT       },
    ]);
    base_grammar.insert("VOTE", vec![
        GrammarEltBase {code : 0x50d8000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VOTE(?^:\.(?<mode>ALL|ANY|EQ)) (?:(?^:(?<r0>(?^:[a-zA-Z_]\w*))), |(?<nor0>))(?^:(?<p45>(?^:P[0-6T]))), (?^:(?<p39not>\!)?(?<p39>(?^:P[0-6T])));)"#, itype : &voteT     },
    ]);
    base_grammar.insert("VSET", vec![
        GrammarEltBase {code : 0x4004000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?VSET(?^:\.(?<cmp>LT|EQ|LE|GT|NE|GE))(?^:(?:\.(?<UD>UD))?(?:\.(?<SD>SD))?(?:\.(?<sign1>[SU])(?<size1>8|16|32))?(?:\.(?<sign2>[SU])(?<size2>8|16|32))?)(?^:(?:\.(?<mode>MRG_16[HL]|MRG_8B[0-3]|ACC|MIN|MAX))?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &shftT        },
    ]);
    base_grammar.insert("XMAD", vec![
        GrammarEltBase {code : 0x5b00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<mode>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?^:(?<i20>(?<neg>\-)?(?^:(?^:0[xX][0-9a-fA-F]+)|(?^:\d+[xX]<[^>]+>)|\d+))(?<r20neg>\.NEG)?)|(?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r20>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r20part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1))?(?<reuse2>\.reuse)?)), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T     },
        GrammarEltBase {code : 0x5900000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<mode>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?(?<r39s20>(?<r20>(?^:[a-zA-Z_]\w*)))\|?(?:\.(?<r39part>H0|H1))?(?<reuse2>\.reuse)?), (?^:(?<r39neg>\-)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c39>(?^:0[xX][0-9a-fA-F]+))\]);)"#, itype : &x32T        },
        GrammarEltBase {code : 0x5e00000000000000, rule : r#"(?^:^(?^:\@(?<predNot>\!)?P(?<predNum>[0-6]) )?XMAD(?^:(?:\.(?<type1>U16|S16))?(?:\.(?<type2>U16|S16))?(?:\.(?<modec>MRG|PSL\.CLO|PSL|CHI|CLO|CSFU))?(?<CBCC>\.CBCC)?) (?^:(?<r0>(?^:[a-zA-Z_]\w*))(?<CC>\.CC)?), (?^:(?<r8neg>\-)?(?<r8abs>\|)?(?<r8>(?^:[a-zA-Z_]\w*))\|?(?:\.(?<r8part>H0|H1|B0|B1|B2|B3|H0_H0|H1_H1|F32))?(?<reuse1>\.reuse)?), (?^:(?<r20neg>\-)?(?<r20abs>\|)?c\[(?<c34>(?^:0[xX][0-9a-fA-F]+))\]\s*\[(?<c20>(?^:0[xX][0-9a-fA-F]+))\]\|?(?:\.(?<r20partx>H0|H1|B0|B1|B2|B3))?), (?^:(?<r39neg>\-)?(?<r39>(?^:[a-zA-Z_]\w*))(?:\.(?<r39part>H0|H1|H0_H0|H1_H1|F32))?(?<reuse3>\.reuse)?);)"#, itype : &x32T     },
    ]);

    let mut grammar = HashMap::new();
    for (k, v) in base_grammar.iter() {
        let mut vec = Vec::<GrammarElt>::new();
        for elt in &v {
            vec.push(elt.into());
        }
        grammar.insert(*k, vec);
    }
    grammar
}

pub fn build_flags() -> MutStrMap<MutStrMap<SVal>> {
    // filter white space and convert to vec
    let flagsvec = flagsstr
        .split("\n")
        .filter_map(|s| if regex_match(r"\S+", s) {
            Some(s)
        } else {
            None
        })
        .collect::<Vec<&str>>();
    let mut flaginfo = None::<(&str, MutStrMap<u64>)>;
    let mut flags = MutStrMap::<MutStrMap<SVal>>::new();
    let mut ops = Vec::<&str>::new();
    for line in flagsvec {
        let matches = regex_matches(r"^(0x[0-9a-z]+)\s*(.*)", &line);
        if matches.len() == 0 {
            let opsname = line.split(":").collect::<Vec<&str>>();
            let opsstr = &opsname[0];
            ops = opsstr.split(",").collect();
            flaginfo = if opsname.len() == 2 {
                Some(((&opsname[1]).trim(), MutStrMap::new()))
            } else {
                None
            };
        } else {
            let caps = &matches[0];
            let val = hex(&caps[1]);
            if flaginfo.is_some() {
                let (flag, mut flagmap) = flaginfo.clone().unwrap();
                let key: String = (&caps[2]).into();
                flagmap[&key] = val;
                for o in &ops {
                    flags[&String::from(*o)][flag] = flagmap.clone().into();
                }
            } else {
                let key: String = (&caps[2]).into();
                for o in &ops {
                    flags[&String::from(*o)][&key] = val.into();
                }
            }
        }
    }
    flags
}

fn re_matches_by_name<'i, 'r>(
    line: &'i str,
    rule: &'r Regex,
    cap_data: &mut HashMap<&'r str, &'i str>,
) -> bool {
    let matches = re_matches(rule, line);
    let names = re_match_names(rule);
    if matches.is_empty() {
        return false;
    }
    for caps in matches {
        for name in &names {
            let name_match = caps.name(name);
            if name_match.is_some() {
                cap_data.insert(name, name_match.unwrap().as_str());
            }
        }
    }
    return true;
}

pub fn parse_instruct<'i, 'r>(
    inst: &'i str,
    rule: &'r Regex,
    cap_data: &mut HashMap<&'r str, &'i str>,
) -> bool {
    let matches = re_matches(rule, inst);
    let names = re_match_names(rule);
    if matches.is_empty() {
        return false;
    }
    for caps in matches {
        for name in &names {
            let name_match = caps.name(name);
            if name_match.is_some() {
                cap_data.insert(name, name_match.unwrap().as_str());
            }
        }
    }
    return true;
}

pub fn normalize_spacing(inst: &str) -> String {
    let s = regex_replace(r"\t", " ", inst);
    regex_replace(r"\s{2,}", " ", &s)
}
pub fn print_ctrl(code: u64) -> String {
    let stall = (code & 0x0_000f) >> 0;
    let yieldf = (code & 0x00010) >> 4; // yield flag
    let wrtdb = (code & 0x000e0) >> 5; // write dependency barier
    let readb = (code & 0x00700) >> 8; // read  dependency barier
    let watdb = (code & 0x1f800) >> 11; // wait on dependency barier
    let yieldf = if yieldf != 0 { "-" } else { "Y" };
    let wrtdb = if wrtdb == 7 {
        "-".into()
    } else {
        format!("{}", wrtdb + 1)
    };
    let readb = if readb == 7 {
        "-".into()
    } else {
        format!("{}", readb + 1)
    };
    let watdb = if watdb != 0 {
        format!("{:2x}", watdb)
    } else {
        "--".into()
    };
    format!("{}{}{}{}{:x}", watdb, readb, wrtdb, yieldf, stall)
}

pub fn read_ctrl(ctrl: &str, context: &str) -> u64 {
    let c: Vec<&str> = ctrl.split(":").collect();
    let (watdb, readb, wrtdb, yieldf, stall) = (c[0], c[1], c[2], c[3], c[4]);
    let watdb = if watdb == "--" { 0 } else { hex(watdb) };
    let readb = if readb == "-" { 7 } else { hex(readb) - 1 };
    let wrtdb = if wrtdb == "-" { 7 } else { hex(wrtdb) - 1 };
    let yieldf = if yieldf == "y" || yieldf == "Y" { 0 } else { 1 };
    let stall = hex(stall);
    if watdb != watdb & 0x3f {
        panic!(
            "wait dep out of range(0x00-0x3f): {:x} at {}",
            watdb,
            context
        );
    }
    (watdb << 11) | (readb << 8) | (wrtdb << 5) | (yieldf << 4) | (stall << 0)
}
pub fn process_sass_ctrl_line(
    line: &str,
    ctrl: Option<&mut Vec<u64>>,
    ruse: Option<&mut Vec<u64>>,
) -> bool {
    let matches = regex_matches(r"^\s+\/\* (0x[0-9a-f]+)", line);
    if matches.is_empty() {
        return false;
    }
    let code = hex(&matches[0][1]);
    if let Some(r) = ctrl {
        r.push((code & 0x000000000001ffff) >> 0);
        r.push((code & 0x0000003fffe00000) >> 21);
        r.push((code & 0x07fffc0000000000) >> 42);
    }
    if let Some(r) = ruse {
        r.push((code & 0x00000000001e0000) >> 17);
        r.push((code & 0x000003c000000000) >> 38);
        r.push((code & 0x7800000000000000) >> 59)
    }
    true
}

pub fn get_reg_num<'a, 'b>(regmap: &'a MutStrMap<SVal>, regname: &'b str) -> String {
    if regmap.contains_key(regname) {
        regmap.get(regname).unwrap().clone().into()
    } else {
        regname.into()
    }
}
pub fn get_vec_registers(
    vectors: &HashMap<&str, Vec<String>>,
    cap_data: &HashMap<&str, &str>,
) -> Option<String> {
    let regname = cap_data.get("r0");
    let regname = if regname.is_some() {
        *regname.unwrap()
    } else {
        return None;
    };
    if regname == "RZ" {
        return None;
    }
    if cap_data["type"] == ".64" || cap_data["131w4"] == "0x3" {
        let matches = regex_matches(r"^R(\d+)$", regname);
        if !matches.is_empty() {
            let n = hex(&matches[0][1]);
            return Some(format!("R{}R{}", n, n + 1));
        }
        if !vectors.contains_key(regname) {
            println!("{} is not a 64bit vector register", regname);
        }
        let reg = &vectors[regname];
        return Some(format!("{}{}", reg[0], reg[1]));
    }

    if cap_data["type"] == ".128" || cap_data["131w4"] == "0xf" {
        let matches = regex_matches(r"^R(\d+)$", regname);
        if !matches.is_empty() {
            let n = hex(&matches[0][1]);
            return Some(format!("R{}R{}R{}R{}", n, n + 1, n + 2, n + 3));
        }
        if !vectors.contains_key(regname) || vectors[regname].len() != 4 {
            println!("{} is not a 128bit vector register", regname);
        }
        Some(
            vectors[regname]
                .iter()
                .map(|s| s.clone())
                .collect::<String>(),
        );
    }

    Some(regname.into())
}

pub fn get_addr_vec_registers<'a>(
    vectors: &'a HashMap<&'a str, Vec<String>>,
    cap_data: &'a HashMap<&'a str, &str>,
) -> Option<String> {
    let regname = cap_data.get("r8");
    let regname = if let Some(r) = regname {
        r
    } else {
        return None;
    };
    if *regname == "RZ" {
        return None;
    }
    if !cap_data.contains_key("E") {
        return Some(regname.clone().into());
    }
    let matches = regex_matches(r"^R(\d+)$", regname);
    if !matches.is_empty() {
        let n = hex(&matches[0][1]);
        return Some(format!("R{}R{}", n, n + 1));
    }
    if !vectors.contains_key(regname) {
        println!("{:?}", vectors);
        println!("{} not a 64bit vector register", regname);
    }
    Some(
        vectors[regname][0..1]
            .iter()
            .map(|s| s.clone())
            .collect::<String>(),
    )
}
pub fn replace_xmads(file: &str) -> String {


    unimplemented!();
    "".into()
}
pub struct SassGrammar<'a> {
    ctrl_re: Regex,
    pred_re: Regex,
    inst_re: Regex,
    comm_re: Regex,
    asm_re: Regex,
    sass_re: Regex,
    flags: MutStrMap<MutStrMap<SVal>>,
    immed_codes: HashMap<u64, u64>,
    reuse_codes: HashMap<&'a str, u64>,
    immed_ops: Vec<&'a str>,
    const_codes: HashMap<&'a str, u64>,
    operands: HashMap<&'a str, Box<Fn(&str) -> u64>>,
}

impl<'a> SassGrammar<'a> {
    pub fn new() -> Self {
        let pred_re_str = r"(?<pred>@!?(?<predReg>P\d)\s+)";
        let inst_re_base_str = r"?(?<op>\w+)(?<rest>[^;]*;)";
        let ctrl_re_str = r"(?<ctrl>[0-9a-fA-F\-]{2}:[1-6\-]:[1-6\-]:[\-yY]:[0-9a-fA-F])";
        let inst_re_str = format!("{}{}", pred_re_str, inst_re_base_str);
        let comm_re_str = r"(?<comment>.*)";
        let asm_re_str = format!(
            r"^{}(?<space>\s+){}{}",
            ctrl_re_str,
            inst_re_str,
            comm_re_str
        );
        let sass_re_str = format!(
            r"^\s+/\*(?<num>[0-9a-f]+)\*/\s+{}\s+/\* (?<code>0x[0-9a-f]+)",
            inst_re_str
        );
        let ctrl_re = Regex::new(ctrl_re_str).unwrap();
        let pred_re = Regex::new(pred_re_str).unwrap();
        let inst_re = Regex::new(&inst_re_str).unwrap();
        let comm_re = Regex::new(comm_re_str).unwrap();
        let asm_re = Regex::new(&asm_re_str).unwrap();
        let sass_re = Regex::new(&sass_re_str).unwrap();
        let mut immed_codes: HashMap<u64, u64> = HashMap::new();
        let mut reuse_codes: HashMap<&'a str, u64> = HashMap::new();
        let mut const_codes: HashMap<&'a str, u64> = HashMap::new();
        immed_codes.insert(0x5c, 0x64);
        immed_codes.insert(0x5b, 0x6d);
        immed_codes.insert(0x59, 0x6b);
        immed_codes.insert(0x58, 0x68);
        reuse_codes.insert("reuse1", 1);
        reuse_codes.insert("reuse2", 2);
        reuse_codes.insert("reuse3", 4);
        const_codes.insert("c20", 0x10);
        const_codes.insert("c39", 0x08);
        let immed_ops = vec!["i20", "f20", "d20"];
        SassGrammar {
            ctrl_re: ctrl_re,
            pred_re: pred_re,
            inst_re: inst_re,
            comm_re: comm_re,
            asm_re: asm_re,
            sass_re: sass_re,
            flags: build_flags(),
            immed_codes: immed_codes,
            reuse_codes: reuse_codes,
            immed_ops: immed_ops,
            const_codes: const_codes,
            operands: build_operands(),
        }
    }
    pub fn gen_reuse_code<'i, 'r>(&self, cap_data: &mut HashMap<&'r str, &'i str>) -> u64 {
        let mut reuse = 0;
        for k in self.reuse_codes.keys() {
            if cap_data.contains_key(k) {
                reuse |= self.reuse_codes[k];
            }
        }
        reuse
    }
    pub fn gen_code(
        &self,
        op: &str,
        grammar: &GrammarElt,
        cap_data: &mut HashMap<&'a str, &str>,
        mut test: Option<&mut Vec<&'a str>>,
    ) -> (u64, u64) {
        let flags = &self.flags[op];
        let mut code = grammar.code;
        let mut reuse = 0 as u64;
        let immed_code = self.immed_codes[&(code >> 56)];
        if cap_data.contains_key("noPred") {
            cap_data.remove("noPred");
            if let Some(ref mut r) = test {
                r.push("noPred");
            }
        } else {
            let mut p = if cap_data.contains_key("predNum") {
                hex(cap_data["predNum"])
            } else {
                7
            };
            if let Some(ref mut r) = test {
                r.push("PredNum");
            }
            if cap_data.contains_key("predNot") {
                p |= 8;
                if let Some(ref mut r) = test {
                    r.push("PredNot");
                }
                code ^= p << 16;
                cap_data.remove("predNum");
                cap_data.remove("predNot");
            }

        }
        for rcode in ["rcode1", "rcode2", "rcode3"].iter() {
            if let Some(_) = cap_data.remove(rcode) {
                reuse |= self.reuse_codes[rcode];
                if let Some(ref mut r) = test {
                    r.push(rcode);
                }
            }
        }
        for capture in cap_data.keys() {
            if self.immed_ops.contains(capture) {
                code ^= (immed_code << 56);
            } else if self.const_codes.contains_key(capture) {
                code ^= self.const_codes[capture] << 56;
            }
            if self.operands.contains_key(capture) {
                if *capture != "r20" || !cap_data.contains_key("r39s20") {
                    code ^= self.operands[capture](cap_data[capture]);
                    if let Some(ref mut r) = test {
                        r.push(capture);
                    }
                }
            }
            if flags.contains_key(capture) {
                match flags[*capture] {
                    SVal::Str(ref s) => {
                        code ^= hex(&s);
                        if let Some(ref mut r) = test {
                            r.push(capture);
                        }

                    }
                    SVal::StringMap(ref flagmap) => {
                        let flag: u64 = hex(&flagmap[*capture]);
                        code ^= flag;
                        if let Some(ref mut r) = test {
                            r.push(capture);
                            /* XXX fixme */
                            r.push("capdata->capture")
                        }

                    }
                    _ => panic!("unexpected type"),
                }
            } else if !self.operands.contains_key(capture) && test.is_none() {
                println!("UNUSED: {}: {}: {}", op, capture, cap_data[capture]);
                println!("{:?}", flags);
            }
        }
        (code, reuse)
    }
    pub fn process_asm_line<'r, 'i>(
        &self,
        line: &str,
        linenum: usize,
        cap_data: &mut HashMap<&'r str, SVal>,
    ) -> bool {
        let mut map = HashMap::new();
        if !re_matches_by_name(line, &self.asm_re, &mut map) {
            return false;
        }
        cap_data.insert("linenum", linenum.into());
        cap_data.insert("pred", map["pred"].into());
        cap_data.insert("predReg", map["predReg"].into());
        cap_data.insert("space", map["space"].into());
        cap_data.insert("op", map["op"].into());
        cap_data.insert("comment", map["comment"].into());
        cap_data.insert(
            "inst",
            normalize_spacing(&format!("{}{}{}", map["pred"], map["op"], map["rest"])).into(),
        );
        cap_data.insert("ctrl", read_ctrl(map["ctrl"], line).into());
        true
    }
    pub fn process_sass_line<'r, 'i>(
        &self,
        line: &'i str,
        linenum: usize,
        cap_data: &mut HashMap<&'r str, SVal>,
    ) -> bool {
        let mut map = HashMap::new();
        if !re_matches_by_name(line, &self.sass_re, &mut map) {
            return false;
        }
        cap_data.insert("num", hex(map["num"]).into());
        cap_data.insert("pred", map["pred"].into());
        cap_data.insert("op", map["op"].into());
        cap_data.insert(
            "ins",
            normalize_spacing(&format!("{}{}", map["op"], map["rest"])).into(),
        );
        cap_data.insert(
            "inst",
            normalize_spacing(&format!("{}{}{}", map["pred"], map["op"], map["rest"])).into(),
        );
        cap_data.insert("code", hex(map["code"]).into());
        true
    }
}
