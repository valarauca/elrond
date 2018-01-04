#![allow(dead_code, unused_imports)]

mod traits;
pub use self::traits::ElfMagicNumber;

mod abi;
pub use self::abi::{ElfAbi, Abi};

mod endian;
pub use self::endian::{ElfEndian, Endian};

mod class;
pub use self::class::{ElfClass,Class};

mod magic;
pub use self::magic::{ElfMagicNumbers,parse_elf_magic};

