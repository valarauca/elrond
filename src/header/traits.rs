use super::super::magic::{ElfMagicNumber};
use super::super::varsize::{VarSize};
use super::super::phdr::{parse_phdr,PHDR};
use super::super::{Fault,BufferBorrow};
use super::super::header::{ElfHeaderBase};
use super::super::section::{Section, Sections};

use super::file_type::FileType;
use super::arch::Arch;

use std::sync::Arc;

/// Header of an elf file
pub trait ElfHeader<'a>: ElfMagicNumber + Arch + FileType + BufferBorrow<'a> + 'a {
    fn duplicate(&self) -> ElfHeaderBase<'a>;
    fn e_version(&self) -> usize;
    fn e_entry(&self) -> VarSize;
    fn e_phoff(&self) -> usize;
    fn e_shoff(&self) -> usize;
    fn e_flags(&self) -> VarSize;
    fn e_ehsize(&self) -> usize;
    fn e_phentsize(&self) -> usize;
    fn e_phnum(&self) -> usize;
    fn e_shentsize(&self) -> usize;
    fn e_shnum(&self) -> usize;
    fn e_shstrndx(&self) -> usize;

    fn has_program_headers(&self) -> bool {
        self.e_phnum() > 0
            &&
        self.e_phentsize() > 0
    }

    fn has_section_headers(&self) -> bool {
        self.e_shnum() > 0
            &&
        self.e_shentsize() > 0
    }

    fn get_phdr(&self) -> Result<Box<[Box<PHDR<'a>+'a>]>,Fault> {
        let headers = self.e_phnum();
        let mut ret_vec = Vec::with_capacity(headers);
        for index in 0..headers {
            ret_vec.push(parse_phdr(index,self)?);
        }
        Ok(ret_vec.into_boxed_slice())
    }
    
    fn get_sections(&self) -> Result<Sections<'a>,Fault> {
        Sections::new(self)
    }
}

