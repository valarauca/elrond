

mod flags;
pub use self::flags::{Attributes,MemoryAttributes};

mod ph_type;
pub use self::ph_type::{ProgramHeaderType,HeaderType};

mod traits;
pub use self::traits::{PHDR};

mod phdr32;
mod phdr64;
mod phdr;
pub use self::phdr::{parse_phdr};
