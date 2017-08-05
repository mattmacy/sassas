
use memmap::{Mmap, Protection};
use std::{collections, io, slice};
use elf;
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr,
          Elf64_Sym};

pub struct Cubin {}
pub struct SymEnt {}
#[derive(Clone, Debug)]
pub enum SecHdr {
    StrTab(Vec<String>),
    SymTab32(Vec<Elf32_Sym>),
    SymTab64(Vec<Elf64_Sym>),
    Empty,
    Other(u32),
}


impl Cubin {
    pub fn new(file: String) -> io::Result<Self> {
        let fp = Mmap::open_path(file, Protection::Read)?;
        let elf32_hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
        let class = elf32_hdr.e_type;
        if class == elf::ELFCLASS32 as u16 {
            Self::new32(fp)
        } else if class == elf::ELFCLASS64 as u16 {
            Self::new64(fp)
        } else {
            panic!("invalid class type: {}", class)
        }
    }
    fn new32(fp: Mmap) -> io::Result<Self> {
        let hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
        let arch = hdr.e_flags & 0xff;
        let addr_size = if hdr.e_flags & 0x400 == 0x400 { 64 } else { 32 };
        let (off, len) = (hdr.e_phoff as isize, hdr.e_phentsize as usize);
        let phdrs = (unsafe {
                         slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Phdr, len)
                     }).to_vec();
        let (off, len) = (hdr.e_shoff as isize, hdr.e_shentsize as usize);
        let shdrs = (unsafe {
                         slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Shdr, len)
                     }).to_vec();
        let stridx = hdr.e_shstrndx;
        let mut shdrvals = Vec::new();
        let mut shdrmap = collections::HashMap::new();
        for shdr in &shdrs {
            let (off, len) = (shdr.sh_offset as isize, shdr.sh_size as usize);
            let data = unsafe { slice::from_raw_parts(fp.ptr().offset(off) as *const u8, len) };
            if shdr.sh_type == elf::SHT_NOBITS || shdr.sh_size == 0 {
                shdrvals.push((SecHdr::Empty, data.to_vec()));
                continue;
            }
            let sh = match shdr.sh_type {
                elf::SHT_STRTAB => {
                    let strtab = data.split(|ch| *ch == b'\0')
                        .map(|slice| String::from_utf8(slice.to_vec()).unwrap())
                        .collect::<Vec<String>>();
                    SecHdr::StrTab(strtab)
                }
                elf::SHT_SYMTAB => {
                    let mut v = Vec::new();
                    let mut offset = 0;
                    while offset < shdr.sh_size {
                        let sym = unsafe {
                            &*(fp.ptr().offset(off + offset as isize) as *const Elf32_Sym) as
                                &Elf32_Sym
                        };
                        v.push(sym.clone());
                        offset += shdr.sh_entsize;
                    }
                    SecHdr::SymTab32(v)
                }
                _ => SecHdr::Other(shdr.sh_type),
            };
            shdrvals.push((sh, data.to_vec()));
        }
        let &(ref strtabent, _) = &shdrvals[stridx as usize];
        let strtab = match strtabent {
            &SecHdr::StrTab(ref v) => v,
            _ => panic!("expected strtab, got: {:?}", strtabent),
        };
        for shdr in &shdrs {
            shdrmap.insert(strtab[shdr.sh_name as usize].clone(), shdr.clone());
        }
        //let strtab = collections::HashMap::new();


        Ok(Cubin {})
    }
    fn new64(fp: Mmap) -> io::Result<Self> {
        let hdr = unsafe { &*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr };
        Ok(Cubin {})
    }
}
