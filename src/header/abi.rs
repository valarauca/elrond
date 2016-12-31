
build_enum!{
    @ARRAY
    @ENUM ABI {
        SystemV,
        HPUX,
        NetBSD,
        Linux,
        Solaris,
        AIX,
        IRIX,
        FreeBSD,
        OpenBSD,
        OpenVMS,
        NonStopKernel,
        AROS,
        FenixOS,
        CloudABI,
        Sortix,
        StandAlone
    }
    @TOFUNC to_abi => ABI {
        b"\x00" => ABI::SystemV,
        b"\x01" => ABI::HPUX,
        b"\x02" => ABI::NetBSD,
        b"\x03" => ABI::Linux,
        b"\x06" => ABI::Solaris,
        b"\x07" => ABI::AIX,
        b"\x08" => ABI::IRIX,
        b"\x09" => ABI::FreeBSD,
        b"\x0C" => ABI::OpenBSD,
        b"\x0D" => ABI::OpenVMS,
        b"\x0E" => ABI::NonStopKernel,
        b"\x0F" => ABI::AROS,
        b"\x10" => ABI::FenixOS,
        b"\x11" => ABI::CloudABI,
        b"\x53" => ABI::Sortix,
        b"\xff" => ABI::StandAlone
        ;
        "\n\nValue {:?} is non-standard\n\n"
    }
    @PUB
        read_abi;
        1
}
