/*
Copyright 2016 William Cody Laeder

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/


//! Elrond reads Elf files
//!
//! This is an embryonic Elf Parser written in NOM. It is not feature complete. It requires a lot
//! more testing this is just a cleaned up version I can throw on `crates.io` to start testing
//! with.
//!
//! There are no benchmarks.
//!
//! There are no cool features. It doesn't support dynamic objects (yet).
//!
//! It isn't _super_ ergonomic. I more-or-less read BSD source code
//! and a pdf to produce this. I'm not even sure it works.

#[macro_use]
extern crate nom;
extern crate byteorder;

#[macro_use]
mod macros;
mod value;
mod header;
mod section;
mod pht;

#[allow(unused_imports)]
pub use section::{
    Section,
    SectionType,
    SegFlag
};
use section::read_section;
#[allow(unused_imports)]
pub use value::{
    Elf_Addr,
    Elf_Off,
    Elf_VarWord,
    Elf_Half,
    Elf_SHalf,
    Elf_Word,
    Elf_SWord,
    Elf_XWord,
    Elf_Sxword
};
#[allow(unused_imports)]
pub use header::{
    Endian,
    Platform,
    ElfHeader,
    ABI,
    Kind,
    Arch
};
use header::read_elf_header;
use pht::{read_pht_64,read_pht_32};
use value::get_platform;
#[allow(unused_imports)]
pub use pht::{
    ProgramHeaderTable,
    RamFlags
};
pub use nom::IResult;

///Holds the ELF Data
#[derive(Debug)]
pub struct Elf {
    pub header: ElfHeader,
    pub sections: Vec<Section>,
    pub progheader: Vec<ProgramHeaderTable>
}
impl Elf {
    /// Parse a buffer.
    ///
    /// This function will extract the `Section` and `ProgramHeaderTable` information.
    /// It does not try to extract their full binary/data information. You can all the
    /// individual `Section` and/or `ProgramHeaderTable` for that. They expose the
    /// `read_data` and `borrow_data` functions which can fetch the contents the
    /// header is referencing.
    pub fn parse(x: &[u8] ) -> Option<Elf> {
        let (_,header) = match read_elf_header(x) {
            IResult::Done(x,y) => (x,y),
            _ => return None
        };
        let mut ret_sec = Vec::<Section>::with_capacity(0);
        let mut ret_pht = Vec::<ProgramHeaderTable>::with_capacity(0);

        //get section info
        let section_offset: usize = header.shoff.into();
        let section_size: usize = header.shentsize.into();
        let section_num: usize = header.shnum.into();
        if section_num > 0 {
            let section_table_end: usize = section_size * section_num + section_offset;
            let sec = &x[section_offset..section_table_end];
            let mut sections = Vec::with_capacity(section_num);
            let mut val: &[u8] = sec;
            for _ in 0..section_num {
                let (rem, sec) = match read_section(val) {
                    IResult::Done(x,y) => (x,y),
                    _ => return None
                };
                val = rem;
                sections.push(sec);
            }
            ret_sec = sections;
        }

        //get prog header table info
        let pht_offset: usize = header.phoff.into();
        let pht_size: usize = header.shentsize.into();
        let pht_num: usize = header.shnum.into();
        if pht_num > 0 {
            let pht_end: usize = pht_size * pht_num + pht_offset;
            let sec = &x[pht_offset..pht_end];
            let mut pht = Vec::with_capacity(pht_num);
            let mut val: &[u8] = sec;
            for _ in 0..pht_num {

                //layout of pht changes based on 32/64
                if get_platform() == Platform::Bit32 {
                    let (rem, sec) = match read_pht_32(val) {
                        IResult::Done(x,y) => (x,y),
                        _ => return None
                    };
                    val = rem;
                    pht.push(sec);
                } else {
                    let (rem, sec) = match read_pht_64(val) {
                        IResult::Done(x,y) => (x,y),
                        _ => return None
                    };
                    val = rem;
                    pht.push(sec);
                }
            }
            ret_pht = pht;
        }
        Some(Elf {
            header: header,
            sections: ret_sec,
            progheader: ret_pht
        })
    }
}
