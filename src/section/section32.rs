
use super::super::nom::{le_u32,be_u32,IResult};
use super::super::{
    Abi, ElfAbi, Endian, ElfEndian, Class, ElfClass, ElfMagicNumber,
    Arch, ElfArch, FileType, ElfFileType, ElfHeader, BufferBorrow,
    VarSize, Fault, ElfHeaderBase,
};
use super::section_type::{
    SectionKind,SectionType,
    parse_elf_section_type_be, parse_elf_section_type_le
};
use super::attributes::{SectionAttributes, SectMemAttrib, build_attributes};
use super::traits::Section;
use super::find_null;

use std::ffi::CStr;
use std::sync::Arc;
use std::borrow::Cow;

pub struct Section32<'a> {
    header: ElfHeaderBase<'a>,
    name: Cow<'a,str>,
    link: Option<Cow<'a,str>>,
    index: usize,
    sh_flags: Box<[SectMemAttrib]>,
    sections: Arc<[PreSection32]>
}
impl<'a> Section32<'a> {

    ///Build a new section
    fn new<E: ElfHeader<'a>+'a+?Sized>(data: &Arc<[PreSection32]>, index: usize, header: &E)
        -> Result<Section32<'a>,Fault>
    {
        let name_index = header.e_shstrndx();
        let own_name = data[index].sh_name.clone() as usize;
        let name = match get_name(data, header, name_index, own_name)? {
            Option::Some(var) => var,
            Option::None => return Err(Fault::NameNotInStrTab)
        };
        let link_name = data[index].sh_name.clone() as usize;
        let link = get_name(data, header, name_index, link_name)?;
        let flags = build_attributes(data[index].sh_flags.clone() as u32);
        Ok(Section32 {
            header: header.duplicate(),
            name: name,
            link: link,
            index: index,
            sh_flags: flags, 
            sections: data.clone(), 
        })
    }
}

/// Get the name of a section
fn get_name<'a, E: ElfHeader<'a>+'a+?Sized>(data: &Arc<[PreSection32]>, header: &E, name_index: usize, index: usize)
    -> Result<Option<Cow<'a,str>>,Fault>
{
    if index == 0 {
        return Ok(None);
    }
    let strtab = if name_index < data.len() {
        match data[name_index].get_data(header) {
            Option::Some(val) => val,
            Option::None => return Err(Fault::StrTabNotInElf),
        }
    } else {
        return Err(Fault::StrTabNotInElf);
    };
    if index < strtab.len() {
        Ok(find_null(&strtab[index..]))
    } else {
        Err(Fault::NameNotInStrTab)
    }
}

//inter just parsed w/o metadata
#[derive(Clone)]
struct PreSection32 {
    sh_name: u32,
    sh_type: SectionKind,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32
}
impl PreSection32 {

    /// Get buffer from file
    fn get_data<'a, E: ElfHeader<'a>+'a+?Sized>(&self, header: &E) -> Option<&'a [u8]> {
        let offset = self.sh_offset.clone() as usize;
        let size = self.sh_size.clone() as usize;
        if size == 0 {
            return None;
        }
        let end = offset + size;
        let buffer = header.get_buffer();
        if offset < buffer.len() && end < buffer.len() {
            Some(&buffer[offset..end])
        } else {
            None
        }
    }

    /// The actual parse logic
    #[inline(always)]
    pub fn build<'a, E: ElfHeader<'a>+?Sized+'a>(temp: &[u8], header: &E)
        -> Result<PreSection32,Fault> 
    {
        //declare internal parsers
        named!(parse_pre_sect_64_le<PreSection32>,do_parse!(
            sh_name: le_u32 >>
            sh_type: parse_elf_section_type_le >>
            sh_flags: le_u32 >>
            sh_addr: le_u32 >>
            sh_offset: le_u32 >>
            sh_size: le_u32 >>
            sh_link: le_u32 >>
            sh_info: le_u32 >>
            sh_addralign: le_u32 >>
            sh_entsize: le_u32 >>
            (PreSection32 {
                sh_name, sh_type, sh_flags, sh_addr, sh_offset,
                sh_size, sh_link, sh_info, sh_addralign, sh_entsize
            })
        ));
        named!(parse_pre_sect_64_be<PreSection32>,do_parse!(
            sh_name: be_u32 >>
            sh_type: parse_elf_section_type_be >>
            sh_flags: be_u32 >>
            sh_addr: be_u32 >>
            sh_offset: be_u32 >>
            sh_size: be_u32 >>
            sh_link: be_u32 >>
            sh_info: be_u32 >>
            sh_addralign: be_u32 >>
            sh_entsize: be_u32 >>
            (PreSection32 {
                sh_name, sh_type, sh_flags, sh_addr, sh_offset,
                sh_size, sh_link, sh_info, sh_addralign, sh_entsize
            })
        ));
        let sect = match header.get_endian() {
            ElfEndian::Little => parse_pre_sect_64_le(temp),
            ElfEndian::Big => parse_pre_sect_64_be(temp)
        };
        match sect {
            IResult::Incomplete(_) => Err(Fault::TooSmol),
            IResult::Error(_) => Err(Fault::Complex),
            IResult::Done(_,s) => Ok(s)
        }
    }
}
pub fn parse_section32<'a, E: ElfHeader<'a>+'a+?Sized>(header: &E)
    -> Result<Arc<[Arc<Section<'a>+'a>]>,Fault>
{
    // ensure we aren't in 32bit mode
    if header.is_64bits() {
        return Err(Fault::Bits64ParserFailed);
    }
    // check for sections
    let size = header.e_shentsize();
    let num = header.e_shnum();
    if size == 0 || num == 0 {
        return Ok(Arc::from(Vec::new()));
    }
    let offset = header.e_shoff();
    // collect sections
    let mut ret_vec = Vec::with_capacity(num);
    for index in 0..num {
        let start = offset + size * index;
        let end = start + size;
        let buffer = header.get_buffer();
        if start < buffer.len() && end < buffer.len() {
            ret_vec.push(PreSection32::build(&buffer[start..end],header)?);
        } else {
            return Err(Fault::TooSmol);
        }
    }
    //build arc slice
    let arc_slice: Arc<[PreSection32]> = Arc::from(ret_vec);
    let mut sections: Vec<Arc<Section<'a>+'a>> = Vec::with_capacity(num);
    for index in 0..num {
        sections.push(Arc::new(Section32::new(&arc_slice,index,header)?));
    }
    let arc: Arc<[Arc<Section<'a>+'a>]> = Arc::from(sections);
    Ok(arc)
}

impl<'a> SectionAttributes for Section32<'a> {
    fn get_attributes<'b>(&'b self) -> &'b [SectMemAttrib] {
        &self.sh_flags
    }
}
impl<'a> SectionType for Section32<'a> {
    fn get_section_type(&self) -> SectionKind {
        self.sections[self.index].sh_type.clone()
    }
}
impl<'a> Abi for Section32<'a> {
    #[inline(always)] 
    fn get_abi(&self) -> ElfAbi {
        self.header.get_abi()
    }
}
impl<'a> Endian for Section32<'a> {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.header.get_endian()
    }
}
impl<'a> FileType for Section32<'a> {
    #[inline(always)]
    fn get_file_type(&self) -> ElfFileType {
        self.header.get_file_type()
    }
}
impl<'a> ElfMagicNumber for Section32<'a> {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.header.get_abi_version()
    }
}
impl<'a> Arch for Section32<'a> {
    #[inline(always)]
    fn get_arch(&self) -> ElfArch {
        self.header.get_arch()
    }
}
impl<'a> Class for Section32<'a> {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.header.get_class()
    }
}
impl<'a> BufferBorrow<'a> for Section32<'a> {
    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.header.get_buffer()
    }
}
impl<'a> ElfHeader<'a> for Section32<'a> {
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
impl<'a> Section<'a> for Section32<'a> {
    #[inline(always)]
    fn sh_name(&self) -> usize {
        self.sections[self.index].sh_name.clone() as usize
    }
    #[inline(always)]
    fn name<'b>(&'b self) -> &'b str {
        self.name.as_ref()
    }
    #[inline(always)]
    fn link_name<'b>(&'b self) -> Option<&'b str> {
        match &self.link {
            &Option::None => None,
            &Option::Some(ref name) => Some(name)
        }
    }
    #[inline(always)]
    fn sh_addr(&self) -> VarSize {
        VarSize::from(self.sections[self.index].sh_addr.clone())
    }
    #[inline(always)]
    fn sh_offset(&self) -> usize {
        self.sections[self.index].sh_offset.clone() as usize
    }
    #[inline(always)]
    fn sh_size(&self) -> usize {
        self.sections[self.index].sh_size.clone() as usize
    }
    #[inline(always)]
    fn sh_link(&self) -> usize {
        self.sections[self.index].sh_link.clone() as usize
    }
    #[inline(always)]
    fn sh_info(&self) -> usize {
        self.sections[self.index].sh_info.clone() as usize
    }
    #[inline(always)]
    fn sh_addralign(&self) -> VarSize {
        VarSize::from(self.sections[self.index].sh_addralign.clone())
    }
    #[inline(always)]
    fn sh_entsize(&self) -> usize {
        self.sections[self.index].sh_entsize.clone() as usize
    }
}

