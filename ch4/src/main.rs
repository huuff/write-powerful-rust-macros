use make_public_macro::{delete, public};

#[public]
struct Example {
    first: String,
    pub second: u32,
}

#[delete]
struct EmptyStruct {}

#[public]
struct TupleExample(u8, u32);

fn main() {}
