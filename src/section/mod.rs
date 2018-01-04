
mod traits;
pub use self::traits::Section;

mod section_type;
pub use self::section_type::{SectionKind,SectionType};

mod attributes;
pub use self::attributes::{SectionAttributes,SectMemAttrib};

mod section64;
mod section32;
mod section;
pub use self::section::{Sections, SectionsIter};

use std::borrow::Cow;
pub fn find_null<'a>(buffer: &'a [u8]) -> Option<Cow<'a,str>> {
    let mut term = 0usize;
    let len = buffer.len();
    for i in 0..len {
        if buffer[i] == 0 {
            break;
        } else {
            term += 1;
            continue;
        }
    }
    let temp = &buffer[0..term];
    Some(String::from_utf8_lossy(temp))
}
