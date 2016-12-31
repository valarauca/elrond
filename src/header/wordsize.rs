build_enum!{
    @ARRAY
    @ENUM Platform {
        Bit32,
        Bit64
    }
    @TOFUNC to_platform => Platform {
        b"\x01" => Platform::Bit32,
        b"\x02" => Platform::Bit64
        ;
        "\n\nValue {:?} is non-standard for EI_CLASS\n\n"
    }
    @PUB
        read_platform;
        1
}
