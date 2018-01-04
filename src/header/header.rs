
use super::super::nom::IResult;

use super::super::{
    Abi, ElfAbi, Endian, ElfEndian, Class, ElfClass, ElfMagicNumber,
    Arch, ElfArch, FileType, ElfFileType, ElfHeader, BufferBorrow,
    VarSize, Fault
};
use super::header32::{parse_elf32_header,Elf32Header};
use super::header64::{parse_elf64_header,Elf64Header};

use std::sync::Arc;

named!(parse_elf_header<ElfHeaderBase>,
    alt_complete!( elf64 | elf32 )
);

/// This type wraps a trait object
///
/// It allows us to avoid allocations when
/// passing around the inner super object
#[derive(Clone)]
pub struct ElfHeaderBase<'a> {
    data: Arc<ElfHeader<'a>+'a>,
}
impl<'a> From<Elf32Header<'a>> for ElfHeaderBase<'a> {
    fn from(x: Elf32Header<'a>) -> ElfHeaderBase<'a> {
        ElfHeaderBase{ data: Arc::new(x) }
    }
}
impl<'a> From<Elf64Header<'a>> for ElfHeaderBase<'a> {
    fn from(x: Elf64Header<'a>) -> ElfHeaderBase<'a> {
        ElfHeaderBase{ data: Arc::new(x) }
    }
}
impl<'a> Abi for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_abi(&self) -> ElfAbi {
        self.data.as_ref().get_abi()
    }
}
impl<'a> Endian for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.data.as_ref().get_endian()
    }
}
impl<'a> Class for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.data.as_ref().get_class()
    }
}
impl<'a> ElfMagicNumber for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.data.as_ref().get_abi_version()
    }
}
impl<'a> FileType for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.data.as_ref().get_file_type()
    }
}
impl<'a> Arch for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.data.as_ref().get_arch()
    }
}
impl<'a> BufferBorrow<'a> for ElfHeaderBase<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.data.as_ref().get_buffer()
    }
}
impl<'a> ElfHeader<'a> for ElfHeaderBase<'a> {
    #[inline(always)]
    fn duplicate(&self) -> ElfHeaderBase<'a> {
        self.clone()
    }
    #[inline(always)]
    fn e_version(&self) -> usize {
        self.data.as_ref().e_version()
    }

    #[inline(always)]
    fn e_entry(&self) -> VarSize {
        self.data.as_ref().e_entry()
    }

    #[inline(always)]
    fn e_phoff(&self) -> usize {
        self.data.as_ref().e_phoff()
    }

    #[inline(always)]
    fn e_shoff(&self) -> usize {
        self.data.as_ref().e_shoff()
    }

    #[inline(always)]
    fn e_flags(&self) -> VarSize {
        self.data.as_ref().e_flags()
    }

    #[inline(always)]
    fn e_ehsize(&self) -> usize {
        self.data.as_ref().e_ehsize()
    }

    #[inline(always)]
    fn e_phentsize(&self) -> usize {
        self.data.as_ref().e_phentsize()
    }

    #[inline(always)]
    fn e_phnum(&self) -> usize {
        self.data.as_ref().e_phnum()
    }

    #[inline(always)]
    fn e_shentsize(&self) -> usize {
        self.data.as_ref().e_shentsize()
    }

    #[inline(always)]
    fn e_shnum(&self) -> usize {
        self.data.as_ref().e_shnum()
    }

    #[inline(always)]
    fn e_shstrndx(&self) -> usize {
        self.data.as_ref().e_shstrndx()
    }
}
fn elf64<'a>(b: &'a [u8]) -> IResult<&'a [u8],ElfHeaderBase<'a>> {
    match parse_elf64_header(b) {
        IResult::Incomplete(n) => IResult::Incomplete(n),
        IResult::Error(e) => IResult::Error(e),
        IResult::Done(x,y) => IResult::Done(x,ElfHeaderBase{ data: Arc::new(y) })
    }
}
fn elf32<'a>(b: &'a [u8]) -> IResult<&'a [u8],ElfHeaderBase<'a>> {
    match parse_elf32_header(b) {
        IResult::Incomplete(n) => IResult::Incomplete(n),
        IResult::Error(e) => IResult::Error(e),
        IResult::Done(x,y) => IResult::Done(x,ElfHeaderBase{ data: Arc::new(y) })
    }
}

    

pub fn parse_elf<'a>(buffer: &'a [u8])
    -> Result<ElfHeaderBase<'a>,Fault>
{
    match parse_elf_header(buffer) {
        IResult::Incomplete(_) => Err(Fault::TooSmol),
        IResult::Error(_) => Err(Fault::Complex),
        IResult::Done(_,x) => Ok(x)
    }
}
