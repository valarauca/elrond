use super::super::nom::{
    le_u64, be_u64,
    le_i64, be_i64,
    IResult,
};
use super::super::{
    Fault,
    ElfClass, Class,
    ElfEndian, Endian
};


struct PreRel64 {
    r_offset: u64,
    r_info: u64
}
impl PreRel64 {
    fn new<E>(buffer: &[u8], elf: &E)
        -> Result<PreRel64,Fault>
        where E: Class+Endian+?Sized
    {
        named!(parser_pre_rel64_le<PreRel64>, do_parse!(
            r_offset: le_u64 >>
            r_info: le_u64 >>
            ((PreRel64{ r_offset, r_info }))
        ));
        named!(parser_pre_rel64_be<PreRel64>, do_parse!(
            r_offset: be_u64 >>
            r_info: be_u64 >>
            ((PreRel64{ r_offset, r_info }))
        ));
        if elf.is_64bits() {
            return Err(Fault::Bits64ParserFailed);
        }
        if buffer.len() >= 16 {
            let data = match elf.get_endian() {
                ElfEndian::Little => parser_pre_rel64_le(buffer),
                ElfEndian::Big => parser_pre_rel64_be(buffer)
            };
            match data {
                IResult::Done(_,x) => Ok(x),
                IResult::Error(_) => Err(Fault::Complex),
                IResult::Incomplete(_) => Err(Fault::TooSmol)
            }
        } else {
            Err(Fault::TooSmol)
        }
    }
}
struct PreRela64 {
    r_offset: u64,
    r_info: u64,
    r_addend: i64,
}
impl PreRela64 {
    fn new<E>(buffer: &[u8], elf: &E)
        -> Result<PreRela64,Fault>
        where E: Class+Endian+?Sized
    {
        named!(parse_pre_rela64_le<PreRela64>, do_parse!(
            r_offset: le_u64 >>
            r_info: le_u64 >>
            r_addend: le_i64 >>
            ((PreRela64{ r_offset, r_info, r_addend }))
        ));
        named!(parse_pre_rela64_be<PreRela64>, do_parse!(
            r_offset: be_u64 >>
            r_info: be_u64 >>
            r_addend: be_i64 >>
            ((PreRela64{ r_offset, r_info, r_addend }))
        ));
        if elf.is_64bits() {
            return Err(Fault::Bits64ParserFailed);
        }
        if buffer.len() >= 18 {
            let data = match elf.get_endian() {
                ElfEndian::Little => parse_pre_rela64_le(buffer),
                ElfEndian::Big => parse_pre_rela64_be(buffer)
            };
            match data {
                IResult::Done(_,x) => Ok(x),
                IResult::Error(_) => Err(Fault::Complex),
                IResult::Incomplete(_) => Err(Fault::TooSmol)
            }
        } else {
            Err(Fault::TooSmol)
        }
    }
}


