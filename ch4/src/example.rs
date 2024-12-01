use make_public_macro::public;

#[allow(dead_code)]
#[public(exclude(fourth, third))]
pub struct Example {
    first: String,
    pub second: u32,
    third: bool,
    fourth: String,
}

impl Example {
    pub fn new() -> Self {
        Example {
            first: "first".to_string(),
            second: 9,
            third: false,
            fourth: "lol".to_string(),
        }
    }
}
