
use super::{
    Elf_Word,
    Elf_Addr,
    Elf_VarWord,
    Elf_Off
};
use super::value::{
    read_word,
    read_plat_word,
    read_addr,
    read_off
};
pub mod kind;
#[allow(dead_code)]
use self::kind::read_sec_kind;
pub use self::kind::SectionType;
use std::io;

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct Section {
    pub sh_name: Elf_Word,
    pub sh_type: SectionType,
    pub sh_flags: Elf_VarWord,
    pub sh_addr: Elf_Addr,
    pub sh_offset: Elf_Off,
    pub sh_size: Elf_VarWord,
    pub sh_link: Elf_Word,
    pub sh_info: Elf_Word,
    pub sh_addralign: Elf_VarWord,
    pub sh_entsize: Elf_VarWord
}

/*
 * Handle Segement Flags
 */
 #[allow(dead_code)]
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SegFlag {
    Write,
    Alloc,
    ExecInstr,
    RelaLivePatch,
    RoAfterInit,
    MaskProc
}

const MASK: &'static [usize;6] = &[0x1,0x2,0x4,0x00100000,0x00200000,0xf0000000];
const OUTS: &'static [SegFlag;6] = &[
    SegFlag::Write,
    SegFlag::Alloc,
    SegFlag::ExecInstr,
    SegFlag::RelaLivePatch,
    SegFlag::RoAfterInit,
    SegFlag::MaskProc
];

impl Section {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn get_flags(&self) -> Vec<SegFlag> {
        let val: usize = self.sh_flags.into();
        let lambda = |x: (&usize,&SegFlag) | -> Option<SegFlag> {
            if (val & *x.0) != 0 {
                Some(x.1.clone())
            } else {
                None
            }
        };
        MASK.iter()
            .zip(OUTS.iter())
            .filter_map(lambda)
            .collect()
    }
    /// Return's that data from the File associated with this section
    #[allow(dead_code)]
    pub fn read_data<R: io::Read+io::Seek>(&self,r: &mut R) -> io::Result<Vec<u8>> {
        #[allow(unused_imports)]
        use std::io::Read;
        #[allow(unused_imports)]
        use std::io::SeekFrom;
        #[allow(unused_imports)]
        use std::io::Seek;

        let offset: usize = self.sh_offset.into();
        let size: usize = self.sh_size.into();
        let mut ret_vec = Vec::with_capacity(size);
        unsafe{ret_vec.set_len(size)};
        let offset = SeekFrom::Start(offset as u64);
        let _ = r.seek(offset)?;
        let _ = r.read_exact(ret_vec.as_mut_slice())?;
        Ok(ret_vec)
    }
    /// Return the data as an aliased borrow
    ///
    /// #None
    ///
    /// Return's an `Option::None` when the buffer is insufficient size
    #[allow(dead_code)]
    pub fn borrow_data<'a>(&self, buff: &'a [u8]) -> Option<&'a [u8]> {
        let offset: usize = self.sh_offset.into();
        let size: usize = self.sh_size.into();
        let end = offset+size;
        if buff.len() >= end {
            Some( &buff[offset..end])
        } else {
            None
        }
    }
}

named!(pub read_section<Section>, chain!(
    shname: read_word~
    kind: read_sec_kind~
    flags: read_plat_word~
    addr: read_addr~
    offset: read_off~
    size: read_plat_word~
    link: read_word~
    info: read_word~
    addralign: read_plat_word~
    entsize: read_plat_word,
    || Section {
        sh_name: shname,
        sh_type: kind,
        sh_flags: flags,
        sh_addr: addr,
        sh_offset: offset,
        sh_size: size,
        sh_link: link,
        sh_info: info,
        sh_addralign: addralign,
        sh_entsize: entsize
    }
));
