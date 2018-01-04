
use super::super::nom::{
    le_u32, be_u32,
    le_i32, be_i32,
    IResult,
};
use super::super::{
    Fault,
    ElfClass, Class,
    ElfEndian, Endian
};


struct PreRel32 {
    r_offset: u32,
    r_info: u32
}
impl PreRel32 {
    fn new<E>(buffer: &[u8], elf: &E)
        -> Result<PreRel32,Fault>
        where E: Class+Endian+?Sized
    {
        named!(parser_pre_rel32_le<PreRel32>, do_parse!(
            r_offset: le_u32 >>
            r_info: le_u32 >>
            ((PreRel32{ r_offset, r_info }))
        ));
        named!(parser_pre_rel32_be<PreRel32>, do_parse!(
            r_offset: be_u32 >>
            r_info: be_u32 >>
            ((PreRel32{ r_offset, r_info }))
        ));
        if elf.is_64bits() {
            return Err(Fault::Bits32ParserFailed);
        }
        if buffer.len() >= 8 {
            let data = match elf.get_endian() {
                ElfEndian::Little => parser_pre_rel32_le(buffer),
                ElfEndian::Big => parser_pre_rel32_be(buffer)
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
struct PreRela32 {
    r_offset: u32,
    r_info: u32,
    r_addend: i32,
}
impl PreRela32 {
    fn new<E>(buffer: &[u8], elf: &E)
        -> Result<PreRela32,Fault>
        where E: Class+Endian+?Sized
    {
        named!(parse_pre_rela32_le<PreRela32>, do_parse!(
            r_offset: le_u32 >>
            r_info: le_u32 >>
            r_addend: le_i32 >>
            ((PreRela32{ r_offset, r_info, r_addend }))
        ));
        named!(parse_pre_rela32_be<PreRela32>, do_parse!(
            r_offset: be_u32 >>
            r_info: be_u32 >>
            r_addend: be_i32 >>
            ((PreRela32{ r_offset, r_info, r_addend }))
        ));
        if elf.is_64bits() {
            return Err(Fault::Bits32ParserFailed);
        }
        if buffer.len() >= 12 {
            let data = match elf.get_endian() {
                ElfEndian::Little => parse_pre_rela32_le(buffer),
                ElfEndian::Big => parse_pre_rela32_be(buffer)
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

