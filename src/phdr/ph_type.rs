use super::super::nom::{le_u32,be_u32};

new_enum! {@var_with_unknown
    type_name: ProgramHeaderType;
    inner_type: u32;
    new_trait: {
        trait_name: HeaderType;
        getter_method: get_ptype;
    };
    parser: {
        name: {
            big_endian: parse_elf_ph_type_be;
            little_endian: parse_elf_ph_type_le;
        };
        nom: {
            big_endian: be_u32;
            little_endian: le_u32;
        };
    };
    values: {
        (is_none, None, 0),
        (is_load, Load, 1),
        (is_dynamic, Dynamic, 2),
        (is_interp, Interp, 3),
        (is_note, Note, 4),
        (is_program_header_table, PHDR, 6),
        (is_lo_os, LoOS, 0x60000000),
        (is_hi_os, HiOS, 0x6FFFFFFF),
        (is_low_proc, LoProc, 0x70000000),
        (is_hi_proc, HiProc, 0x7FFFFFFF)
    };
}

