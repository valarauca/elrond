pub mod endian;
pub use self::endian::{read_endian,Endian};
pub mod wordsize;
pub use self::wordsize::{read_platform,Platform};
pub mod abi;
pub use self::abi::{read_abi,ABI};
pub mod kind;
pub use self::kind::{read_kind, Kind};
pub mod arch;
pub use self::arch::{read_arch,Arch};


use super::{
    Elf_Addr,
    Elf_Off,
    Elf_Word,
    Elf_Half,
};
use super::value::{
    define_reader,
    read_word,
    read_addr,
    read_off,
    read_half
};
use super::nom::IResult;


///Reads the header of an ELF file
#[derive(Debug,Clone,PartialEq,Eq)]
struct Header {
    pub endian: Endian,
    pub wordsize: Platform,
    pub abi: ABI
}

named!(read_header<Header>, chain!(
    tag!("\x7F\x45\x4C\x46")~
    e: read_endian~
    w: read_platform~
    a: read_abi,
    || Header {
        endian: e,
        wordsize: w,
        abi: a
    }
));
impl Header {
    ///Configures the word size and endianness of the file
    pub fn define_platfrom(&self) {
        define_reader(&self.endian,&self.wordsize);
    }
}

///Full Elf Header
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct ElfHeader {
    pub endian: Endian,
    pub wordsize: Platform,
    pub abi: ABI,
    pub kind: Kind,
    pub arch: Arch,
    //version
    pub entry: Elf_Addr,
    pub phoff: Elf_Off,
    pub shoff: Elf_Off,
    pub flags: Elf_Word,
    pub ehsize: Elf_Half,
    pub phentsize: Elf_Half,
    pub phnum: Elf_Half,
    pub shentsize: Elf_Half,
    pub shnum: Elf_Half,
    pub shstrndx: Elf_Half
}
named!( read_full_elf_header<ElfHeader>, chain!(
    kind: read_kind~
    arch: read_arch~
    read_word~
    entr: read_addr~
    phof: read_off~
    shof: read_off~
    flag: read_word~
    ehsi: read_half~
    phen: read_half~
    phnu: read_half~
    shen: read_half~
    shun: read_half~
    shst: read_half,
    || ElfHeader{
        endian: Endian::Little,
        wordsize: Platform::Bit32,
        abi: ABI::SystemV,
        kind: kind,
        arch: arch,
        entry: entr,
        phoff: phof,
        shoff: shof,
        flags: flag,
        ehsize: ehsi,
        phentsize: phen,
        phnum: phnu,
        shentsize: shen,
        shnum: shun,
        shstrndx: shst
    }
));

macro_rules! try_parse {
    ($data:expr) => {
        match $data {
            IResult::Done(x,y) => (x,y),
            IResult::Incomplete(z) => return IResult::Incomplete(z),
            IResult::Error(z) => return IResult::Error(z)
        };
    }
}

///Reads an Elf Header. Configures word size and endianness as it does so
#[allow(dead_code)]
#[inline(always)]
pub fn read_elf_header<'a>(i: &'a[u8]) -> IResult<&'a[u8], ElfHeader> {
    let (_,header) = try_parse!(read_header(i));
    //set thread locals
    header.define_platfrom();
    //read platform dependent info
    let (rem,mut full_header) = try_parse!(read_full_elf_header(&i[16..]));
    //set information that was already parsed
    full_header.endian = header.endian.clone();
    full_header.wordsize = header.wordsize.clone();
    full_header.abi = header.abi.clone();
    //return success!
    IResult::Done(rem,full_header)
}
