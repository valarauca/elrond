build_enum!{
    @ARRAY
    @ENUM Endian {
        Little,
        Big
    }
    @TOFUNC to_endian => Endian {
        b"\x01" => Endian::Little,
        b"\x02" => Endian::Big
        ;
        "\n\nValue {:?} is non-standard for EI_DATA\n\n"
    }
    @PUB
        read_endian;
        1
}

macro_rules! define_func {
    ($name: ident, $kind:ty) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub fn $name<'a>(&self,x: &'a [u8]) -> $kind {
            #[allow(unused_imports)]
            use super::super::byteorder::{ReadBytesExt,BigEndian,LittleEndian};
            use std::io::Cursor;
            let mut rdr = Cursor::new(x);
            match *self {
                Endian::Little => match rdr.$name::<LittleEndian>() {
                    Ok(x) => x,
                    Err(_) => unreachable!()
                },
                Endian::Big => match rdr.$name::<BigEndian>() {
                    Ok(x) => x,
                    Err(_) => unreachable!()
                },
            }
        }
    }
}

impl Endian {
    define_func!(read_u32, u32);
    define_func!(read_u16, u16);
    define_func!(read_u64, u64);
    define_func!(read_i32, i32);
    define_func!(read_i16, i16);
    define_func!(read_i64, i64);
}
