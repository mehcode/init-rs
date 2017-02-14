#![feature(proc_macro)]

extern crate init;
use init::init;

#[init]
fn init() {
    println!("example::init");
}

fn main() {
    println!("example::main");
}
