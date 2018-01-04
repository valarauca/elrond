
use super::super::{ElfHeader, VarSize, BufferBorrow};

use super::{
    SymBinding, SymbolBinding,
    SymType, SymbolType,
    SymProt, SymbolProtection,
    find_null,
};

pub trait SymEntry<'a>: SymbolBinding + SymbolType + SymbolProtection + ElfHeader<'a> {

    fn get_name<'b>(&'b self) -> Option<&'b str>;

    fn st_value(&self) -> VarSize;
    fn st_size(&self) -> VarSize;
    fn st_shndx(&self) -> u16;
}
