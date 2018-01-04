
use super::super::{ElfHeader, ElfClass, Fault, Class};

use super::section32::{parse_section32};
use super::section64::{parse_section64};
use super::traits::{Section};

use std::sync::Arc;
use std::ops::Index;

macro_rules! get_sect {
    ($func_name: ident, $name: expr) => {
        pub fn $func_name(&self) -> Option<Arc<Section<'a>+'a>> {
            self.get_section_by_name($name)
        }
    }
}

/// Holds all sections
#[derive(Clone)]
pub struct Sections<'a> {
    data: Arc<[Arc<Section<'a>+'a>]>
}

impl<'a> Sections<'a> {

    /// Build new sections
    pub fn new<E: ElfHeader<'a>+'a+?Sized>(elf: &E) -> Result<Sections<'a>,Fault> {
        let data = match elf.get_class() {
            ElfClass::Bits64 => parse_section64(elf)?,
            ElfClass::Bits32 => parse_section32(elf)?,
        };
        Ok(Sections { data } )
    }

    /// Get a section from its name
    ///
    /// In the event there are multiple sections with the same name, this
    /// will return the first
    #[inline]
    pub fn get_section_by_name(&self, name: &str) -> Option<Arc<Section<'a>+'a>> {
        self.data.iter()
            .filter(|x| x.name() == name)
            .map(|x| x.clone())
            .next()
    }

    /// Get the number of sections
    #[inline(always)]
    pub fn num(&self) -> usize {
        self.data.len()
    }

    get_sect!(get_bss, ".bss");
    get_sect!(get_comment, ".comment");
    get_sect!(get_data, ".data");
    get_sect!(get_data1, ".data1");
    get_sect!(get_debug, ".debug");
    get_sect!(get_dyanmic, ".dynamic");
    get_sect!(get_dynstr, ".dynstr");
    get_sect!(get_dynsym, ".dynsym");
    get_sect!(get_fini, ".fini");
    get_sect!(get_got, ".got");
    get_sect!(get_hash, ".hash");
    get_sect!(get_init, ".init");
    get_sect!(get_interp, ".interp");
    get_sect!(get_line, ".line");
    get_sect!(get_note, ".note");
    get_sect!(get_plt, ".plt");
    get_sect!(get_rodata, ".rodata");
    get_sect!(get_rodata1, ".rodata1");
    get_sect!(get_shstrtab, ".shstrtab");
    get_sect!(get_strtab, ".strtab");
    get_sect!(get_symtab, ".symtab");
    get_sect!(get_text, ".text");
    get_sect!(get_sdata, ".sdata");
    get_sect!(get_tdesc, ".tdesc");
    get_sect!(get_sbss, ".sbss");
    get_sect!(get_lit8, ".lit8");
    get_sect!(get_gptab, ".gptab");
    get_sect!(get_conflict, ".conflict");
    get_sect!(get_reginfo, ".reginfo");
    get_sect!(get_liblist, ".liblist");
}
/// Iterate over sections
pub struct SectionsIter<'a> {
    index: usize,
    data: Arc<[Arc<Section<'a>+'a>]>
}
impl<'a> Iterator for SectionsIter<'a> {
    type Item = Arc<Section<'a>+'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(self.data[index].clone())
    }
}
impl<'a> IntoIterator for Sections<'a> {
    type Item = Arc<Section<'a>+'a>;
    type IntoIter = SectionsIter<'a>;
    fn into_iter(self) -> SectionsIter<'a> {
        SectionsIter {
            index: 0,
            data: self.data.clone()
        }
    }
}


