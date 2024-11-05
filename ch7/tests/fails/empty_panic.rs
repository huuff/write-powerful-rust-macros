use panic_to_result_macro::panic_to_result;

struct Person {
    age: u8,
    name: String,
}

#[panic_to_result]
fn create_person_with_empty_panic(age: u8, name: String) -> Person {
    if age > 30 {
        panic!();
    }

    Person { age, name }
}

fn main() {}