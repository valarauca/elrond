use super::super::nom::{le_u16,be_u16};

new_enum! {@var_with_unknown
    type_name: ElfArch;
    inner_type: u16;
    new_trait: {
        trait_name: Arch;
        getter_method: get_arch;
    };
    parser: {
        name: {
            big_endian: parse_elf_arch_be;
            little_endian: parse_elf_arch_le;
        };
        nom: {
            big_endian: be_u16;
            little_endian: le_u16;
        };
    };
    values: {
        (is_none, None, 0),
        (is_intel386, Intel386, 3),
        (is_motorola68k, Motorola68k, 4),
        (is_motorola88k, Motorola88k, 5),
        (is_intel486, Intel486, 6),
        (is_intel860, Intel860, 7),
        (is_powerpc, PowerPC, 20),
        (is_powerpc64, PowerPC64, 21),
        (is_arm, ARM, 40),
        (is_itanium, IA64, 50),
        (is_amd64, AMD64, 62),
        (is_aarch64, Aarch64, 183),
        (is_linux_bpf, LinuxBPF, 247),
        (is_open_risc, OpenRISC, 92)
    };
}

