#[derive(Debug)]
pub struct IacInput {
    pub bucket: Option<Bucket>,
    pub lambda: Option<Lambda>,
}

#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    pub has_event: bool,
}

#[derive(Debug)]
pub struct Lambda {
    pub name: String,
    pub memory: Option<u16>,
    pub time: Option<u16>,
}

impl IacInput {
    pub fn has_resources(&self) -> bool {
        fn is_ide_completion() -> bool {
            std::env::var_os("RUST_IDE_PROC_MACRO_COMPLETION_DUMMY_IDENTIFIER")
                .is_some_and(|it| !it.is_empty())
        }

        !is_ide_completion() && (self.bucket.is_some() || self.lambda.is_some())
    }
}
