use aws_sdk_lambda::{
    error::SdkError as LambdaError,
    operation::{add_permission::AddPermissionError, create_function::CreateFunctionError},
};
use aws_sdk_s3::{
    error::SdkError as S3Error,
    operation::{
        create_bucket::CreateBucketError,
        put_bucket_notification_configuration::PutBucketNotificationConfigurationError,
    },
};
use proc_macro::TokenStream;
use proc_macro2::Span;

#[derive(Debug)]
pub enum IacError {
    Bucket(String),
    Lambda(String),
    Event(String),
}

impl IacError {
    pub fn into_compile_error(self) -> TokenStream {
        match self {
            IacError::Bucket(message) => syn::Error::new(
                Span::call_site(),
                format!("bucket could not be created: {message}"),
            )
            .into_compile_error()
            .into(),
            IacError::Lambda(message) => syn::Error::new(
                Span::call_site(),
                format!("lambda could not be created: {message}"),
            )
            .into_compile_error()
            .into(),
            IacError::Event(message) => syn::Error::new(
                Span::call_site(),
                format!("event could not be created: {message}"),
            )
            .into_compile_error()
            .into(),
        }
    }
}

macro_rules! generate_from_error {
    ($mine:expr, $aws:ty) => {
        impl From<$aws> for IacError {
            fn from(value: $aws) -> Self {
                $mine(format!("{value:?}"))
            }
        }
    };
}

generate_from_error!(IacError::Bucket, S3Error<CreateBucketError>);
generate_from_error!(IacError::Lambda, LambdaError<CreateFunctionError>);
generate_from_error!(
    IacError::Event,
    S3Error<PutBucketNotificationConfigurationError>
);
generate_from_error!(IacError::Lambda, LambdaError<AddPermissionError>);
