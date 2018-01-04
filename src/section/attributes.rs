
/// Attributes of a section
#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
pub enum SectMemAttrib {
    /// Section contains executable instructions
    ExecInstr,
    /// Section is allocated in the memory of program image
    Alloc,
    /// Section contains writable data
    Write, 
    /// Environment-specific usage
    MaskOS,
    /// Processor-specific usage
    MaskProc,
}

pub trait SectionAttributes {
    fn get_attributes<'a>(&'a self) -> &'a [SectMemAttrib];
    fn is_exec_instr(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == SectMemAttrib::ExecInstr)
            .next()
            .is_some()
    }
    fn is_write(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == SectMemAttrib::Write)
            .next()
            .is_some()
    }
    fn is_alloc(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == SectMemAttrib::Alloc)
            .next()
            .is_some()
    }
    fn is_mask_os(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == SectMemAttrib::MaskOS)
            .next()
            .is_some()
    }
    fn is_mask_proc(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == SectMemAttrib::MaskProc)
            .next()
            .is_some()
    }
}

const ARR: &'static [(SectMemAttrib,u32)] = &[
    (SectMemAttrib::Write, 0x1),
    (SectMemAttrib::Alloc, 0x2),
    (SectMemAttrib::ExecInstr, 0x4),
    (SectMemAttrib::MaskOS, 0x0F000000),
    (SectMemAttrib::MaskProc, 0xF0000000)
];

pub fn build_attributes(x: u32) -> Box<[SectMemAttrib]> {
    ARR.iter()
        .filter(|v| v.1.clone() & x > 0)
        .map(|v| v.0)
        .collect::<Vec<SectMemAttrib>>()
        .into_boxed_slice()
}

