use std::io;
use super::value::{
    Elf_Word,
    Elf_Addr,
    Elf_VarWord,
    Elf_Off
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

Reference:

Cody the way you read the structure will differe based on 32/64 bit ELF

typedef struct elf32_phdr{
  Elf32_Word	p_type;
  Elf32_Off	p_offset;
  Elf32_Addr	p_vaddr;
  Elf32_Addr	p_paddr;
  Elf32_Word	p_filesz;
  Elf32_Word	p_memsz;
  Elf32_Word	p_flags;
  Elf32_Word	p_align;
} Elf32_Phdr;

typedef struct elf64_phdr {
  Elf64_Word p_type;
  Elf64_Word p_flags;
  Elf64_Off p_offset;		/* Segment file offset */
  Elf64_Addr p_vaddr;		/* Segment virtual address */
  Elf64_Addr p_paddr;		/* Segment physical address */
  Elf64_Xword p_filesz;		/* Segment size in file */
  Elf64_Xword p_memsz;		/* Segment size in memory */
  Elf64_Xword p_align;		/* Segment alignment, file & memory */
} Elf64_Phdr;

*/
