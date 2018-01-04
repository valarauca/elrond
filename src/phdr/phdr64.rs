
use super::super::nom::{le_u32, be_u32, be_u64, le_u64, IResult};

use super::super::{
    Abi, ElfAbi, Endian, ElfEndian, Class, ElfClass, ElfMagicNumber,
    Arch, ElfArch, FileType, ElfFileType, ElfHeader, BufferBorrow,
    VarSize, Fault, ElfHeaderBase,
};

use super::flags::{
    Attributes, MemoryAttributes,
    build_attributes
};
use super::ph_type::{
    ProgramHeaderType,HeaderType,
    parse_elf_ph_type_be, parse_elf_ph_type_le
};
use super::traits::PHDR;

use std::sync::Arc;

pub struct Phdr64<'a> {
    header: ElfHeaderBase<'a>,
    p_type: ProgramHeaderType,
    p_flags: Box<[Attributes]>,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}

pub fn parse_phdr64<'a, E: ElfHeader<'a>+?Sized+'a>(index: usize, header: &E)
    -> Result<Box<PHDR<'a>+'a>,Fault>
{
    // ensure we aren't in 32bit mode
    if header.is_32bits() {
        return Err(Fault::Bits64ParserFailed);
    }
    // check for headers
    let size = header.e_phentsize();
    let num = header.e_phnum();
    if size == 0 || index >= num {
        return Err(Fault::HeaderDoesntExist);
    }
    // bounds check + build temp buffer
    let offset = header.e_phoff();
    let start = offset + size * index;
    let end = start + size;
    let buffer = header.get_buffer();
    if start < buffer.len() && end < buffer.len() {
        PrePhdr64::parse(&buffer[start..end],header)
    } else {
        Err(Fault::TooSmol)
    }
}

struct PrePhdr64 {
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
    p_flags: u32,
    p_type: ProgramHeaderType
}
impl PrePhdr64 {
    #[inline(always)]
    fn parse<'a, E: ElfHeader<'a>+?Sized+'a>(temp: &[u8], header: &E)
        -> Result<Box<PHDR<'a>+'a>,Fault>
    {
        let phdr = match header.get_endian() {
            ElfEndian::Little => parse_phdr64_le(temp),
            ElfEndian::Big => parse_phdr64_be(temp)
        };
        // check error
        match phdr {
            IResult::Incomplete(_) => Err(Fault::TooSmol),
            IResult::Error(_) => Err(Fault::Complex),
            IResult::Done(_,ref p) => {
                Ok(Box::new(Phdr64 {
                    header: header.duplicate(),
                    p_offset: p.p_offset.clone(),
                    p_vaddr: p.p_vaddr.clone(),
                    p_paddr: p.p_paddr.clone(),
                    p_filesz: p.p_filesz.clone(),
                    p_memsz: p.p_memsz.clone(),
                    p_flags: build_attributes(p.p_flags.clone()),
                    p_align: p.p_align.clone(),
                    p_type: p.p_type.clone()
                }))
            }
        }
    }
}

named!(parse_phdr64_le<PrePhdr64>,do_parse!(
    p_type: parse_elf_ph_type_le >>
    p_flags: le_u32 >>
    p_offset: le_u64 >>
    p_vaddr: le_u64 >>
    p_paddr: le_u64 >>
    p_filesz: le_u64 >>
    p_memsz: le_u64 >>
    p_align: le_u64 >>
    (PrePhdr64 { p_type,p_offset, p_vaddr, p_paddr,
        p_filesz, p_memsz, p_align, p_flags: p_flags
    })
));
named!(parse_phdr64_be<PrePhdr64>,do_parse!(
    p_type: parse_elf_ph_type_be >>
    p_flags: be_u32 >>
    p_offset: be_u64 >>
    p_vaddr: be_u64 >>
    p_paddr: be_u64 >>
    p_filesz: be_u64 >>
    p_memsz: be_u64 >>
    p_align: be_u64 >>
    (PrePhdr64 { p_type,p_offset, p_vaddr, p_paddr,
        p_filesz, p_memsz, p_align, p_flags: p_flags
    })
));

impl<'a> Abi for Phdr64<'a> {
    #[inline(always)] 
    fn get_abi(&self) -> ElfAbi {
        self.header.get_abi()
    }
}
impl<'a> Endian for Phdr64<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.header.get_endian()
    }
}
impl<'a> FileType for Phdr64<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.header.get_file_type()
    }
}
impl<'a> ElfMagicNumber for Phdr64<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.header.get_abi_version()
    }
}
impl<'a> Arch for Phdr64<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.header.get_arch()
    }
}
impl<'a> Class for Phdr64<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.header.get_class()
    }
}
impl<'a> BufferBorrow<'a> for Phdr64<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.header.get_buffer()
    }
}
impl<'a> MemoryAttributes for Phdr64<'a> {
    #[inline(always)]
    fn get_attributes<'c>(&'c self) -> &'c [Attributes] {
        &self.p_flags
    }
}
impl<'a> HeaderType for Phdr64<'a> {
    #[inline(always)]
    fn get_ptype(&self) -> ProgramHeaderType {
        self.p_type.clone()
    }
}
impl<'a> ElfHeader<'a> for Phdr64<'a> {
    #[inline(always)]
    fn duplicate(&self) -> ElfHeaderBase<'a> {
        self.header.clone()
    }
    #[inline(always)]
    fn e_version(&self) -> usize {
        self.header.e_version()
    }
    #[inline(always)]
    fn e_entry(&self) -> VarSize {
        self.header.e_entry()
    }
    #[inline(always)]
    fn e_phoff(&self) -> usize {
        self.header.e_phoff()
    }
    #[inline(always)]
    fn e_shoff(&self) -> usize {
        self.header.e_shoff()
    }
    #[inline(always)]
    fn e_flags(&self) -> VarSize {
        self.header.e_flags()
    }
    #[inline(always)]
    fn e_ehsize(&self) -> usize {
        self.header.e_ehsize()
    }
    #[inline(always)]
    fn e_phentsize(&self) -> usize {
        self.header.e_phentsize()
    }
    #[inline(always)]
    fn e_phnum(&self) -> usize {
        self.header.e_phnum()
    }
    #[inline(always)]
    fn e_shentsize(&self) -> usize {
        self.header.e_shentsize()
    }
    #[inline(always)]
    fn e_shnum(&self) -> usize {
        self.header.e_shnum()
    }
    #[inline(always)]
    fn e_shstrndx(&self) -> usize {
        self.header.e_shstrndx()
    }
}
impl<'a> PHDR<'a> for Phdr64<'a> {

    #[inline(always)]
    fn p_offset(&self) -> usize {
        self.p_offset.clone()  as usize
    }
    #[inline(always)]
    fn p_vaddr(&self) -> VarSize {
        VarSize::from(self.p_vaddr.clone())
    }
    #[inline(always)]
    fn p_paddr(&self) -> VarSize {
        VarSize::from(self.p_paddr.clone())
    }
    #[inline(always)]
    fn p_filesz(&self) -> usize {
        self.p_filesz.clone()  as usize
    }
    #[inline(always)]
    fn p_memsz(&self) -> VarSize {
        VarSize::from(self.p_memsz.clone())
    }
    #[inline(always)]
    fn p_align(&self) -> VarSize {
        VarSize::from(self.p_align.clone())
    }
}
