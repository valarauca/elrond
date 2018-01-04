use super::super::nom::{le_u16,be_u16};

new_enum! {@var_with_unknown
    type_name: ElfFileType;
    inner_type: u16;
    new_trait: {
        trait_name: FileType;
        getter_method: get_file_type;
    };
    parser: {
        name: {
            big_endian: parse_elf_file_type_be;
            little_endian: parse_elf_file_type_le;
        };
        nom: {
            big_endian: be_u16;
            little_endian: le_u16;
        };
    };
    values: {
        (is_none, None, 0),
        (is_relocatble, Relocatable, 1),
        (is_executable, Executable, 2),
        (is_shared, Shared, 3),
        (is_core, Core, 4),
        (is_lo_os, LoOS, 0xFE00),
        (is_hi_os, HiOS, 0xFEFF),
        (is_low_proc, LoProc, 0xFF00),
        (is_hi_proc, HiProc, 0xFFFF)
    };
}
    
