

#[macro_export]
macro_rules! parse_test {
    (@LIST $func:ident, $($dut:expr=>$eq:expr),*) => {
        $(
            let dut = $dut;
            let (_,val) = $func(dut).unwrap();
            assert_eq!(val, $eq);
        )*
    };
    ($dut:expr, $func:ident, $eq: expr) => {
        let dut = $dut;
        let (_,val) = $func(dut).unwrap();
        assert_eq!(val, $eq);
    };
}

#[macro_export]
macro_rules! build_enum {
    (
        @ARRAY
        @ENUM $enum_name:ident{$($val:ident),*}
        @TOFUNC $func_name:ident => $ret: ty {$($func_val:expr=>$func_out:expr),*; $panic:expr }
        @PUB $name: ident; $take: expr
    ) => {
        #[allow(dead_code)]
        #[derive(Copy,Clone,Debug,PartialEq,Eq)]
        pub enum $enum_name {
            $(
                $val,
            )*
        }
        #[allow(dead_code)]
        #[inline(always)]
        fn $func_name( x: &[u8]) -> $ret {
            match x {
                $(
                    $func_val => $func_out,
                )*
                y => panic!($panic,y)
            }
        }
        named!(pub $name<$ret>, map!(take!($take), $func_name));
    };
    (
        @INT $symbol: ident; $code: ty => $rego_code: ty;
        @ENUM $enum_name:ident{$($val:ident),*}
        @TOFUNC $func_name:ident => $ret: ty {$($func_val:expr=>$func_out:expr),*; $panic:expr }
        @PUB $name: ident
    ) => {
        #[allow(dead_code)]
        #[derive(Copy,Clone,Debug,PartialEq,Eq)]
        pub enum $enum_name {
            $(
                $val,
            )*
        }
        #[allow(dead_code)]
        #[inline(always)]
        fn $func_name( i: $code) -> $ret {
            let x: $rego_code = i.into();
            match x {
                $(
                    $func_val => $func_out,
                )*
                y => panic!($panic,y)
            }
        }
        named!(pub $name<$ret>, map!($symbol, $func_name));
    };
}

#[macro_export]
macro_rules! std_val {
    (
        @PLATFORM
        @ID: $identifier: ident;
        @INTO_FROM: $($kind: ty),*;
        @READER: $reader:ident => $new_sym: ident;
    ) => {
        #[allow(dead_code)]
        #[derive(Debug,PartialEq,Eq,Clone,Copy)]
        pub enum $identifier {
            Bits32(u32),
            Bits64(u64)
        }
        $(
            impl Into<$kind> for $identifier {
                #[inline(always)]
                #[allow(dead_code)]
                fn into(self) -> $kind {
                    match self {
                        $identifier::Bits32(x) => x as $kind,
                        $identifier::Bits64(x) => x as $kind
                    }
                }
            }
            impl From<$kind> for $identifier {
                #[inline(always)]
                #[allow(dead_code)]
                fn from(x: $kind) -> Self {
                    match get_platform() {
                        Platform::Bit32 => $identifier::Bits32( x as u32),
                        Platform::Bit64 => $identifier::Bits64( x as u64)
                    }
                }
            }
        )*
        #[inline(always)]
        #[allow(dead_code)]
        pub fn $new_sym<'a>(i: &'a [u8]) -> IResult<&'a [u8], $identifier> {
            match $reader(i) {
                IResult::Done(x,y) => IResult::Done(x,y.into()),
                IResult::Error(z) => IResult::Error(z),
                IResult::Incomplete(z) => IResult::Incomplete(z)
            }
        }
    };
    (
        @ID: $identifier: ident; $internal: ty;
        @INTO_FROM: $($kind: ty),*;
        @READER: $reader:ident => $new_sym: ident;
    ) => {
        #[allow(dead_code)]
        #[derive(Debug,PartialEq,Eq,Clone,Copy)]
        pub struct $identifier ( pub $internal);
        $(
            impl Into<$kind> for $identifier {
                #[inline(always)]
                #[allow(dead_code)]
                fn into(self) -> $kind {
                    self.0 as $kind
                }
            }
            impl From<$kind> for $identifier {
                #[inline(always)]
                #[allow(dead_code)]
                fn from(x: $kind) -> Self {
                    $identifier ( x as $internal)
                }
            }
        )*
        #[inline(always)]
        #[allow(dead_code)]
        pub fn $new_sym<'a>(i: &'a [u8]) -> IResult<&'a [u8], $identifier> {
            match $reader(i) {
                IResult::Done(x,y) => IResult::Done(x,y.into()),
                IResult::Error(z) => IResult::Error(z),
                IResult::Incomplete(z) => IResult::Incomplete(z)
            }
        }
    };
}

#[macro_export]
macro_rules! generate_reader {
    (
        $name: ident; $internal: ident;$code: ty;
    ) => {
        ///read a u16 (in correct endianness)
        #[inline(always)]
        #[allow(dead_code)]
        fn $internal(x: &[u8]) -> $code {
            get_endian().$name(x)
        }
        named!($name<$code>, map!(take!(::std::mem::size_of::<$code>), $internal));
    }
}
