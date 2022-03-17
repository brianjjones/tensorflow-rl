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

    /// Return the location of the shared library `openvino-sys` will link to. If compiled with
    /// runtime linking, this will attempt to discover the location of a `inference_engine_c_api`
    /// shared library on the system. Otherwise (with dynamic linking or compilation from source),
    /// this relies on a static path discovered at build time.
    ///
    /// Knowing the location of the OpenVINO libraries is critical to avoid errors, unfortunately.
    /// OpenVINO loads target-specific libraries on demand for performing inference. To do so, it
    /// relies on a `plugins.xml` file that maps targets (e.g. CPU) to the target-specific
    /// implementation library. At runtime, users must pass the path to this file so that OpenVINO
    /// can inspect it and load the required libraries to satisfy the user's specified targets. By
    /// default, the `plugins.xml` file is found in the same directory as the libraries, e.g.
    /// `find().unwrap().parent()`.
    pub fn find() -> Option<PathBuf> {
        Some(PathBuf::from("/usr/local/lib/libtensorflow.so"))
        // Some(PathBuf::from(env!("/usr/local/lib/libtensorflow.so")))
        // if cfg!(feature = "runtime-linking") {
        //     openvino_finder::find("inference_engine_c_api")
        // } else {
        //     Some(PathBuf::from(env!("OPENVINO_LIB_PATH")))
        // }
    }
}
