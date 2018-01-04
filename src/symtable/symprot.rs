use super::super::nom::{le_u8};

new_enum!{@byte_parser_with_unknown
    type_name: SymProt;
    new_trait: {
        trait_name: SymbolProtection;
        getter_method: get_sym_protection;
    }
    parser_name: parse_binding;
    values: {
        (is_default, Default, 0),
        (is_internal, Internal, 1),
        (is_hidden, Hidden, 2),
        (is_protected, Protected, 3)
    }
}

