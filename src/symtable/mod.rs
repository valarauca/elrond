
mod bind;
pub use self::bind::{SymBinding,SymbolBinding};
mod symtype;
pub use self::symtype::{SymType,SymbolType};
mod symprot;
pub use self::symprot::{SymProt,SymbolProtection};
mod symtable32;
pub use self::symtable32::{parse_symtable32};
mod symtable64;
pub use self::symtable64::{parse_symtable64};
mod traits;
pub use self::traits::{SymEntry};
mod symtable;
pub use self::symtable::{parse_symtable};


use std::borrow::Cow;
pub fn find_null<'a>(buffer: &'a [u8]) -> Option<Cow<'a,str>> {
    let mut term = 0usize;
    let len = buffer.len();
    for i in 0..len {
        if buffer[i] == 0 {
            break;
        } else {
            term += 1;
            continue;
        }
    }
    let temp = &buffer[0..term];
    Some(String::from_utf8_lossy(temp))
}

