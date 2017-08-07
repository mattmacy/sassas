
use memmap::{Mmap, Protection};
use std::{io, slice};
use std::collections::{VecDeque, HashMap};
use itertools::zip;
use sval::{SVal, SymBind, KernelSection, SecHdr, ElfSymbol};
use unsafe_lib::MutStrMap;
use elf;
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr,
          Elf64_Sym};

pub struct Cubin {
    pub table: MutStrMap<MutStrMap<SVal>>,
}
static SYMBIND: [SymBind; 3] = [SymBind::Local, SymBind::Global, SymBind::Weak];

enum ElfSecHdrs {
    Elf32Shdrs(Vec<Elf32_Shdr>),
    Elf64Shdrs(Vec<Elf64_Shdr>),
}

impl Cubin {
    pub fn new(file: String) -> io::Result<Self> {
        let fp = Mmap::open_path(file, Protection::Read)?;
        let elf32_hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
        if !(elf32_hdr.e_type as u32 == elf::ELFCLASS32 ||
                 elf32_hdr.e_type as u32 == elf::ELFCLASS64)
        {
            panic!("invalid class type: {}", elf32_hdr.e_type);
        }
        Self::build(elf32_hdr.e_type as u32, fp)
    }
    fn build(hdrtype: u32, fp: Mmap) -> io::Result<Self> {
        let mut cubintbl: MutStrMap<MutStrMap<SVal>> = MutStrMap::new();
        let mut shdrvals = Vec::new();
        let (stridx, elf_shdrs);

        if hdrtype == elf::ELFCLASS32 {
            cubintbl["Fields"]["Class"] = 32.into();
            let hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
            stridx = hdr.e_shstrndx;
            let arch = (hdr.e_flags & 0xff) as u32;
            cubintbl["Fields"]["Arch"] = arch.into();
            let addr_size = if hdr.e_flags & 0x400 == 0x400 { 64 } else { 32 };
            cubintbl["Fields"]["AddressSize"] = addr_size.into();
            let (off, len) = (hdr.e_phoff as isize, hdr.e_phentsize as usize);

            let phdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Phdr, len)
                         }).to_vec();
            cubintbl["Fields"]["prgHdrs"] = phdrs.into();
            let (off, len) = (hdr.e_shoff as isize, hdr.e_shentsize as usize);
            let shdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Shdr, len)
                         }).to_vec();
            elf_shdrs = ElfSecHdrs::Elf32Shdrs(shdrs.clone());
            cubintbl["Fields"]["secHdrs"] = shdrs.clone().into();
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
                            v.push(ElfSymbol::Sym32(sym.clone()));
                            offset += shdr.sh_entsize;
                        }
                        SecHdr::SymTab(v, data.to_vec())
                    }
                    _ => SecHdr::Other(shdr.sh_type, data.to_vec()),
                };
                shdrvals.push(sh);
            }
        } else {
            cubintbl["Fields"]["Class"] = 64.into();
            let hdr = unsafe { &*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr };
            stridx = hdr.e_shstrndx;
            let arch = (hdr.e_flags & 0xff) as u32;
            cubintbl["Fields"]["Arch"] = arch.into();
            let addr_size = if hdr.e_flags & 0x400 == 0x400 { 64 } else { 32 };
            cubintbl["Fields"]["AddressSize"] = addr_size.into();
            let (off, len) = (hdr.e_phoff as isize, hdr.e_phentsize as usize);
            let phdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf64_Phdr, len)
                         }).to_vec();
            cubintbl["Fields"]["prgHdrs"] = phdrs.into();
            let (off, len) = (hdr.e_shoff as isize, hdr.e_shentsize as usize);
            let shdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf64_Shdr, len)
                         }).to_vec();
            elf_shdrs = ElfSecHdrs::Elf64Shdrs(shdrs.clone());
            cubintbl["Fields"]["secHdrs"] = shdrs.clone().into();
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
                                &*(fp.ptr().offset(off + offset as isize) as *const Elf64_Sym) as
                                    &Elf64_Sym
                            };
                            v.push(ElfSymbol::Sym64(sym.clone()));
                            offset += shdr.sh_entsize;
                        }
                        SecHdr::SymTab(v, data.to_vec())
                    }
                    _ => SecHdr::Other(shdr.sh_type, data.to_vec()),
                };
                shdrvals.push(sh);
            }
        }
        let strtab = match shdrvals[stridx as usize] {
            SecHdr::StrTab(ref v, _) => v.clone(),
            _ => panic!("strtab not found"),
        };
        let mut shdrmap: HashMap<String, (SVal, SecHdr)> = HashMap::new();
        match elf_shdrs {
            ElfSecHdrs::Elf32Shdrs(ref shdrs) => {
                for (shdr, sh) in zip(shdrs, &shdrvals) {
                    let name = strtab[shdr.sh_name as usize].clone();
                    shdrmap.insert(name.clone(), (shdr.clone().into(), sh.clone()));
                    cubintbl["SecHdrs"][&name] = sh.clone().into();
                }
            }
            ElfSecHdrs::Elf64Shdrs(ref shdrs) => {
                for (shdr, sh) in zip(shdrs, &shdrvals) {
                    let name = strtab[shdr.sh_name as usize].clone();
                    shdrmap.insert(name.clone(), (shdr.clone().into(), sh.clone()));
                    cubintbl["SecHdrs"][&name] = sh.clone().into();
                }
            }

        }
        let &(_, ref symtab_) = &shdrmap[".symtab"];
        let symtab = match symtab_ {
            &SecHdr::SymTab(ref t, _) => t,
            _ => panic!("expect Symtab"),
        };
        for syment in symtab {
            let symname = strtab[syment.name()].clone();
            let (flags, info) = match elf_shdrs {
                ElfSecHdrs::Elf32Shdrs(ref shdrs) => {
                    let shdr = &shdrs[syment.shndx()];
                    (
                        <u64 as ::num::NumCast>::from(shdr.sh_flags).unwrap(),
                        shdr.sh_info,
                    ) as (u64, u32)
                }
                ElfSecHdrs::Elf64Shdrs(ref shdrs) => {
                    let shdr = &shdrs[syment.shndx()];
                    (shdr.sh_flags, shdr.sh_info) as (u64, u32)
                }
            };
            let shval = &shdrvals[syment.shndx()];
            if syment.info() & 0x10 == 0x10 {
                cubintbl["Symbols"][&symname] = syment.clone().into();
                continue;
            }
            // Skip sections not tagged FUNC
            if syment.info() & 0x0f != 0x02 {
                continue;
            }
            cubintbl["Symbols"][&symname] = syment.clone().into();

            // Create a hash of kernels for output
            //my $kernelSec = $cubin->{Kernels}{$symEnt->{Name}} = $secHdr;
            //kernsecmap.insert(symname.clone(), sh.clone());
            let mut kernel_secmap: HashMap<&'static str, SVal> = HashMap::new();

            // Extract local/global/weak binding info
            kernel_secmap.insert("Linkage", SYMBIND[((syment.info() & 0xf0) >> 4)].into());
            // Extract the kernel instructions
            let data = match shval {
                &SecHdr::Other(_, ref data) => data,
                _ => panic!("unexpected hdr: {:?}", shval),
            };
            kernel_secmap.insert("Data", data.clone().into());
            // Extract the max barrier resource identifier used and add 1. Should be 0-16.
            // If a register is used as a barrier resource id, then this value is the max of 16.
            kernel_secmap.insert("BarCnt", (((flags & 0x01f00000) >> 20) as u32).into());
            // Extract the number of allocated registers for this kernel.
            kernel_secmap.insert("RegCnt", (((info & 0xff000000) >> 24) as u32).into());

            // Extract the size of shared memory this kernel uses.
            let shared_name = format!(".nv.shared.{}", symname);
            let (size, shared_sec) = if shdrmap.contains_key(&shared_name) {
                let &(ref shdr, ref sh) = &shdrmap[&shared_name];
                match *shdr {
                    SVal::Elf32Shdr(s) => (s.sh_size as u32, Some(sh.clone())),
                    SVal::Elf64Shdr(s) => (s.sh_size as u32, Some(sh.clone())),
                    _ => unreachable!(),
                }
            } else {
                (0, None)
            };
            //kernel_sec.insert("SharedSec", shared_sec.clone().into());
            kernel_secmap.insert("SharedSize", (size as u32).into());

            // Attach constant0 section
            let &(ref constsh, ref constant_sec) = &shdrmap[&format!(".nv.constant0.{}", symname)];

            // Extract the kernel parameter data.
            let infoname = format!(".nv.info.{}", symname);

            if !shdrmap.contains_key(&infoname) {
                continue;
            }
            let &(ref paramsh, ref paramshval) = &shdrmap[&infoname];
            let data = match paramshval {
                &SecHdr::Other(_, ref data) => unsafe {
                    ::std::mem::transmute::<Vec<u8>, Vec<u32>>(data.clone())
                },
                _ => panic!("got unexpected hdr type: {:?}", paramshval),
            };
            kernel_secmap.insert("ParamSec", Self::extract_param_sec(data).into());
            let kernel_sec = KernelSection {
                name: symname.clone(),
                shared_sec: shared_sec,
                constant_sec: constant_sec.clone(),
                map: kernel_secmap,
            };
            cubintbl["Kernels"][&symname] = kernel_sec.into();
        }

        Ok(Cubin { table: cubintbl })
    }
    fn extract_param_sec(data: Vec<u32>) -> MutStrMap<SVal> {
        let mut param_sec = MutStrMap::new();
        param_sec.insert("ParamData", SVal::DataL(data.clone()));
        let hex32 = data.iter()
            .map(|v| format!("0x{:08x}", *v))
            .collect::<Vec<String>>();
        param_sec.insert("ParamHex", hex32.into());
        // find the first param delimiter
        let mut idx = 0;
        while idx < data.len() && data[idx] != 0x00080a04 {
            idx += 1;
        }
        let first = data[idx + 2] & 0xFFFF;
        idx += 4;
        let mut params = VecDeque::new();
        while idx < data.len() && data[idx] == 0x000c1704 {
            let ord = data[idx + 2] & 0xFFFF;
            let offset = format!("0x{:02x}", first + (data[idx + 2] >> 16));
            let psize = data[idx + 3] >> 18;
            let align = if data[idx + 3] & 0x400 == 0x400 {
                1 << (data[idx + 3] & 0x3ff)
            } else {
                0
            };
            let param = format!("{}:{}:{}:{}", ord, offset, psize, align);
            params.push_front(param);
            idx += 4;
        }
        let static_params = data[0..idx - 1].to_vec();
        param_sec.insert("StaticParams", static_params.into());
        while idx < data.len() {
            let code = data[idx] & 0xFFFF;
            let size = data[idx] >> 16;
            let step = (size / 4) as usize;
            idx += 1;
            let slice = data[idx..idx + step].to_vec();
            match code {
                // EIATTR_MAXREG_COUNT
                0x1b03 => param_sec.insert("MAXREG_COUNT", size.into()),
                // EIATTR_S2RCTAID_INSTR_OFFSETS
                0x1d04 => param_sec.insert("CTAIDOffsets", slice.into()),
                // EIATTR_EXIT_INSTR_OFFSETS
                0x1c04 => param_sec.insert("ExitOffsets", slice.into()),
                // EIATTR_CTAIDZ_USED
                0x0401 => param_sec.insert("CTADIZUsed", true.into()),
                // EIATTR_REQNTID
                0x1004 => param_sec.insert("REQNTID", slice.into()),
                // EIATTR_MAX_THREADS
                0x0504 => param_sec.insert("MAXNTID", slice.into()),
                // EIATTR_CRS_STACK_SIZE
                0x1e04 => param_sec.insert("STACKSIZE", slice.into()),
                _ => {
                    println!("Unknown Code 0x{:02x} (size:{})\n", code, size);
                    None
                }
            };
            idx += step;
        }
        param_sec
    }
}
