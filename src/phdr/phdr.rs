
use super::super::{ElfHeader, ElfClass, Fault};

use super::phdr64::{parse_phdr64};
use super::phdr32::{parse_phdr32};
use super::traits::{PHDR};

pub fn parse_phdr<'a,E: ElfHeader<'a>+?Sized+'a>(index: usize, header: &E)
    -> Result<Box<PHDR<'a>+'a>,Fault>
{
    match header.get_class() {
        ElfClass::Bits64 => parse_phdr64(index, header),
        ElfClass::Bits32 => parse_phdr32(index, header)
    }
}
