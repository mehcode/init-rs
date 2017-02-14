#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use std::env;
use syn::{Ident, ItemKind};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn init(_: TokenStream, target: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = target.to_string();

    // Create a "unique" name for the init function
    let package_name = env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
    let package_version = env::var("CARGO_PKG_VERSION").unwrap().replace('.', "_");
    let abi_name = Ident::new(format!("_rust_init_{}_{}", package_name, package_version));

    // Parse the string representation
    let item = syn::parse_item(&s).unwrap();
    match item.node {
        ItemKind::Fn(..) => {
            // Build
            let item_name = &item.ident;
            let gen = quote! {
                #[inline(always)]
                #item

                #[no_mangle]
                pub extern "C" fn #abi_name() {
                    #item_name();
                }
            };

            // Return
            gen.parse().unwrap()
        }

        _ => {
            // Not handled; pass-through
            target
        }
    }
}
