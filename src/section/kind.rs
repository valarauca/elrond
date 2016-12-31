
#[allow(unused_imports)]
use super::super::value::{read_word,Elf_Word};
build_enum!{
    @INT
        read_word;
        Elf_Word => usize;
    @ENUM SectionType {
        Null,
        Progbits,
        Symtab,
        Strtab,
        Rela,
        Hash,
        Dynamic,
        Note,
        NoBits,
        Rel,
        Shlib,
        DynSyn,
        Num,
        LoProc,
        HiProc,
        LoUser,
        HiUser
    }
    @TOFUNC to_kind => SectionType {
        0 => SectionType::Null,
        1 => SectionType::Progbits,
        2 => SectionType::Symtab,
        3 => SectionType::Strtab,
        4 => SectionType::Rela,
        5 => SectionType::Hash,
        6 => SectionType::Dynamic,
        7 => SectionType::Note,
        8 => SectionType::NoBits,
        9 => SectionType::Rel,
        10 => SectionType::Shlib,
        11 => SectionType::DynSyn,
        12 => SectionType::Num,
        0x70000000 => SectionType::LoProc,
        0x7fffffff => SectionType::HiProc,
        0x80000000 => SectionType::LoUser,
        0xffffffff => SectionType::HiUser
        ;
        "\n\nValue {:?} is non-standard for SH_TYPE\n\n"
    }
    @PUB read_sec_kind
}
