use ch8_builder_macro::Builder;

#[allow(dead_code)]
#[derive(Builder)]
struct Gleipnir {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_builder_for_struct_with_one_renamed_property() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder().tops_of("mountains".to_string()).build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_props_one_custom_name() {
        #[derive(Builder)]
        struct Gleipnir {
            #[rename("tops_of")]
            roots_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleipnir::builder()
            .tops_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();

        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }
}
