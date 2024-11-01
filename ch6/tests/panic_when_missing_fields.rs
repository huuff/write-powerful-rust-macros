use builder_macro::Builder;

#[test]
#[should_panic]
fn should_panic_when_field_is_missing() {
    #[derive(Builder)]
    struct Gleipnir {
        _roots_of: String,
    }

    Gleipnir::builder().build();
}
