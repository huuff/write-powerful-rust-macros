use panic_to_result_macro::panic_to_result;

struct Person {
    age: u8,
    name: String,
}

#[panic_to_result]
fn create_person_with_result(age: u8, name: String) -> Result<Person, String> {
    if age > 30 {
        panic!("I hope I die before I get old")
    }

    Ok(Person { age, name })
}

fn main() {}
