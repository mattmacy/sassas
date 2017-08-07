use elf::{Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Phdr, Elf64_Shdr, Elf64_Sym};
use unsafe_lib::MutStrMap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct KernelSection {
    pub name: String,
    pub constant_sec: SecHdr,
    pub shared_sec: Option<SecHdr>,
    pub map: HashMap<&'static str, SVal>,
}

#[derive(Clone, Debug)]
pub enum ElfSymbol {
    Sym32(Elf32_Sym),
    Sym64(Elf64_Sym),
}
impl ElfSymbol {
    pub fn name(&self) -> usize {
        use self::ElfSymbol::*;
        match *self {
            Sym32(s) => s.st_name as usize,
            Sym64(s) => s.st_name as usize,
        }
    }
    pub fn shndx(&self) -> usize {
        use self::ElfSymbol::*;
        match *self {
            Sym32(s) => s.st_shndx as usize,
            Sym64(s) => s.st_shndx as usize,
        }
    }
    pub fn info(&self) -> usize {
        use self::ElfSymbol::*;
        match *self {
            Sym32(s) => s.st_info as usize,
            Sym64(s) => s.st_info as usize,
        }
    }
}


#[derive(Clone, Debug)]
pub enum SecHdr {
    StrTab(Vec<String>, Vec<u8>),
    SymTab(Vec<ElfSymbol>, Vec<u8>),
    Empty,
    Other(u32, Vec<u8>),
}
impl Default for SecHdr {
    fn default() -> Self {
        SecHdr::Empty
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SymBind {
    Local,
    Global,
    Weak,
}

#[derive(Clone, Debug)]
pub enum SVal {
    Bool(bool),
    Int(i32),
    UInt(u32),
    Float(f32),
    Data(Vec<u8>),
    DataL(Vec<u32>),
    Elf32Phdrs(Vec<Elf32_Phdr>),
    Elf64Phdrs(Vec<Elf64_Phdr>),
    Elf32Shdrs(Vec<Elf32_Shdr>),
    Elf64Shdrs(Vec<Elf64_Shdr>),
    Elf32Shdr(Elf32_Shdr),
    Elf64Shdr(Elf64_Shdr),
    KernelSection(KernelSection),
    StrVec(Vec<String>),
    SecHdr(SecHdr),
    SymBind(SymBind),
    ElfSymbol(ElfSymbol),
    SymEnt32(Elf32_Sym),
    SymEnt64(Elf64_Sym),
    Map(MutStrMap<SVal>),
    Required,
}
impl Default for SVal {
    fn default() -> Self {
        SVal::Required
    }
}

macro_rules! impl_from {
    ($type:ty, $name:ident) => (
        impl From<$type> for SVal {
            fn from(input: $type) -> Self {
                SVal:: $name (input)
            }
        }
        impl From<SVal> for $type {
            fn from(input : SVal) -> Self {
                match input {
                    self::SVal:: $name (x) => x.clone(),
                    _ => unimplemented!(),
                }
            }
        }
    )
}
impl_from!(f32, Float);
impl_from!(i32, Int);
impl_from!(u32, UInt);
impl_from!(bool, Bool);
impl_from!(Elf32_Sym, SymEnt32);
impl_from!(Elf64_Sym, SymEnt64);
impl_from!(Vec<Elf32_Phdr>, Elf32Phdrs);
impl_from!(Vec<Elf64_Phdr>, Elf64Phdrs);
impl_from!(Vec<Elf32_Shdr>, Elf32Shdrs);
impl_from!(Vec<Elf64_Shdr>, Elf64Shdrs);
impl_from!(Elf32_Shdr, Elf32Shdr);
impl_from!(Elf64_Shdr, Elf64Shdr);
impl_from!(ElfSymbol, ElfSymbol);
impl_from!(KernelSection, KernelSection);
impl_from!(SecHdr, SecHdr);
impl_from!(Vec<u8>, Data);
impl_from!(Vec<u32>, DataL);
impl_from!(Vec<String>, StrVec);
impl_from!(SymBind, SymBind);
impl_from!(MutStrMap<SVal>, Map);
