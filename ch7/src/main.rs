use panic_to_result_macro::panic_to_result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[allow(dead_code)]
#[panic_to_result]
fn create_person(name: String, age: u32) -> Person {
    if age > 30 {
        panic!("I hope I die before I get old");
    }

    Person { name, age }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    #[should_panic]
    #[test]
    fn should_panic_on_invalid_age() {
        create_person("S".to_string(), 32).unwrap();
    }
}
