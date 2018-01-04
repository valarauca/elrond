
use super::super::nom::{ErrorKind, Needed, IResult,Err};

new_enum! {@limited_byte_parser
    type_name: ElfClass;
    new_trait {
        trait_name: Class;
        getter_method: get_class;
    };
    limited_parser {
        error_code: 3u32;
        parser_name: parse_elf_class;
    };
    values {
        (is_32bits, Bits32, 1),
        (is_64bits, Bits64, 2)
    };
}
