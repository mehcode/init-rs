#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate lazy_static;

use std::env;
use syn::{Ident, ItemKind};
use std::sync::RwLock;
use proc_macro::TokenStream;

lazy_static! {
    static ref CRATES: RwLock<Vec<String>> = Default::default();
}

#[proc_macro_attribute]
pub fn init(_: TokenStream, target: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = target.to_string();

    // Parse the string representation
    match syn::parse_item(&s) {
        Ok(item) => {
            match item.node {
                // Function definition
                ItemKind::Fn(..) => {
                    // Create a "unique" name for the init function of self
                    let package_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
                    let self_abi_name = Ident::new(format!("_rust_init_{}", package_name));

                    // Create ABI names for all libraries
                    let mut crate_abi_names = Vec::new();
                    for crate_name in &*CRATES.read().unwrap() {
                        crate_abi_names.push(Ident::new(format!("_rust_init_{}", crate_name)));
                    }

                    // TODO: Put a ONCE wrapper to double ensure the init only gets called once

                    let crate_abi_names_2 = crate_abi_names.clone();
                    let item_name = &item.ident;
                    let gen = quote! {
                        #[inline(always)]
                        #item

                        extern "Rust" {
                            #(fn #crate_abi_names() -> ();)*
                        }

                        #[no_mangle]
                        #[allow(unused_unsafe)]
                        pub extern "C" fn #self_abi_name() {
                            // Ordered initialization for each _extern_ crate
                            unsafe { #(#crate_abi_names_2();)* }

                            // Initialization for _this_ crate
                            #item_name();
                        }
                    };

                    return gen.parse().unwrap();
                }

                // Extern Crate
                ItemKind::ExternCrate(name) => {
                    let item_name = name.unwrap_or(item.ident);

                    // Remember that this "extern crate" has an init
                    CRATES.write().unwrap().push(item_name.to_string());
                }

                _ => {
                }
            }
        }

        _ => {
        }
    }

    // Not handled; pass-through
    target
}
