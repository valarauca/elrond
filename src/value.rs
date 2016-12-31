
use std::cell::RefCell;
use super::nom::IResult;
use super::header::endian::Endian;
use super::header::wordsize::Platform;

thread_local!( static ENDIAN: RefCell<Endian> = RefCell::new(Endian::Little));
thread_local!( static PLATFORM: RefCell<Platform> = RefCell::new(Platform::Bit64));

///Set thread global endian value
#[allow(dead_code)]
#[inline(always)]
fn set_endian(x: Endian) {
    ENDIAN.with(|cell| {
        *cell.borrow_mut() = x;
    });
}
///Set thread global platform value
#[allow(dead_code)]
#[inline(always)]
fn set_word(x: Platform) {
    PLATFORM.with(|cell| {
        *cell.borrow_mut() = x;
    });
}
///Defines the way words are read from this point onward
pub fn define_reader(a: &Endian, b: &Platform) {
    set_endian(a.clone());
    set_word(b.clone());
}

/*
 * Read Endianess/Plaform word size
 */
#[inline(always)]
fn get_endian() -> Endian {
    ENDIAN.with(|cell| -> Endian {
        *cell.borrow()
    })
}
#[inline(always)]
pub fn get_platform() -> Platform {
    PLATFORM.with(|cell| -> Platform {
        *cell.borrow()
    })
}


/*
 * Macro based definations of readers
 * What is defined here is ednian agnostic
 * ways of reading from the buffer
 * and made nicely compible with Nom
 */
generate_reader!{
    read_u16;
    to_u16;
    u16;
}
generate_reader!{
    read_i16;
    to_i16;
    i16;
}
generate_reader!{
    read_u32;
    to_u32;
    u32;
}
generate_reader!{
    read_i32;
    to_i32;
    i32;
}
generate_reader!{
    read_u64;
    to_u64;
    u64;
}
generate_reader!{
    read_i64;
    to_i64;
    i64;
}

/*
 * Read the correctly sized word dependent on the platform
 */
#[inline(always)]
#[allow(dead_code)]
fn read_platform_word<'a>(i: &'a[u8]) -> IResult<&'a[u8],u64> {
    match get_platform() {
        Platform::Bit32 => match read_u32(i) {
            IResult::Done(x,y) => IResult::Done(x,y as u64),
            IResult::Error(z) => IResult::Error(z),
            IResult::Incomplete(z) => IResult::Incomplete(z)
        },
        Platform::Bit64 => read_u64(i)
    }
}

/*
 * Define the types which are different for different
 * platforms
 */
std_val!{
    @PLATFORM
    @ID:
        Elf_Addr;
    @INTO_FROM:
        usize, u32, u64;
    @READER:
        read_platform_word => read_addr;
}
std_val!{
    @PLATFORM
    @ID:
        Elf_Off;
    @INTO_FROM:
        usize, u32, u64;
    @READER:
        read_platform_word => read_off;
}
std_val!{
    @PLATFORM
    @ID:
        Elf_VarWord;
    @INTO_FROM:
        usize, u32, u64;
    @READER:
        read_platform_word => read_plat_word;
}
/*
 * Define the standard types
 */
std_val!{
    @ID:
        Elf_Half;
        u16;
    @INTO_FROM:
        usize, u16, u32, u64;
    @READER:
        read_u16 => read_half;
}
std_val!{
    @ID:
        Elf_SHalf;
        i16;
    @INTO_FROM:
        isize, i16, i32, i64;
    @READER:
        read_i16 => read_shalf;
}
std_val!{
    @ID:
        Elf_Word;
        u32;
    @INTO_FROM:
        usize, u32, u64;
    @READER:
        read_u32 => read_word;
}
std_val!{
    @ID:
        Elf_SWord;
        i32;
    @INTO_FROM:
        isize, i32, i64;
    @READER:
        read_i32 => read_sword;
}
std_val!{
    @ID:
        Elf_XWord;
        u64;
    @INTO_FROM:
        usize, u64;
    @READER:
        read_u64 => read_xword;
}
std_val!{
    @ID:
        Elf_Sxword;
        i64;
    @INTO_FROM:
        isize, i64;
    @READER:
        read_i64 => read_sxword;
}
