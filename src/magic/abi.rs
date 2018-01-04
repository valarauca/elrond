use super::super::nom::le_u8;

new_enum! {@byte_parser_with_unknown
    type_name: ElfAbi;
    new_trait: {
        trait_name: Abi;
        getter_method: get_abi;
    }
    parser_name: parse_elf_abi;
    values: {
        (is_systemv, SystemV, 0),
        (is_hpux, HPUX, 1),
        (is_netbsd, NetBSD, 2),
        (is_linux, Linux, 3),
        (is_solaris, Solaris, 6),
        (is_aix, AIX, 7),
        (is_irix, IRIX, 8),
        (is_freebsd, FreeBSD, 9),
        (is_openbsd, OpenBSD, 0xC),
        (is_openvms, OpenVMS, 0xD)
    }
}

