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

#[allow(dead_code, clippy::never_loop, while_true)]
#[panic_to_result]
fn always_errs_creating_person(name: String, age: u32) -> Person {
    while true {
        panic!("don't wanna create person");
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

    #[test]
    fn should_err_on_invalid_age() {
        let actual = create_person("S".to_string(), 32);

        assert_eq!(
            actual.expect_err("this should be an error"),
            "I hope I die before I get old".to_string()
        );
    }

    #[test]
    fn what_errs_always_errs() {
        let result = always_errs_creating_person("Mike".to_string(), 81);

        assert_eq!(
            result.expect_err("this should be an error"),
            "don't wanna create person",
        );
    }
}
