
use memmap::{Mmap, Protection};
use std::{io, slice};
use std::io::{Read, Write};
use elf;
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr,
          Elf64_Sym};
use std::ffi::CString;
use std::os::raw::c_char;

pub struct Cubin {}
pub struct SymEnt {}
pub enum SecHdr {
    StrTab(Vec<String>),
    SymTab32(Vec<Elf32_Sym>),
    SymTab64(Vec<Elf64_Sym>),
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
        }).clone()
            .to_vec();
        let (off, len) = (hdr.e_shoff as isize, hdr.e_shentsize as usize);
        let shdrs = (unsafe {
            slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Shdr, len)
        }).clone()
            .to_vec();
        let shdrs: Vec<Elf32_Shdr> = shdrs
            .iter()
            .filter(|h| h.sh_type != elf::SHT_NOBITS && h.sh_size != 0)
            .map(|h| *h)
            .collect();
        let mut shdrvals = Vec::new();
        for shdr in shdrs {
            let (off, len) = (shdr.sh_offset as isize, shdr.sh_size as usize);
            match shdr.sh_type {
                elf::SHT_STRTAB => {
                    let cstr = (unsafe {
                        slice::from_raw_parts(fp.ptr().offset(off) as *const u8, len)
                    }).to_vec();
                    let strs = (unsafe { CString::from_vec_unchecked(cstr).into_string() })
                        .ok()
                        .unwrap();
                    let strtab = strs.split('\0').map(|s| s.into()).collect::<Vec<String>>();
                    shdrvals.push(SecHdr::StrTab(strtab));
                }
                elf::SHT_SYMTAB => {}
                _ => {}
            }
        }


        Ok(Cubin {})
    }
    fn new64(fp: Mmap) -> io::Result<Self> {
        let hdr = unsafe { &*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr };
        Ok(Cubin {})
    }
}
