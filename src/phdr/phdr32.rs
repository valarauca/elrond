
use super::super::nom::{le_u32, be_u32, IResult};

use super::super::{
    Abi, ElfAbi, Endian, ElfEndian, Class, ElfClass, ElfMagicNumber,
    Arch, ElfArch, FileType, ElfFileType, BufferBorrow,
    VarSize, Fault, ElfHeader, ElfHeaderBase
};

use super::flags::{Attributes, MemoryAttributes, build_attributes};
use super::ph_type::{
    ProgramHeaderType,HeaderType,
    parse_elf_ph_type_be, parse_elf_ph_type_le
};
use super::traits::PHDR;

use std::sync::Arc;

pub struct Phdr32<'a> {
    header: ElfHeaderBase<'a>,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: Box<[Attributes]>,
    p_align: u32,
    p_type: ProgramHeaderType,
}

pub fn parse_phdr32<'a, E: ElfHeader<'a>+'a+?Sized>(index: usize, header: &E)
    -> Result<Box<PHDR<'a>+'a>,Fault>
{
    // ensure we aren't in 64bit mode
    if header.is_64bits() {
        return Err(Fault::Bits32ParserFailed);
    }
    // check for headers
    let size = header.e_phentsize();
    let num = header.e_shnum();
    if size == 0 || index >= num {
        return Err(Fault::HeaderDoesntExist);
    }
    // bounds check + build temp buffer
    let offset = header.e_phoff();
    let start = offset + size * index;
    let end = start + size;
    let buffer = header.get_buffer();
    let temp = if start < buffer.len() && end < buffer.len() {
        &buffer[start..end]
    } else {
        return Err(Fault::TooSmol);
    };
    PrePhdr32::parse(temp, header)
}

struct PrePhdr32 {
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
    p_type: ProgramHeaderType
}
impl PrePhdr32 {
    #[inline(always)]
    pub fn parse<'a, E:ElfHeader<'a>+'a+?Sized>(temp: &[u8], header: &E)
        -> Result<Box<PHDR<'a>+'a>,Fault>
    {
        let phdr = match header.get_endian() {
            ElfEndian::Little => parse_phdr32_le(temp),
            ElfEndian::Big => parse_phdr32_be(temp)
        };
        // check error
        match phdr {
            IResult::Incomplete(_) => Err(Fault::TooSmol),
            IResult::Error(_) => Err(Fault::Complex),
            IResult::Done(_,ref p) => {
                Ok(Box::new(Phdr32 {
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

named!(parse_phdr32_le<PrePhdr32>,do_parse!(
    p_type: parse_elf_ph_type_le >>
    p_offset: le_u32 >>
    p_flags: le_u32 >>
    p_vaddr: le_u32 >>
    p_paddr: le_u32 >>
    p_filesz: le_u32 >>
    p_memsz: le_u32 >>
    p_flags: le_u32 >>
    p_align: le_u32 >>
    ( PrePhdr32{ p_type,p_offset, p_vaddr, p_paddr,
        p_filesz, p_memsz, p_align, p_flags
    } )
));
named!(parse_phdr32_be<PrePhdr32>,do_parse!(
    p_type: parse_elf_ph_type_be >>
    p_offset: be_u32 >>
    p_flags: be_u32 >>
    p_vaddr: be_u32 >>
    p_paddr: be_u32 >>
    p_filesz: be_u32 >>
    p_memsz: be_u32 >>
    p_flags: be_u32 >>
    p_align: be_u32 >>
    ( PrePhdr32{ p_type,p_offset, p_vaddr, p_paddr,
        p_filesz, p_memsz, p_align, p_flags
    } )
));

impl<'a> Abi for Phdr32<'a> {
    #[inline(always)] 
    fn get_abi(&self) -> ElfAbi {
        self.header.get_abi()
    }
}
impl<'a> Endian for Phdr32<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.header.get_endian()
    }
}
impl<'a> FileType for Phdr32<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.header.get_file_type()
    }
}
impl<'a> ElfMagicNumber for Phdr32<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.header.get_abi_version()
    }
}
impl<'a> Arch for Phdr32<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.header.get_arch()
    }
}
impl<'a> Class for Phdr32<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.header.get_class()
    }
}
impl<'a> BufferBorrow<'a> for Phdr32<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.header.get_buffer()
    }
}
impl<'a> MemoryAttributes for Phdr32<'a> {
    #[inline(always)]
    fn get_attributes<'c>(&'c self) -> &'c [Attributes] {
        &self.p_flags
    }
}
impl<'a> HeaderType for Phdr32<'a> {
    #[inline(always)]
    fn get_ptype(&self) -> ProgramHeaderType {
        self.p_type.clone()
    }
}
impl<'a> ElfHeader<'a> for Phdr32<'a> {
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
impl<'a> PHDR<'a> for Phdr32<'a> {

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
