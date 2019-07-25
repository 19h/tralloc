extern crate tralloc;

use std::fs::File;

#[global_allocator]
static GLOBAL: tralloc::Allocator = tralloc::Allocator {};

fn main() {
    tralloc::Allocator::write_to_stderr();
    tralloc::Allocator::activate();

    let s = String::from("Hello world!");

    let mut v = Vec::new();

    v.push(1);

    println!("{:?}", s);
}
