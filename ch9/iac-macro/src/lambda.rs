use aws_config::{BehaviorVersion, Region};
use aws_sdk_lambda::{
    error::SdkError,
    operation::{
        add_permission::{AddPermissionError, AddPermissionOutput},
        create_function::{CreateFunctionError, CreateFunctionOutput},
    },
    types::FunctionCode,
    Client,
};

use crate::model::Lambda;

pub struct LambdaClient {
    client: Client,
}

impl LambdaClient {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("eu-south-2"))
            .load()
            .await;

        LambdaClient {
            client: Client::new(&config),
        }
    }

    pub async fn create_lambda(
        &self,
        lambda: &Lambda,
    ) -> Result<CreateFunctionOutput, SdkError<CreateFunctionError>> {
        let mut builder = self
            .client
            .create_function()
            .function_name(&lambda.name)
            .role("arn:aws:iam:1111111111:role/change")
            .code(
                FunctionCode::builder()
                    .s3_bucket("my-lambda_bucket")
                    .s3_key("example.zip")
                    .build(),
            );

        if let Some(time) = lambda.time {
            builder = builder.timeout(time.into());
        }

        if let Some(mem) = lambda.memory {
            builder = builder.memory_size(mem.into());
        }

        builder.send().await
    }

    pub async fn add_bucket_permission(
        &self,
        lambda: &Lambda,
        bucket: &str,
    ) -> Result<AddPermissionOutput, SdkError<AddPermissionError>> {
        self.client
            .add_permission()
            .function_name(&lambda.name)
            .principal("*")
            .statement_id("StatementId")
            .action("lambda:InvokeFunction")
            .source_arn(format!("arn:aws:s3::::{}", bucket))
            .send()
            .await
    }
}
