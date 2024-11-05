struct Person {
    age: u32,
    name: String,
}

#[panic_to_result_macro::panic_to_result]
fn create_person_with_two_issues(name: String, age: u32) -> Result<Person, String> {
    if age > 30 {
        panic!();
    }
    Ok(Person { name, age })
}
