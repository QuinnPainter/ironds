#![allow(unused_imports)]
#![allow(unused_macros)]

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

macro_rules! add_link_script {
    ($name:expr) => {{
        let out_path = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let mut out_file = File::create(out_path.join($name)).unwrap();
        out_file.write_all(include_bytes!(concat!("linkerscripts/", $name))).unwrap();
        println!("cargo:rustc-link-search={}", out_path.display());
    }};
}

fn main() {
    #[cfg(feature = "arm9")]
    add_link_script!("arm9_link.ld");
    #[cfg(feature = "arm7")]
    add_link_script!("arm7_link.ld");
    
    // todo: add "rerun-if-changed" statements here so the build.rs only gets rebuilt when necessary
}
