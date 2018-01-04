
use super::abi::{ElfAbi, Abi};
use super::endian::{ElfEndian, Endian};
use super::class::{ElfClass, Class};

/// Elf Magic Number information
///
/// A lot of data is packed into the first 16bytes
/// of an ElfFile.
///
/// This provides a simple way to extract it.
pub trait ElfMagicNumber: Abi + Endian + Class {
    
    /// Get the specific version of ABI
    ///
    /// This field is ABI and Platform specific, so its
    /// values are not standardized, so no enum.
    fn get_abi_version(&self) -> u8;
}
