use macros::{compose, private};

private! {
    struct Privy {
        pub public_var: u8,
        priv_var: u8,
    }
}

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    #[rustfmt::skip]
    let composed = compose!(add_one . add_one . stringify);

    println!("{}", composed(7));
}
