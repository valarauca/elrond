
#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
pub enum Attributes {
    Execute,
    Write,
    Read,
}

pub trait MemoryAttributes {
    fn get_attributes<'a>(&'a self) -> &'a [Attributes];
    fn executable(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == Attributes::Execute)
            .next()
            .is_some()
    }
    fn writable(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == Attributes::Write)
            .next()
            .is_some()
    }
    fn readable(&self) -> bool {
        self.get_attributes()
            .iter()
            .filter(|x| **x == Attributes::Read)
            .next()
            .is_some()
    }
}

const ARR: &'static [(Attributes,u32)] = &[
    (Attributes::Read, 4),
    (Attributes::Write, 2),
    (Attributes::Execute, 1),
];

pub fn build_attributes(x: u32) -> Box<[Attributes]> {
    ARR.iter()
        .filter(|v| v.1.clone() & x > 0)
        .map(|v| v.0)
        .collect::<Vec<Attributes>>()
        .into_boxed_slice()
}
