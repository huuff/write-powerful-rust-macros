#[macro_use]
extern crate hello_world_macro;

#[derive(HelloNoDep)]
struct Example;

pub fn main() {
    let example = Example;
    example.hello_world();
}
