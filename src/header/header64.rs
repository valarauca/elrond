
use super::super::nom::{
    IResult, ErrorKind, Needed,
    le_u32, le_u16, le_u64,
    be_u32, be_u16, be_u64,
    Err,
};
use super::super::magic::{
    ElfMagicNumbers, parse_elf_magic,ElfMagicNumber,
    ElfAbi,Abi,
    ElfEndian,Endian,
    ElfClass,Class,
};
use super::super::{VarSize,Fault,BufferBorrow};
use super::header::{ElfHeaderBase};
use super::super::phdr::{parse_phdr,PHDR};
use super::arch::{
    ElfArch, Arch,
    parse_elf_arch_be, parse_elf_arch_le
};
use super::file_type::{
    ElfFileType, FileType,
    parse_elf_file_type_be, parse_elf_file_type_le
};
use super::traits::ElfHeader;
use std::sync::Arc;

#[derive(Clone)]
pub struct Elf64Header<'a> {
    buffer: &'a [u8],
    header: ElfMagicNumbers,
    e_type: ElfFileType,
    e_machine: ElfArch,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

struct E64Pre {
    e_type: ElfFileType,
    e_machine: ElfArch,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[inline(always)]
fn complete<'a>(buffer: &'a [u8], x: E64Pre, header: ElfMagicNumbers)
    -> Elf64Header<'a>
{
    Elf64Header {
        header, buffer,
        e_type: x.e_type,
        e_machine: x.e_machine,
        e_version: x.e_version,
        e_entry: x.e_entry,
        e_phoff: x.e_phoff,
        e_shoff: x.e_shoff,
        e_flags: x.e_flags,
        e_ehsize: x.e_ehsize,
        e_phentsize: x.e_phentsize,
        e_phnum: x.e_phnum,
        e_shentsize: x.e_shentsize,
        e_shnum: x.e_shnum,
        e_shstrndx: x.e_shstrndx,
    }
}
named!(parse_le<E64Pre>, do_parse!(
    e_type: parse_elf_file_type_le >>
    e_machine: parse_elf_arch_le >> 
    e_version: le_u32 >>
    e_entry: le_u64 >>
    e_phoff: le_u64 >>
    e_shoff: le_u64 >>
    e_flags: le_u32 >>
    e_ehsize: le_u16 >>
    e_phentsize: le_u16 >>
    e_phnum: le_u16 >>
    e_shentsize: le_u16 >>
    e_shnum: le_u16 >>
    e_shstrndx: le_u16 >>
    (E64Pre { e_type, e_machine, e_version, e_entry,
        e_phoff, e_shoff, e_flags, e_ehsize, e_phentsize,
        e_phnum, e_shentsize, e_shnum, e_shstrndx
    })
));
named!(parse_be<E64Pre>, do_parse!(
    e_type: parse_elf_file_type_be >>
    e_machine: parse_elf_arch_be >> 
    e_version: be_u32 >>
    e_entry: be_u64 >>
    e_phoff: be_u64 >>
    e_shoff: be_u64 >>
    e_flags: be_u32 >>
    e_ehsize: be_u16 >>
    e_phentsize: be_u16 >>
    e_phnum: be_u16 >>
    e_shentsize: be_u16 >>
    e_shnum: be_u16 >>
    e_shstrndx: be_u16 >>
    (E64Pre { e_type, e_machine, e_version, e_entry,
        e_phoff, e_shoff, e_flags, e_ehsize, e_phentsize,
        e_phnum, e_shentsize, e_shnum, e_shstrndx
    })
));

/// Actually dot he parsing
pub fn parse_elf64_header<'a>(b: &'a [u8])
    -> IResult<&'a [u8], Elf64Header<'a>>
{

    //read header
    let (rem,magic) = match parse_elf_magic(b) {
        IResult::Incomplete(n) => return IResult::Incomplete(n),
        IResult::Error(e) => return IResult::Error(e),
        IResult::Done(x,y) => (x,y)
    };
    
    //determine if we can parse this
    let class = magic.get_class();
    let endian = magic.get_endian();
    let var = match (class,endian) {
        (ElfClass::Bits32,_) => return IResult::Error(Err::Code(ErrorKind::Custom(4))),
        (ElfClass::Bits64,ElfEndian::Little) => parse_le(rem),
        (ElfClass::Bits64,ElfEndian::Big) => parse_be(rem)
    };
    
    //forward error
    match var {
        IResult::Incomplete(n) => IResult::Incomplete(n),
        IResult::Error(e) => IResult::Error(e),
        IResult::Done(r,y) => IResult::Done(r,complete(b,y,magic))
    }
}



impl<'a> Abi for Elf64Header<'a> {
    #[inline(always)]
    fn get_abi(&self) -> ElfAbi {
        self.header.get_abi()
    }
}
impl<'a> Endian for Elf64Header<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.header.get_endian()
    }
}
impl<'a> Class for Elf64Header<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.header.get_class()
    }
}
impl<'a> ElfMagicNumber for Elf64Header<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.header.get_abi_version()
    }
}
impl<'a> FileType for Elf64Header<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.e_type.clone()
    }
}
impl<'a> Arch for Elf64Header<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.e_machine.clone()
    }
}
impl<'a> BufferBorrow<'a> for Elf64Header<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.buffer
    }
}
impl<'a> ElfHeader<'a> for Elf64Header<'a> {
    #[inline(always)]
    fn duplicate(&self) -> ElfHeaderBase<'a> {
        ElfHeaderBase::from(self.clone())
    }
    #[inline(always)]
    fn e_version(&self) -> usize {
        self.e_version.clone() as usize
    }

    #[inline(always)]
    fn e_entry(&self) -> VarSize {
        VarSize::from(self.e_entry.clone())
    }

    #[inline(always)]
    fn e_phoff(&self) -> usize {
        self.e_phoff.clone() as usize
    }

    #[inline(always)]
    fn e_shoff(&self) -> usize {
        self.e_shoff.clone() as usize
    }

    #[inline(always)]
    fn e_flags(&self) -> VarSize {
        VarSize::from(self.e_flags.clone())
    }

    #[inline(always)]
    fn e_ehsize(&self) -> usize {
        self.e_ehsize.clone() as usize
    }

    #[inline(always)]
    fn e_phentsize(&self) -> usize {
        self.e_phentsize.clone() as usize
    }

    #[inline(always)]
    fn e_phnum(&self) -> usize {
        self.e_phnum.clone() as usize
    }

    #[inline(always)]
    fn e_shentsize(&self) -> usize {
        self.e_shentsize.clone() as usize
    }

    #[inline(always)]
    fn e_shnum(&self) -> usize {
        self.e_shnum.clone() as usize
    }

    #[inline(always)]
    fn e_shstrndx(&self) -> usize {
        self.e_shstrndx.clone() as usize
    }
}
