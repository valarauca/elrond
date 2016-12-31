
use super::value::{
    Elf_Word,
    Elf_Addr,
    Elf_VarWord,
    Elf_Off
};


pub struct Section {
    pub sh_name: Elf_Word,
    pub sh_type: Elf_Word,
    pub sh_flags: Elf_VarWord,
    pub sh_addr: Elf_Addr,
    pub sh_offset: Elf_Off,
    pub sh_size: Elf_VarWord,
    pub sh_link: Elf_Word,
    pub sh_info: Elf_Word,
    pub sh_addralign: Elf_VarWord,
    pub sh_entsize: Elf_VarWord
}
