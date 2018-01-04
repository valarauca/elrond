
use std::fmt;

/// Sometimes ELF32 vs ELF64 has
/// different lenght values.
///
/// Blindly converting everyting to
/// `usize` or `isize` could result in
/// a loss of precision on some platforms
/// so here ya go.
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum VarSize {
    Bits64(u64),
    Bits32(u32),
    Bits16(u16)
}
impl fmt::Debug for VarSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VarSize::Bits64(ref v) => write!(f, "64bit {:#016X}", v.clone()),
            &VarSize::Bits32(ref v) => write!(f, "32bit {:#016X}", v.clone()),
            &VarSize::Bits16(ref v) => write!(f, "16bit {:#016X}", v.clone()),
        }
    }
}
impl Into<usize> for VarSize {
    #[inline(always)]
    fn into(self) -> usize {
        match self {
            VarSize::Bits64(x) => x as usize,
            VarSize::Bits32(x) => x as usize,
            VarSize::Bits16(x) => x as usize,
        }
    }
}
impl From<u16> for VarSize {
    #[inline(always)]
    fn from(x: u16) -> VarSize {
        VarSize::Bits16(x)
    }
}
impl From<u32> for VarSize {
    #[inline(always)]
    fn from(x: u32) -> VarSize {
        VarSize::Bits32(x)
    }
}
impl From<u64> for VarSize {
    #[inline(always)]
    fn from(x: u64) -> VarSize {
        VarSize::Bits64(x)
    }
}

