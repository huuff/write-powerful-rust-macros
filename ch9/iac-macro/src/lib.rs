mod error;
mod input;
mod lambda;
mod model;
mod s3;

use crate::s3::S3Client;
use error::IacError;
use lambda::LambdaClient;
use model::IacInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);

    if ii.has_resources() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        match rt.block_on(create_infra(ii)) {
            Ok(_) => quote!().into(),
            Err(e) => e.into_compile_error(),
        }
    } else {
        quote!().into()
    }
}

async fn create_infra(iac_input: IacInput) -> Result<(), IacError> {
    let s3_client = S3Client::new().await;
    let lambda_client = LambdaClient::new().await;

    let mut output = None;

    if let Some(lambda) = &iac_input.lambda {
        eprintln!("creating lambda...");
        output = Some(lambda_client.create_lambda(lambda).await?);
    }

    if let Some(bucket) = &iac_input.bucket {
        eprintln!("creating bucket");
        s3_client.create_bucket(bucket).await?;
        if bucket.has_event {
            eprintln!("linking bucket and lambda by an element");
            let lambda_arn_output = output.expect("when we have an event, we should have a lambda");
            let lambda = iac_input
                .lambda
                .expect("when we have an event, we should have a lambda");

            let lambda_arn = lambda_arn_output
                .function_arn()
                .expect("creating a lambda should return its ARN");
            lambda_client
                .add_bucket_permission(&lambda, &bucket.name)
                .await?;

            s3_client
                .link_bucket_with_lambda(bucket, lambda_arn)
                .await?;
        }
    }

    Ok(())
}
