
use super::super::{Section, ElfClass, Fault};

use super::{
    SymEntry,
    parse_symtable32,
    parse_symtable64,
};

use std::sync::Arc;

pub fn parse_symtable<'a,S>(symtab: &S)
    -> Result<Arc<[Box<SymEntry<'a>+'a>]>,Fault>
    where S: Section<'a>+'a+?Sized
{
    match symtab.get_class() {
        ElfClass::Bits64 => parse_symtable64(symtab),
        ElfClass::Bits32 => parse_symtable32(symtab)
    }
}
