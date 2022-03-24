#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod linking;

mod generated;
pub use generated::*;
// pub use crate::TF_Code::*;

pub mod library {
    use std::path::PathBuf;

    // Include the definition of `load` here. This allows hiding all of the "extra" linking-related
    // functions in the same place, without polluting the top-level namespace (which should only
    // contain foreign functions and types).
    #[doc(inline)]
    pub use super::generated::load;

    pub fn find() -> Option<PathBuf> {
        Some(PathBuf::from("/usr/local/lib/libtensorflow.so"))
    }
}
