
use super::super::nom::{IResult, ErrorKind, le_u8};

use super::super::{Fault, nom_bullshit};

/*
 * LE/BE doesn't matter when reading bytes
 * individual bytes
 */
use super::abi::{ElfAbi, Abi, parse_elf_abi};
use super::endian::{ElfEndian, Endian, parse_elf_endian};
use super::class::{ElfClass, Class, parse_elf_class};
use super::traits::{ElfMagicNumber};

/// Data extracted from magic numbers
#[derive(Copy,Clone,Debug,Hash)]
pub struct ElfMagicNumbers {
    class: ElfClass,
    endian: ElfEndian,
    abi: ElfAbi,
    abi_version: u8,
}
impl Abi for ElfMagicNumbers {
    #[inline(always)]
    fn get_abi(&self) -> ElfAbi {
        self.abi.clone()
    }
}
impl Endian for ElfMagicNumbers {
    #[inline(always)]
    fn get_endian(&self) -> ElfEndian {
        self.endian.clone()
    }
}
impl Class for ElfMagicNumbers {
    #[inline(always)]
    fn get_class(&self) -> ElfClass {
        self.class.clone()
    }
}
impl ElfMagicNumber for ElfMagicNumbers {
    #[inline(always)]
    fn get_abi_version(&self) -> u8 {
        self.abi_version.clone()
    }
}

named!(check_magic<&[u8],&[u8],u32>,
    add_return_error!(ErrorKind::Custom(1u32),
    tag!(b"\x7F\x45\x4C\x46")
));

named!(pub parse_elf_magic<&[u8],ElfMagicNumbers,u32>, do_parse!(
    check_magic >>
    class: parse_elf_class >>
    endian: parse_elf_endian >> 
    take!(1) >>
    abi: parse_elf_abi >>
    abi_version: le_u8 >>
    take!(7) >>
    ( ElfMagicNumbers{
        class, endian, abi, abi_version
    })
));

#[test]
fn test_check_magic() {

    //passing test
    let dut0 = b"\x7F\x45\x4C\x46\xDE\xAD\xBE\xEF";
    let (rem,var) = match check_magic(dut0) {
        IResult::Done(x,y) => (x,y),
        IResult::Incomplete(n) => panic!("check magic failed {:?}",n),
        IResult::Error(e) => panic!("check magic failed {:?}",e)
    };
    assert_eq!(rem.len(), 4);
    assert_eq!(rem, b"\xDE\xAD\xBE\xEF");
    assert_eq!(var.len(), 4);
    assert_eq!(var, b"\x7F\x45\x4C\x46");
    
    //failing test
    let dut1 = b"\x00\x45\x4C\x46\xDE\xAD\xBE\xEF";
    match check_magic(dut1) {
        IResult::Done(_,_) => panic!("check magic should fail"),
        IResult::Incomplete(n) => panic!("check magic wrong error {:?}",n),
        IResult::Error(e) => {
            match nom_bullshit(e) {
                ErrorKind::Custom(1u32) => { }
                ErrorKind::Custom(x) => panic!("check magic wrong error code {:?}", x),
                e => panic!("check magic wrong error code {:?}", e)
            }
        }
    };
}

#[test]
fn test_parse_elf_magic() {

    //passing test
    let dut0: &[u8] = b"\x7F\x45\x4C\x46\x02\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let (rem, magic) = match parse_elf_magic(dut0) {
        IResult::Done(x,m) => (x,m),
        IResult::Incomplete(n) => panic!("check_version failed {:?}",n),
        IResult::Error(e) => panic!("check_version failed {:?}",e)
    };
    //check remainer
    assert_eq!(rem.len(), 1);
    assert_eq!(rem[0],0);

    //check values
    assert_eq!(magic.get_abi(), ElfAbi::SystemV);
    assert_eq!(magic.get_endian(), ElfEndian::Little);
    assert_eq!(magic.get_class(), ElfClass::Bits64);
    assert_eq!(magic.get_abi_version(), 0);
}

