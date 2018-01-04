
use super::super::nom::{le_u8};

new_enum!{@byte_parser_with_unknown
    type_name: SymType;
    new_trait: {
        trait_name: SymbolType;
        getter_method: get_sym_type;
    }
    parser_name: parse_sym_type;
    values: {
        (is_no_type, NoType, 0),
        (is_object, Object, 1),
        (is_func, Func, 2),
        (is_section, Section, 3),
        (is_file, File, 4),
        (is_lo_proc, LoProc, 13),
        (is_hi_proc, HiProc, 15)
    }
}

