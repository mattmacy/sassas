
use memmap::{Mmap, Protection};
use std::{io, slice, mem};
use std::collections::{VecDeque, HashMap};
use itertools::zip;
use sval::{SVal, SymBind, KernelSection, SecHdr, ElfSymbol};
use unsafe_lib::MutStrMap;
use elf;
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr,
          Elf64_Sym};

pub struct Cubin {
    pub arch: u32,
    pub class: u32,
    pub addr_size: u32,
    pub table: MutStrMap<MutStrMap<SVal>>,
}
static SYMBIND: [SymBind; 3] = [SymBind::Local, SymBind::Global, SymBind::Weak];

enum ElfSecHdrs {
    Elf32Shdrs(Vec<Elf32_Shdr>),
    Elf64Shdrs(Vec<Elf64_Shdr>),
}

impl Cubin {
    pub fn is_elf(file: &String) -> io::Result<bool> {
        let fp = Mmap::open_path(file, Protection::Read)?;
        let hdr_ref: Vec<u8> = vec![0x7f, 0x45, 0x4c, 0x46];
        let hdr = (unsafe { slice::from_raw_parts(fp.ptr() as *const u8, 4) }).to_vec();
        for (h, hr) in zip(hdr, hdr_ref) {
            if h != hr {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn build_strtab(data: &[u8]) -> SecHdr {
        let strtab = data.split(|ch| *ch == b'\0')
            .map(|slice| String::from_utf8(slice.to_vec()).unwrap())
            .collect::<Vec<String>>();
        let mut idx = 0;
        let mut strmap: HashMap<usize, String> = HashMap::new();
        for s in strtab {
            let len = s.len();
            strmap.insert(idx, s.clone());
            idx += len + 1;
        }
        SecHdr::StrTab(strmap, data.to_vec())

    }
    pub fn new(file: &String) -> io::Result<Self> {
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
        let cubintbl: MutStrMap<MutStrMap<SVal>> = MutStrMap::new();
        let mut shdrvals = Vec::new();
        let (stridx, elf_shdrs);
        let mut cubin = Cubin {
            arch: 0,
            class: 0,
            addr_size: 0,
            table: cubintbl,
        };
        if hdrtype == elf::ELFCLASS32 {
            let hdr = unsafe { &*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr };
            stridx = hdr.e_shstrndx;
            cubin.class = 32;
            cubin.arch = (hdr.e_flags & 0xff) as u32;
            cubin.addr_size = if hdr.e_flags & 0x400 == 0x400 { 64 } else { 32 };
            let (off, len) = (hdr.e_phoff as isize, hdr.e_phentsize as usize);

            let phdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Phdr, len)
                         }).to_vec();
            cubin.table["Fields"]["prgHdrs"] = phdrs.into();
            let (off, len) = (hdr.e_shoff as isize, hdr.e_shnum as usize);
            let shdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf32_Shdr, len)
                         }).to_vec();
            elf_shdrs = ElfSecHdrs::Elf32Shdrs(shdrs.clone());
            cubin.table["Fields"]["secHdrs"] = shdrs.clone().into();
            for shdr in &shdrs {
                let (off, len) = (shdr.sh_offset as isize, shdr.sh_size as usize);
                let data = unsafe { slice::from_raw_parts(fp.ptr().offset(off) as *const u8, len) };
                if shdr.sh_type == elf::SHT_NOBITS || shdr.sh_size == 0 {
                    shdrvals.push(SecHdr::Empty);
                    continue;
                }
                let sh = match shdr.sh_type {
                    elf::SHT_STRTAB => Self::build_strtab(data),
                    elf::SHT_SYMTAB => {
                        let mut v = Vec::new();
                        let mut offset = 0;
                        while offset < shdr.sh_size {
                            let sym = unsafe {
                                &*(fp.ptr().offset(off + offset as isize) as *const Elf32_Sym) as
                                    &Elf32_Sym
                            };
                            v.push(ElfSymbol::Sym32(*sym));
                            offset += shdr.sh_entsize;
                        }
                        SecHdr::SymTab(v, data.to_vec())
                    }
                    _ => SecHdr::Other(shdr.sh_type, data.to_vec()),
                };
                shdrvals.push(sh);
            }
        } else {
            let hdr = unsafe { &*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr };
            stridx = hdr.e_shstrndx;
            cubin.class = 64;
            cubin.arch = (hdr.e_flags & 0xff) as u32;
            cubin.addr_size = if hdr.e_flags & 0x400 == 0x400 { 64 } else { 32 };
            let (off, len) = (hdr.e_phoff as isize, hdr.e_phnum as usize);
            let phdrs = (unsafe {
                             slice::from_raw_parts(fp.ptr().offset(off) as *const Elf64_Phdr, len)
                         }).to_vec();
            cubin.table["Fields"]["prgHdrs"] = phdrs.into();
            let (off, len) = (hdr.e_shoff as isize, hdr.e_shentsize as usize);
            let shdrs = (unsafe {
                             slice::from_raw_parts(
                    fp.ptr().offset(off) as *const Elf64_Shdr,
                    hdr.e_shnum as usize,
                )
                         }).to_vec();
            elf_shdrs = ElfSecHdrs::Elf64Shdrs(shdrs.clone());
            cubin.table["Fields"]["secHdrs"] = shdrs.clone().into();
            for shdr in &shdrs {
                let (off, len) = (shdr.sh_offset as isize, shdr.sh_size as usize);
                let data = unsafe { slice::from_raw_parts(fp.ptr().offset(off) as *const u8, len) };
                if shdr.sh_type == elf::SHT_NOBITS || shdr.sh_size == 0 {
                    shdrvals.push(SecHdr::Empty);
                    continue;
                }
                let sh = match shdr.sh_type {
                    elf::SHT_STRTAB => Self::build_strtab(data),
                    elf::SHT_SYMTAB => {
                        let mut v = Vec::new();
                        let mut offset = 0;
                        while offset < shdr.sh_size {
                            let sym = unsafe {
                                &*(fp.ptr().offset(off + offset as isize) as *const Elf64_Sym) as
                                    &Elf64_Sym
                            };
                            v.push(ElfSymbol::Sym64(*sym));
                            offset += shdr.sh_entsize;
                        }
                        SecHdr::SymTab(v, data.to_vec())
                    }
                    _ => SecHdr::Other(shdr.sh_type, data.to_vec()),
                };
                shdrvals.push(sh);
            }
        }

        let shstrtab = match shdrvals[stridx as usize] {
            SecHdr::StrTab(ref v, _) => v.clone(),
            _ => panic!("strtab not found {:?}", shdrvals[stridx as usize]),
        };
        let mut shdrmap: HashMap<String, (SVal, SecHdr)> = HashMap::new();
        match elf_shdrs {
            ElfSecHdrs::Elf32Shdrs(ref shdrs) => {
                for (shdr, sh) in zip(shdrs, &shdrvals) {
                    let namidx = shdr.sh_name as usize;
                    if namidx == 0 {
                        continue;
                    }
                    let name = shstrtab[&namidx].clone();
                    //                    let sval : SVal = *shdr.into();
                    shdrmap.insert(name.clone(), ((*shdr).into(), sh.clone()));
                    cubin.table["SecHdrs"][&name] = sh.clone().into();
                }
            }
            ElfSecHdrs::Elf64Shdrs(ref shdrs) => {
                for (shdr, sh) in zip(shdrs, &shdrvals) {
                    let namidx = shdr.sh_name as usize;
                    if namidx == 0 {
                        continue;
                    }
                    let name = shstrtab[&namidx].clone();
                    shdrmap.insert(name.clone(), ((*shdr).into(), sh.clone()));
                    cubin.table["SecHdrs"][&name] = sh.clone().into();
                }
            }

        }
        let &(_, ref symtab_) = &shdrmap[".symtab"];
        let symtab = match symtab_ {
            &SecHdr::SymTab(ref t, _) => t,
            _ => panic!("expect Symtab"),
        };
        let &(_, ref strtab_) = &shdrmap[".strtab"];
        let strtab = match strtab_ {
            &SecHdr::StrTab(ref v, _) => v.clone(),
            _ => panic!("strtab not found {:?}", strtab_),
        };
        for syment in symtab {
            let symname = strtab[&syment.name()].clone();
            let (flags, info, size) = match elf_shdrs {
                ElfSecHdrs::Elf32Shdrs(ref shdrs) => {
                    let shdr = &shdrs[syment.shndx()];
                    (shdr.sh_flags as u64, shdr.sh_info, shdr.sh_size as u64)
                }
                ElfSecHdrs::Elf64Shdrs(ref shdrs) => {
                    let shdr = &shdrs[syment.shndx()];
                    (shdr.sh_flags, shdr.sh_info, shdr.sh_size)
                }
            };
            let shval = &shdrvals[syment.shndx()];
            if syment.info() & 0x10 == 0x10 {
                cubin.table["Symbols"][&symname] = syment.clone().into();
            }
            // Skip sections not tagged FUNC
            if syment.info() & 0x0f != 0x02 {
                continue;
            }
            cubin.table["Symbols"][&symname] = syment.clone().into();

            // Create a hash of kernels for output
            //my $kernelSec = $cubin->{Kernels}{$symEnt->{Name}} = $secHdr;
            //kernsecmap.insert(symname.clone(), sh.clone());
            let mut kernel_sec = KernelSection::default();

            // Extract local/global/weak binding info
            kernel_sec.linkage = SYMBIND[((syment.info() & 0xf0) >> 4)];
            // Extract the max barrier resource identifier used and add 1. Should be 0-16.
            // If a register is used as a barrier resource id, then this value is the max of 16.
            kernel_sec.bar_cnt = ((flags & 0x01f0_0000) >> 20) as u32;
            // Extract the number of allocated registers for this kernel.
            kernel_sec.reg_cnt = ((info & 0xff00_0000) >> 24) as u32;
            // Extract the kernel instructions
            let data = match shval {
                &SecHdr::Other(_, ref data) => data,
                _ => panic!("unexpected hdr: {:?}", shval),
            };
            kernel_sec.map.insert("Data", data.clone().into());

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
            kernel_sec.shared_sec = shared_sec;
            kernel_sec.shared_size = size as u32;

            // Attach constant0 section
            let &(ref constsh, ref constant_sec) = &shdrmap[&format!(".nv.constant0.{}", symname)];
            kernel_sec.constant_sec = constant_sec.clone();

            // Extract the kernel parameter data.
            let infoname = format!(".nv.info.{}", symname);

            if !shdrmap.contains_key(&infoname) {
                println!("{}: not found", symname);
                continue;
            }
            let &(ref paramsh, ref paramshval) = &shdrmap[&infoname];
            let data: Vec<u32> = match paramshval {
                &SecHdr::Other(_, ref data) => {
                    let mut data = data.clone();
                    let capacity = data.capacity();
                    let (ptr, len, capacity) = {
                        let slice = &mut data[..];
                        (slice.as_mut_ptr() as _, slice.len() / 4, capacity / 4)
                    };
                    assert!(ptr as usize % 4 == 0); // check alignment
                    mem::forget(data); // bye bye!
                    unsafe { Vec::from_raw_parts(ptr, len, capacity) }
                }
                _ => panic!("got unexpected hdr type: {:?}", paramshval),
            };
            let params = Self::extract_param_sec(data);
            kernel_sec.param_cnt = params.len();
            kernel_sec.map.insert("ParamSec", params.into());
            cubin.table["Kernels"][&symname] = kernel_sec.into();
        }
        Ok(cubin)
    }
    pub fn list_kernels(&self) -> MutStrMap<SVal> {
        self.table["Kernels"].clone()
    }
    pub fn list_symbols(&self) -> MutStrMap<SVal> {
        self.table["Symbols"].clone()
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
        while idx < data.len() && data[idx] != 0x0008_0a04 {
            idx += 1;
        }
        let first = data[idx + 2] & 0xFFFF;
        idx += 4;
        let mut params = VecDeque::new();
        while idx < data.len() && data[idx] == 0x000c_1704 {
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
            let (step, slice) = match code {
                0x0401 | 0x1b03 => (1, vec![]),
                _ => {
                    let step = (size / 4) as usize;
                    (step + 1, data[idx..idx + step].to_vec())
                }
            };
            idx += step;
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
        }
        param_sec
    }
}
