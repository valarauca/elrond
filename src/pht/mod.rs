
use super::value::{
    Elf_Word,
    Elf_Addr,
    Elf_VarWord,
    Elf_Off
};

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
