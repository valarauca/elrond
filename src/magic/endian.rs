
use super::super::nom::{ErrorKind, Needed, IResult,Err};

new_enum! {@limited_byte_parser
    type_name: ElfEndian;
    new_trait {
        trait_name: Endian;
        getter_method: get_endian; 
    };
    limited_parser {
        error_code: 2u32;
        parser_name: parse_elf_endian;
    };
    values {
        (is_little, Little, 1),
        (is_big, Big, 2)
    };
}

