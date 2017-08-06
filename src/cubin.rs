
use memmap::{Mmap, Protection};
use std::{collections, io, slice};
use std::collections::{VecDeque, HashMap};
use itertools::zip;
use sval::SVal;
use unsafe_lib::MutMap;
use elf;
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr,
          Elf64_Sym};

pub struct Cubin {}
pub struct SymEnt {}
#[derive(Clone, Debug)]
pub enum SecHdr {
    StrTab(Vec<String>, Vec<u8>),
    SymTab32(Vec<Elf32_Sym>, Vec<u8>),
    SymTab64(Vec<Elf64_Sym>, Vec<u8>),
    Empty,
    Other(u32, Vec<u8>),
}
pub enum CubinFld {
    SHdr(SecHdr),
    Symbols32(collections::HashMap<String, Elf32_Sym>),
    Symbols64(collections::HashMap<&'static str, Elf64_Sym>),
    Empty,
}
#[derive(Clone, Copy)]
pub enum SymBind {
    Local,
    Global,
    Weak,
}
static SYMBIND: [SymBind; 3] = [SymBind::Local, SymBind::Global, SymBind::Weak];

pub struct KernelSection {
    pub name: String,
    pub linkage: SymBind,
    pub kernel_data: Vec<u8>,
    pub bar_cnt: u32,
    pub reg_cnt: u32,
    pub shared_size: u32,
    pub constant_sec: SecHdr,
    pub param_sec: SecHdr,
}
impl Default for CubinFld {
    fn default() -> Self {
        CubinFld::Empty
    }
}


impl Cubin {
    pub fn new(file: String) -> io::Result<Self> {
        let fp = Mmap::open_path(file, Protection::Read)?;
        let elf32_hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
        match elf32_hdr.e_type as u32 {
            elf::ELFCLASS32 => Self::new32(fp),
            elf::ELFCLASS64 => Self::new64(fp),
            _ => panic!("invalid class type: {}", elf32_hdr.e_type),
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
        let mut cubintbl = MutMap::new();
        for shdr in &shdrs {
            let (off, len) = (shdr.sh_offset as isize, shdr.sh_size as usize);
            let data = unsafe { slice::from_raw_parts(fp.ptr().offset(off) as *const u8, len) };
            if shdr.sh_type == elf::SHT_NOBITS || shdr.sh_size == 0 {
                shdrvals.push(SecHdr::Empty);
                continue;
            }
            let sh = match shdr.sh_type {
                elf::SHT_STRTAB => {
                    let strtab = data.split(|ch| *ch == b'\0')
                        .map(|slice| String::from_utf8(slice.to_vec()).unwrap())
                        .collect::<Vec<String>>();
                    SecHdr::StrTab(strtab, data.to_vec())
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
                    SecHdr::SymTab32(v, data.to_vec())
                }
                _ => SecHdr::Other(shdr.sh_type, data.to_vec()),
            };
            shdrvals.push(sh);
        }
        let strtab = match shdrvals[stridx as usize] {
            SecHdr::StrTab(ref v, _) => v.clone(),
            _ => panic!("strtab not found"),
        };
        cubintbl.insert("Symbols", CubinFld::Symbols32(HashMap::new()));
        for (shdr, sh) in zip(&shdrs, &shdrvals) {
            let name = strtab[shdr.sh_name as usize].clone();
            shdrmap.insert(name, (shdr.clone(), sh.clone()));
            //cubintbl.insert(name.clone().as_str(), CubinFld::SHdr(sh.clone()));
        }
        let &(_, ref symtab_) = &shdrmap[".symtab"];
        let symtab = match symtab_ {
            &SecHdr::SymTab32(ref t, _) => t,
            _ => panic!("expect Symtab"),
        };
        let mut symtabmap = collections::HashMap::new();
        //let mut kernsecmap = collections::HashMap::new();
        for syment in symtab {
            let symname = strtab[syment.st_name as usize].clone();
            symtabmap.insert(symname.clone(), syment.clone());
            let sh = &shdrs[syment.st_shndx as usize];
            let shval = &shdrvals[syment.st_shndx as usize];
            if syment.st_info & 0x0f != 0x02 && syment.st_info & 0x10 != 0x10 {
                continue;
            }
            if syment.st_info & 0x10 == 0x10 {
                let mut symmap = match &mut cubintbl["Symbols"] {
                    &mut CubinFld::Symbols32(ref mut map) => map,
                    _ => panic!("no symbols found"),
                };
                symmap.insert(symname.clone(), syment.clone());
                continue;
            }
            // Remaining will all be tagged FUNC
            // Create a hash of kernels for output
            //my $kernelSec = $cubin->{Kernels}{$symEnt->{Name}} = $secHdr;
            //kernsecmap.insert(symname.clone(), sh.clone());

            // Extract local/global/weak binding info
            let linkage = SYMBIND[((syment.st_info & 0xf0) >> 4) as usize];
            // Extract the kernel instructions
            let data = match shval {
                &SecHdr::Other(_, ref data) => data,
                _ => panic!("unexpected hdr: {:?}", sh),
            };
            // Extract the max barrier resource identifier used and add 1. Should be 0-16.
            // If a register is used as a barrier resource id, then this value is the max of 16.
            let bar_cnt = (sh.sh_flags & 0x01f00000) >> 20;
            // Extract the number of allocated registers for this kernel.
            let reg_cnt = (sh.sh_info & 0xff000000) >> 24;

            // Extract the size of shared memory this kernel uses.
            let shared_sec_ = &shdrmap.get(&format!(".nv.shared.{}", symname));
            let size = match shared_sec_ {
                &None => 0,
                &Some(&(ref shdr, _)) => shdr.sh_size,
            };

            // Attach constant0 section
            let &(ref constsh, ref constshval) = &shdrmap[&format!(".nv.constant0.{}", symname)];

            // Extract the kernel parameter data.
            let infoname = format!(".nv.info.{}", symname);

            if !shdrmap.contains_key(&infoname) {
                continue;
            }
            let &(ref paramsh, ref paramshval) = &shdrmap[&infoname];
            let data = match paramshval {
                &SecHdr::Other(_, ref data) => data,
                _ => panic!("got unexpected hdr type: {:?}", paramshval),
            };
            let mut paramshmap: HashMap<&'static str, SVal> = collections::HashMap::new();
            let data32 = unsafe { ::std::mem::transmute::<Vec<u8>, Vec<u32>>(data.clone()) };
            paramshmap.insert("ParamData", SVal::DataL(data32.clone()));
            let hex32 = data32
                .iter()
                .map(|v| format!("0x{:08x}", *v))
                .collect::<Vec<String>>();
            paramshmap.insert("ParamHex", hex32.into());

            // find the first param delimiter
            let mut idx = 0;
            while idx < data32.len() && data32[idx] != 0x00080a04 {
                idx += 1;
            }
            let first = data32[idx + 2] & 0xFFFF;
            idx += 4;
            let mut params = VecDeque::new();
            while idx < data32.len() && data32[idx] == 0x000c1704 {
                let ord = data32[idx + 2] & 0xFFFF;
                let offset = format!("0x{:02x}", first + (data32[idx + 2] >> 16));
                let psize = data32[idx + 3] >> 18;
                let align = if data32[idx + 3] & 0x400 == 0x400 {
                    1 << (data32[idx + 3] & 0x3ff)
                } else {
                    0
                };
                let param = format!("{}:{}:{}:{}", ord, offset, psize, align);
                params.push_front(param);
                idx += 4;
            }
            let static_params = &data32[0..idx - 1];


        }

        Ok(Cubin {})
    }
    fn new64(fp: Mmap) -> io::Result<Self> {
        let hdr = unsafe { &*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr };
        Ok(Cubin {})
    }
}
