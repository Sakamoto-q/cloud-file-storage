use std::sync::Arc;
use crate::unique::Snowflake;
use cf_turnstile::TurnstileClient;
use crate::database::{S3Client, MySQLClient};

pub struct AppState {
    pub mysql: Arc<MySQLClient>,
    pub s3client: Arc<S3Client>,
    pub snowflake: Arc<Snowflake>,
    pub turnstile_client: Arc<TurnstileClient>,
    pub turnstile_sitekey: String,
}

impl AppState {
    pub fn new(mysql_client: MySQLClient, s3_client: S3Client, snowflake: Snowflake, turnstile_client: TurnstileClient, turnstile_sitekey: String) -> Self {
        Self {
            mysql: Arc::new(mysql_client),
            s3client: Arc::new(s3_client),
            snowflake: Arc::new(snowflake),
            turnstile_client: Arc::new(turnstile_client),
            turnstile_sitekey: turnstile_sitekey,
        }
    }
}