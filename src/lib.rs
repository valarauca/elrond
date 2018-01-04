#![allow(unused_imports,dead_code)]

#[macro_use]
extern crate nom;
use nom::{Err,ErrorKind};

#[macro_use]
mod macros;

mod magic;
pub use self::magic::{
    ElfAbi, Abi,
    ElfEndian, Endian,
    ElfClass, Class,
    ElfMagicNumber,
};

mod header;
pub use self::header::{
    ElfFileType, FileType,
    ElfArch, Arch,
    ElfHeaderBase, ElfHeader,
    parse_elf
};

mod varsize;
pub use self::varsize::VarSize;

mod phdr;
pub use self::phdr::{PHDR};

mod section;
pub use self::section::{
    SectionAttributes, SectMemAttrib,
    SectionKind,SectionType,
    Section,
    Sections, SectionsIter,
};

mod symtable;
pub use self::symtable::{
    SymBinding, SymbolBinding,
    SymType, SymbolType,
    SymProt, SymbolProtection, 
    SymEntry,
};
mod relocation;

pub mod prelude {
    pub use super::{
        Abi, Endian, Class,
        FileType, Arch, ElfHeader,
        SectionAttributes, SectMemAttrib, Section,
        SymbolBinding, SymbolType, SymbolProtection, SymEntry,
    };
}

/// Errors in the parser
#[derive(Debug,Clone,Copy)]
pub enum Fault {
    IllegalMagicNumber,         // 1
    IllegalEndianness,          // 2
    IllegalClass,               // 3
    /// Attempted to read 32bit data with 64bit parser
    Bits32ParserFailed,         // 4 wrong parser
    /// Underlying buffer couldn't hold data
    TooSmol,
    /// Internal parser error, generally missed test case
    Complex,
    HeaderDoesntExist,
    /// Attempted to read 32bit data with 64bit parser
    Bits64ParserFailed,
    StrTabNotInElf,
    NameNotInStrTab,
    SymTabNotInElf,
    SymTabTooSmol,
}


/// Helper function to fix nom's bullshit
#[inline(always)]
fn nom_bullshit<P>(x: Err<P,u32>) -> ErrorKind {
    match x {
        Err::Code(e) => e,
        Err::Node(e,_) => e,
        Err::Position(e,_) => e,
        Err::NodePosition(e,_,_) => e
    }
}

/// Returns the underlying buffer of the entire file
pub trait BufferBorrow<'a> {
    fn get_buffer(&self) -> &'a [u8];
}

