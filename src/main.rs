#![feature(plugin)]
#![feature(str_char)]
#![feature(drain)]

#![plugin(clippy)]

mod commands;
mod hashmap_test;
mod sexp;

use std::io::{self, Read};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();

  match sexp::parse(&buf) {
    Err(())  => println!("Failed to parse sexp."),
    Ok(sexp) => println!("{:?}", hashmap_test::go(sexp)),
  }
}
