
#[allow(unused_imports)]
use super::super::value::{read_half,Elf_Half};
build_enum!{
    @INT
        read_half;
        Elf_Half => usize;
    @ENUM Kind {
        Relocatable,
        Executable,
        Shared,
        Core,
        Loproc,
        Hiproc
    }
    @TOFUNC to_endian => Kind {
        1 => Kind::Relocatable,
        2 => Kind::Executable,
        3 => Kind::Shared,
        4 => Kind::Core,
        0xFF00 => Kind::Loproc,
        0xFFFF => Kind::Hiproc
        ;
        "\n\nValue {:?} is non-standard for E_TYPE\n\n"
    }
    @PUB read_kind
}
