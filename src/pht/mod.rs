use std::io;
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

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[allow(dead_code)]
pub struct ProgramHeaderTable {
    pub kind: Elf_Word,
    pub offset: Elf_Off,
    pub vaddr: Elf_Addr,
    pub paddr: Elf_Addr,
    pub flags: Elf_Word,
    pub memsize: Elf_VarWord,
    pub align: Elf_VarWord,
    pub filesz: Elf_VarWord
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum RamFlags {
    Read,
    Write,
    Exec
}
const MASK: &'static [usize;3] = &[
    0x4,
    0x2,
    0x1
];
const OUTS: &'static [RamFlags;3] = &[
    RamFlags::Read,
    RamFlags::Write,
    RamFlags::Exec
];

impl ProgramHeaderTable {

    ///Read RAM flags
    #[allow(dead_code)]
    pub fn ram_flags(&self) -> Vec<RamFlags> {
        let flags: usize = self.flags.into();
        let lambda = | x: (&usize,&RamFlags) | -> Option<RamFlags> {
            if (flags & *x.0) != 0 {
                Some(x.1.clone())
            } else {
                None
            }
        };
        MASK.iter().zip(OUTS.iter()).filter_map(lambda).collect()
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

        let offset: usize = self.offset.into();
        let size: usize = self.filesz.into();
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
        let offset: usize = self.offset.into();
        let size: usize = self.filesz.into();
        let end = offset+size;
        if buff.len() >= end {
            Some( &buff[offset..end])
        } else {
            None
        }
    }
}

/*
 * IDFK why but 32bit and 64bit have the EXACT SAME
 * fields but they are in a different order because
 * Cache cooherance is _that_ important to linus
 *
 * I mean that makes sense what ever...
 */
named!(pub read_pht_64<ProgramHeaderTable>, chain!(
    ptype: read_word~
    pflags: read_word~
    poffset: read_off~
    pvaddr: read_addr~
    ppaddr: read_addr~
    pfilesz: read_plat_word~
    pmemsz: read_plat_word~
    palign: read_plat_word,
    || ProgramHeaderTable {
        kind: ptype,
        offset: poffset,
        vaddr: pvaddr,
        paddr: ppaddr,
        flags: pflags,
        memsize: pmemsz,
        align: palign,
        filesz: pfilesz
    }
));
named!(pub read_pht_32<ProgramHeaderTable>,chain!(
    ptype: read_word~
    poffset: read_off~
    pvaddr: read_addr~
    ppaddr: read_addr~
    pfilesz: read_plat_word~
    pmemsz: read_plat_word~
    pflags: read_word~
    palign: read_plat_word,
    || ProgramHeaderTable {
        kind: ptype,
        offset: poffset,
        vaddr: pvaddr,
        paddr: ppaddr,
        flags: pflags,
        memsize: pmemsz,
        align: palign,
        filesz: pfilesz
    }
));
