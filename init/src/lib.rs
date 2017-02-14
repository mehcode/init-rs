#![feature(macro_reexport)]

//! Mark a function to run before main.
//!
//! `src/main.rs`
//!
//! ```rust
//! extern crate init;
//! use init::init;
//!
//! #[init]
//! fn init() {
//! }
//!
//! fn main() {
//! }
//! ```
//!
//! `build.rs`
//!
//! ```rust
//! extern crate init;
//!
//! fn main() {
//!     init::build();
//! }
//! ```

extern crate gcc;

#[macro_reexport(init)]
extern crate init_codegen;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::env;

/// Build (and link) supporting code. Intended to be used in a build.rs file
pub fn build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
    let package_version = env::var("CARGO_PKG_VERSION").unwrap().replace('.', "_");
    let spec = format!("{}_{}", package_name, package_version);
    let c_abi_name = format!("_c_init_{}", spec);
    let rust_abi_name = format!("_rust_init_{}", spec);
    let c_lib_name = format!("lib_init_{}.a", spec);
    let c_src_name = format!("init_{}.c", spec);

    let source = format!(r#"
        extern void {}();
        extern void _rust_init_example_lib_0_1_0();

        __attribute__ ((constructor, externally_visible))
        void {}() {{
            {}();
        }}
    "#, rust_abi_name, c_abi_name, rust_abi_name);

    let dest_path = Path::new(&out_dir).join(c_src_name);

    {
        let mut f = File::create(&dest_path).unwrap();

        f.write_all(source.as_bytes()).unwrap();
    }

    gcc::compile_library(&c_lib_name, &[&dest_path.to_string_lossy()]);
}
