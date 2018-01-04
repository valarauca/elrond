
/*
 * Declare enums
 *
 * This abstracts away the boiler plate of enum
 * and trait declaration.
 *
 */
macro_rules! new_enum {

    /*
     * Limited Byte Parser
     *
     * Only parse 1 byte (so endian doesn't matter)
     *
     * If value is not in the set it'll return an error code.
     *
     * Requires `nom::{IResult,ErrorKind,Needed};`
     *
     */
    (@limited_byte_parser
        type_name: $enum_type_name: ident;
        new_trait {
            trait_name: $enum_trait_name: ident;
            getter_method: $enum_getter_name: ident;
        };
        limited_parser {
            error_code: $err: expr;
            parser_name: $parser_name: ident;
        };
        values {
            $(($isa_name: ident, $variant: ident, $val: expr)),*
        };
    ) => {
        #[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
        pub enum $enum_type_name {
            $( $variant ),*
        }
        pub trait $enum_trait_name {
            fn $enum_getter_name(&self) -> $enum_type_name;
            $(
                fn $isa_name(&self) -> bool {
                    match self.$enum_getter_name() {
                        $enum_type_name::$variant => true,
                        _ => false
                    }
                }
            )*
        }
        impl $enum_trait_name for $enum_type_name {
            #[inline(always)]
            fn $enum_getter_name(&self) -> $enum_type_name {
                self.clone()
            }
        }
        pub fn $parser_name<'a>(buffer: &'a [u8]) -> IResult<&'a [u8], $enum_type_name, u32> {
            let mut ret: Option<$enum_type_name> = None;
            let len = buffer.len();
            if len >= 1 {
                let var: usize = buffer[0].clone() as usize;
                $(
                    if var == ($val as usize) {
                        ::std::mem::replace(&mut ret, Some($enum_type_name::$variant));
                    }
                )*
            }
            match (ret,len) {
                (_,0) => IResult::Incomplete(Needed::Unknown),
                (Option::None,_) => IResult::Error(Err::Code(ErrorKind::Custom($err as u32))),
                (Option::Some(x),1) => IResult::Done(&buffer[0..0],x),
                (Option::Some(x),_) => IResult::Done(&buffer[1..],x)
            }
        }
    };

    /*
     * Byte Parser w/ Unknown
     *
     * There are some known fields, but the standard allows for alternative
     * values.
     *
     * Value should just be a numeric.
     *
     * Endianness doesn't matter
     *
     * This requires importing `nom::le_u8`
     */
    (@byte_parser_with_unknown
        type_name: $enum_type_name: ident;
        new_trait: {
            trait_name: $enum_trait_name: ident;
            getter_method: $enum_getter_name: ident;
        }
        parser_name: $parser_name: ident;
        values: {
            $(($isa_name: ident, $variant: ident, $val: expr)),*
        }
    ) => {
        #[derive(Copy,Clone,PartialEq,Eq,Hash)]
        pub enum $enum_type_name {
            $( $variant ),* ,Unknown(u8)
        }
        impl ::std::fmt::Debug for $enum_type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $( &$enum_type_name::$variant => write!(f, "{}", stringify!( $variant))),*
                    ,&$enum_type_name::Unknown(ref x) => write!(f, "Unknown({:#02X}", x),
                }
            }
        }
        pub trait $enum_trait_name {
            fn $enum_getter_name(&self) -> $enum_type_name;
            $(
                fn $isa_name(&self) -> bool {
                    match self.$enum_getter_name() {
                        $enum_type_name::$variant => true,
                        _ => false
                    }
                }
            )*
            fn is_unknown(&self) -> bool {
                match self.$enum_getter_name() {
                    $enum_type_name::Unknown(_) => true,
                    _ => false
                }
            }
            fn get_unknown(&self) -> Option<u8> {
                match self.$enum_getter_name() {
                    $enum_type_name::Unknown(x) => Some(x),
                    _ => None
                }
            }
        }
        impl From<u8> for $enum_type_name {
            #[inline(always)]
            fn from(x: u8) -> $enum_type_name {
                $(
                    if x == $val {
                        return $enum_type_name::$variant;
                    }
                )*
                $enum_type_name::Unknown(x)
            }
        }
        impl $enum_trait_name for $enum_type_name {
            #[inline(always)]
            fn $enum_getter_name(&self) -> $enum_type_name {
                self.clone()
            }
        }
        named!(pub $parser_name<&[u8],$enum_type_name,u32>, do_parse!(
            var: le_u8 >>
            ({ $enum_type_name::from(var) })
        ));

    };

    /*
     * Multi-byte parser
     *
     */
    (@var_with_unknown
        type_name: $enum_type_name: ident;
        inner_type: $kind: ty;
        new_trait: {
            trait_name: $enum_trait_name: ident;
            getter_method: $enum_getter_name: ident;
        };
        parser: {
            name: {
                big_endian: $parser_name_be: ident;
                little_endian: $parser_name_le: ident;
            };
            nom: {
                big_endian: $nom_be: ident;
                little_endian: $nom_le: ident;
            };
        };
        values: {
            $(($isa_name: ident, $variant: ident, $val: expr)),*
        };
    ) => {
        #[derive(Copy,Clone,PartialEq,Eq,Hash)]
        pub enum $enum_type_name {
            $( $variant ),* ,Unknown($kind)
        }
        impl ::std::fmt::Debug for $enum_type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $( &$enum_type_name::$variant => write!(f, "{}", stringify!( $variant))),*
                    ,&$enum_type_name::Unknown(ref x) => write!(f, "Unknown({:#08X}", x),
                }
            }
        }
        pub trait $enum_trait_name {
            fn $enum_getter_name(&self) -> $enum_type_name;
            $(
                fn $isa_name(&self) -> bool {
                    match self.$enum_getter_name() {
                        $enum_type_name::$variant => true,
                        _ => false
                    }
                }
            )*
            fn is_unknown(&self) -> bool {
                match self.$enum_getter_name() {
                    $enum_type_name::Unknown(_) => true,
                    _ => false
                }
            }
            fn get_unknown(&self) -> Option<$kind> {
                match self.$enum_getter_name() {
                    $enum_type_name::Unknown(x) => Some(x),
                    _ => None
                }
            }
        }
        impl From<$kind> for $enum_type_name {
            #[inline(always)]
            fn from(x: $kind) -> $enum_type_name {
                $(
                    if x == ($val as $kind) {
                        return $enum_type_name::$variant;
                    }
                )*
                $enum_type_name::Unknown(x)
            }
        }
        impl $enum_trait_name for $enum_type_name {
            #[inline(always)]
            fn $enum_getter_name(&self) -> $enum_type_name {
                self.clone()
            }
        }
        named!(pub $parser_name_be<&[u8],$enum_type_name,u32>, do_parse!(
            var: $nom_be >>
            ({ $enum_type_name::from(var) })
        ));
        named!(pub $parser_name_le<&[u8],$enum_type_name,u32>, do_parse!(
            var: $nom_le >>
            ({ $enum_type_name::from(var) })
        ));
    };
}
