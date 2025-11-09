use std::env;
use std::error::Error;
use std::time::Duration;
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_config::SdkConfig;

pub struct S3Client {
    client: Client,
    bucket: String,
    upload_expire: u64,
    download_expire: u64,
}

impl S3Client {
    pub fn new_from_env(config: SdkConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let bucket = env::var("AWS_S3_BUCKET")?;
        
        let upload_expire = env::var("AWS_S3_UPLOAD_EXPIRE")?
            .parse::<u64>()?;

        let download_expire = env::var("AWS_S3_DOWNLOAD_EXPIRE")?
            .parse::<u64>()?;

        let client = Client::new(&config);
        Ok(Self {
            client,
            bucket,
            upload_expire,
            download_expire,
        })
    }

    pub async fn upload_url(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let presign_config = PresigningConfig::expires_in(Duration::from_secs(self.upload_expire))?;
        let presigned_req = self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presign_config)
            .await?;
        Ok(presigned_req.uri().to_string())
    }

    pub async fn download_url(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let presign_config = PresigningConfig::expires_in(Duration::from_secs(self.download_expire))?;
        let presigned_req = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presign_config)
            .await?;
        Ok(presigned_req.uri().to_string())
    }

    pub async fn list(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .send()
            .await?;

        let mut keys = Vec::new();
        
        for obj in response.contents() {
            if let Some(key) = obj.key() {
                keys.push(key.to_string());
            }
        }
        
        Ok(keys)
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.client.delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }
}