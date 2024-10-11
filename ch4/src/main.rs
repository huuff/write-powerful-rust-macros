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

#[public]
enum ExampleEnum {
    Struct { field_1: u8, field_2: String },
    Tuple(u8),
}

fn main() {}
