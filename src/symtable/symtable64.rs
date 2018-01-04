
use super::super::nom::{
    le_u64, be_u64,
    le_u32, be_u32,
    le_u16, be_u16,
    le_u8,  be_u8,
    IResult
};

use super::super::{
    Abi, ElfAbi, Endian, ElfEndian, Class, ElfClass, ElfMagicNumber,
    Arch, ElfArch, FileType, ElfFileType, ElfHeader, BufferBorrow,
    VarSize, Fault, ElfHeaderBase, Section
};

use super::{
    SymBinding, SymbolBinding,
    SymType, SymbolType,
    SymProt, SymbolProtection,
    find_null,
    SymEntry,
};

use std::borrow::Cow;
use std::sync::Arc;
use std::mem::size_of;

pub struct SymEntry64<'a> {
    header: ElfHeaderBase<'a>,
    name: Option<Cow<'a,str>>,
    st_bind: SymBinding,
    st_type: SymType,
    st_prot: SymProt,
    st_size: u64,
    st_value: u64,
    st_shndx: u16,
}
impl<'a> SymEntry64<'a> {
    fn new<S>(data: &PreSymEntry64, strtab: &S)
        -> Result<SymEntry64<'a>, Fault>
        where S: Section<'a>+'a+?Sized
    {
        let name = data.st_name as usize;
        let name = get_name(strtab, name)?;
        let st_prot = SymProt::from(data.st_other);
        let bind = data.st_info.clone() >> 4;
        let st_bind = SymBinding::from(bind);
        let kind = data.st_info.clone() & 0x0Fu8;
        let st_type = SymType::from(kind);
        let header = strtab.duplicate();
        let st_size = data.st_size.clone();
        let st_value = data.st_value.clone();
        let st_shndx = data.st_shndx.clone();
        Ok(SymEntry64 {
            header, name, st_bind, st_type, st_prot, st_size, st_value, st_shndx
        })
    }
}

fn get_name<'a, S>(strtab: &S, index: usize)
    -> Result<Option<Cow<'a,str>>,Fault> 
    where S: Section<'a>+'a+?Sized
{

    if index == 0 {
        return Ok(None);
    }
    let strtab = match strtab.get_data() {
        Option::Some(val) => val,
        Option::None => return Err(Fault::StrTabNotInElf),
    };
    if index < strtab.len() {
        Ok(find_null(&strtab[index..]))
    } else {
        Err(Fault::NameNotInStrTab)
    }
}

//Just parse without metadata
#[derive(Clone)]
#[repr(packed)]
struct PreSymEntry64 {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}
impl PreSymEntry64 {

    /// The actual parse logic
    #[inline(always)]
    fn build<'a, E: Endian+?Sized+'a>(temp: &[u8], header: &E)
        -> Result<PreSymEntry64,Fault>
    {
        named!(parse_pre_sym_ent_64_le<PreSymEntry64>,do_parse!(
            st_name: le_u32 >>
            st_info: le_u8 >>
            st_other: le_u8 >>
            st_shndx: le_u16 >>
            st_value: le_u64 >>
            st_size: le_u64 >>
            (PreSymEntry64 {
                st_name, st_value, st_size, st_info, st_other, st_shndx
            })
        ));
        named!(parse_pre_sym_ent_64_be<PreSymEntry64>,do_parse!(
            st_name: be_u32 >>
            st_info: be_u8 >>
            st_other: be_u8 >>
            st_shndx: be_u16 >>
            st_value: be_u64 >>
            st_size: be_u64 >>
            (PreSymEntry64 {
                st_name, st_value, st_size, st_info, st_other, st_shndx
            })
        ));
        let ent = match header.get_endian() {
            ElfEndian::Little => parse_pre_sym_ent_64_le(temp),
            ElfEndian::Big => parse_pre_sym_ent_64_be(temp),
        };
        match ent {
            IResult::Incomplete(_) => Err(Fault::TooSmol),
            IResult::Error(_) => Err(Fault::Complex),
            IResult::Done(_,s) => Ok(s)
        }
    }
}

/// Parse a 64bit symbol table
pub fn parse_symtable64<'a,S>(symtab: &S)
    -> Result<Arc<[Box<SymEntry<'a>+'a>]>,Fault>
    where S: Section<'a>+'a+?Sized
{
    if symtab.is_32bits() {
        return Err(Fault::Bits64ParserFailed);
    }
    let data = match symtab.get_data() {
        Option::Some(val) => val,
        Option::None => return Err(Fault::SymTabNotInElf),
    };
    let sym_entry_size = size_of::<PreSymEntry64>();
    let entries = if data.len() != 0 && data.len() >= sym_entry_size {
        let entry_num = data.len() / sym_entry_size;
        let mut entries: Vec<Box<SymEntry<'a>+'a>> = Vec::with_capacity(entry_num);
        for i in 0..entry_num {
            let offset = i * sym_entry_size;
            if offset < data.len() {
                let pre = PreSymEntry64::build(&data[offset..], symtab)?;
                let arg = SymEntry64::new(&pre,symtab)?;
                entries.push(Box::new(arg));
            } else {
                return Err(Fault::SymTabTooSmol);
            }
        }
        entries
    } else {
        Vec::with_capacity(0)
    };
    Ok(Arc::from(entries))
}

impl<'a> SymbolBinding for SymEntry64<'a> {
    #[inline(always)]
    fn get_sym_binding(&self) -> SymBinding {
        self.st_bind.clone()
    }
}
impl<'a> SymbolType for SymEntry64<'a> {
    #[inline(always)]
    fn get_sym_type(&self) -> SymType {
        self.st_type.clone()
    }
}
impl<'a> SymbolProtection for SymEntry64<'a> {
    #[inline(always)]
    fn get_sym_protection(&self) -> SymProt {
        self.st_prot.clone()
    }
}
impl<'a> Abi for SymEntry64<'a> {
    #[inline(always)] 
    fn get_abi(&self) -> ElfAbi {
        self.header.get_abi()
    }
}
impl<'a> Endian for SymEntry64<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.header.get_endian()
    }
}
impl<'a> FileType for SymEntry64<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.header.get_file_type()
    }
}
impl<'a> ElfMagicNumber for SymEntry64<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.header.get_abi_version()
    }
}
impl<'a> Arch for SymEntry64<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.header.get_arch()
    }
}
impl<'a> Class for SymEntry64<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.header.get_class()
    }
}
impl<'a> BufferBorrow<'a> for SymEntry64<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.header.get_buffer()
    }
}
impl<'a> ElfHeader<'a> for SymEntry64<'a> {
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
impl<'a> SymEntry<'a> for SymEntry64<'a> {

    #[inline(always)]
    fn get_name<'b>(&'b self) -> Option<&'b str> {
        match &self.name {
            &Option::None => None,
            &Option::Some(ref x) => Some(x.as_ref())
        }
    }

    #[inline(always)]
    fn st_value(&self) -> VarSize {
        VarSize::from(self.st_value.clone())
    }

    #[inline(always)]
    fn st_size(&self) -> VarSize {
        VarSize::from(self.st_size.clone())
    }

    #[inline(always)]
    fn st_shndx(&self) -> u16 {
        self.st_shndx.clone()
    }
}

