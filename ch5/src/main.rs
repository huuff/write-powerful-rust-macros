use macros::{compose, hello_world, private};

private! {
    #[allow(dead_code)]
    struct Privy {
        pub public_var: u8,
        priv_var: u8,
    }
}

// private! {
//     struct Lol(u8);
// }

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

hello_world!(Privy);

fn main() {
    #[rustfmt::skip]
    let composed = compose!(add_one >> add_one >> stringify);

    println!("{}", composed(7));

    let privy = Privy::new(0, 0);

    privy.hello_world();
}
