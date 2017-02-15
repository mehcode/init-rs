#![feature(proc_macro)]

#[init]
extern crate example_lib;

extern crate init;

use init::init;

#[init]
fn init() {
    println!("example-bin::init");
}

fn main() {
    println!("example-bin::main");
}
