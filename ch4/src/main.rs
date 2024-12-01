#![allow(dead_code)]
mod example;

use example::Example;

fn main() {
    let e = Example::new();

    println!("{}", e.first);
    println!("{}", e.second);
    // println!("{}", e.third);
}
