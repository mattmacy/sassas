
use memmap::{Mmap, Protection};
use std::io;
use std::io::{Read, Write};
use elf::{Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr, Elf32_Sym, Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr, Elf64_Sym, ELFCLASS32, ELFCLASS64};

pub struct Cubin {

}

impl Cubin {
	pub fn new(file: String) -> io::Result<Self> {
		let fp = Mmap::open_path(file, Protection::Read)?;
		let elf32_hdr = unsafe {&*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr};
		let class = elf32_hdr.e_type;
		if class == ELFCLASS32 as u16{
			Self::new32(fp)
		} else if class == ELFCLASS64 as u16 {
			Self::new64(fp)			
		} else {
			panic!("invalid class type: {}", class)
		}
	}
	fn new32(fp: Mmap) -> io::Result<Self> {
		let hdr = unsafe {&*(fp.ptr() as *const Elf32_Ehdr) as &Elf32_Ehdr};

		Ok(Cubin {})
	}
	fn new64(fp: Mmap) -> io::Result<Self> {
		let hdr = unsafe {&*(fp.ptr() as *const Elf64_Ehdr) as &Elf64_Ehdr};
		Ok(Cubin {})
	}
}