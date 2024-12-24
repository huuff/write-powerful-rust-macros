use aws_config::BehaviorVersion;
use aws_sdk_s3::types::{Event, LambdaFunctionConfiguration, NotificationConfiguration};
use aws_sdk_s3::{
    error::SdkError,
    operation::{
        create_bucket::{CreateBucketError, CreateBucketOutput},
        put_bucket_notification_configuration::{
            PutBucketNotificationConfigurationError, PutBucketNotificationConfigurationOutput,
        },
    },
    types::{BucketLocationConstraint, CreateBucketConfiguration},
    Client,
};

use crate::model::Bucket;

pub struct S3Client {
    client: Client,
    region: String,
}

impl S3Client {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        S3Client {
            client: Client::new(&config),
            region: "eu-south-2".to_string(),
        }
    }

    pub async fn create_bucket(
        &self,
        bucket: &Bucket,
    ) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
        let constraint = BucketLocationConstraint::from(self.region.as_str());
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        self.client
            .create_bucket()
            .bucket(&bucket.name)
            .create_bucket_configuration(cfg)
            .send()
            .await
    }

    pub async fn link_bucket_with_lambda(
        &self,
        bucket: &Bucket,
        lambda_arn: &str,
    ) -> Result<
        PutBucketNotificationConfigurationOutput,
        SdkError<PutBucketNotificationConfigurationError>,
    > {
        self.client
            .put_bucket_notification_configuration()
            .bucket(&bucket.name)
            .notification_configuration(
                NotificationConfiguration::builder()
                    .lambda_function_configurations(
                        LambdaFunctionConfiguration::builder()
                            .lambda_function_arn(lambda_arn)
                            .events(Event::from("s3:ObjectCreated:*"))
                            .build()
                            .expect("to create valid lambda function config"),
                    )
                    .build(),
            )
            .send()
            .await
    }
}
