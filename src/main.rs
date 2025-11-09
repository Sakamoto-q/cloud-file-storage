mod router;
mod unique;
mod encrypt;
mod service;
mod database;
mod middleware;

use std::env;
use dotenv::dotenv;
use std::error::Error;
use unique::Snowflake;
use actix_cors::Cors;
use router::{configure, AppState};
use database::{S3Client, MySQLClient};
use cf_turnstile::TurnstileClient;
use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    CryptoProvider::install_default(ring::default_provider()).unwrap();
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3client = S3Client::new_from_env(config)?;
    let snowflake = Snowflake::new_from_env()?;
    let mysql = MySQLClient::new_from_env().await?;
    mysql.init_database("schema.sql").await?;

    let secret = env::var("CLOUDFLARE_TURNSTILE_SECRET")?;
    let sitekey = env::var("CLOUDFLARE_TURNSTILE_SITE_KEY")?;
    
    let turnstile = TurnstileClient::new(secret.into());

    let app_state = web::Data::new(AppState::new(mysql, s3client, snowflake, turnstile, sitekey));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .configure(configure)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await?;

    Ok(())
}