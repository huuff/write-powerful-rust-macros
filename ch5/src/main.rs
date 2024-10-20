use macros::private;

private! {
    struct Privy {
        pub public_var: u8,
        priv_var: u8,
    }
}

fn main() {
    println!("Hello, world!");
}
