
use super::super::{VarSize, Abi, Endian, FileType, Arch, Class, BufferBorrow, ElfHeader};
use super::flags::{MemoryAttributes};
use super::ph_type::{HeaderType};

/// Program Descriptor Header
pub trait PHDR<'a>: HeaderType + MemoryAttributes + Abi + Endian + FileType + Arch + Class + BufferBorrow<'a> + ElfHeader<'a> {
    fn p_offset(&self) -> usize; 
    fn p_vaddr(&self) -> VarSize;
    fn p_paddr(&self) -> VarSize;
    fn p_filesz(&self) -> usize;
    fn p_memsz(&self) -> VarSize;
    fn p_align(&self) -> VarSize;
}
