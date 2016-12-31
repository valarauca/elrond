
use super::value::{
    Elf_Word,
    Elf_Addr,
    Elf_VarWord,
    Elf_Off
};

pub struct Note {
    pub namesize: Elf_Word,
    pub descxz: Elf_Word,
    pub kind: Elf_Word
}
