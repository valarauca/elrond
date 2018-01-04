use super::super::nom::{le_u32,be_u32};

new_enum! {@var_with_unknown
    type_name: SectionKind;
    inner_type: u32;
    new_trait: {
        trait_name: SectionType;
        getter_method: get_section_type;
    };
    parser: {
        name: {
            big_endian: parse_elf_section_type_be;
            little_endian: parse_elf_section_type_le;
        };
        nom: {
            big_endian: be_u32;
            little_endian: le_u32;
        };
    };
    values: {
        (is_null, Null, 0),
        (is_progbits, ProgBits, 1),
        (is_sym_tab, SymTab, 2),
        (is_str_tab, StrTab, 3),
        (is_rela, Rela, 4),
        (is_hash, Hash, 5),
        (is_dynamic, Dynamic, 6),
        (is_note, Note, 7),
        (is_no_bits, NoBits, 8),
        (is_rel, Rel, 9),
        (is_shlib, Shilib, 10),
        (is_dyn_sym, DynSym, 11),
        (is_lo_os, LoOS, 0x60000000),
        (is_hi_os, HiOS, 0x6FFFFFFF),
        (is_low_proc, LoProc, 0x70000000),
        (is_hi_proc, HiProc, 0x7FFFFFFF)
    };
}

