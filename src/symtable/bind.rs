
use super::super::nom::{le_u8};

new_enum!{@byte_parser_with_unknown
    type_name: SymBinding;
    new_trait: {
        trait_name: SymbolBinding;
        getter_method: get_sym_binding;
    }
    parser_name: parse_binding;
    values: {
        (is_local, Local, 0),
        (is_global, Global, 1),
        (is_weak, Weak, 2),
        (is_lo_proc, LoProc, 13),
        (is_hi_proc, HiProc, 15)
    }
}
