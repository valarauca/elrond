
use super::super::{ElfHeader, VarSize, BufferBorrow};

use super::{SectionType, SectionKind, SectionAttributes, SectMemAttrib};

/// Describes a section
pub trait Section<'a>: SectionType + SectionAttributes + ElfHeader<'a> {
    fn sh_name(&self) -> usize;
    fn name<'b>(&'b self) -> &'b str;
    fn link_name<'b>(&'b self) -> Option<&'b str>;
    fn sh_addr(&self) -> VarSize;
    fn sh_offset(&self) -> usize;
    fn sh_size(&self) -> usize;
    fn sh_link(&self) -> usize;
    fn sh_info(&self) -> usize;
    fn sh_addralign(&self) -> VarSize;
    fn sh_entsize(&self) -> usize;
    fn sh_flags<'b>(&'b self) -> &'b [SectMemAttrib] {
        self.get_attributes()
    }
    fn sh_type(&self) -> SectionKind {
        self.get_section_type()
    }
    fn get_data(&self) -> Option<&'a [u8]> {
        let buffer = self.get_buffer();
        let offset = self.sh_offset();
        let size = self.sh_size();
        let end = offset + size;
        if offset < buffer.len() && end < buffer.len() {
            Some(&buffer[offset..end])
        } else {
            None
        }
    }
}
