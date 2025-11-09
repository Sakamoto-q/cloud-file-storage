use std::env;
use std::error::Error;
use snowflake_id_generator::multi_thread::async_generator::SnowflakeGenerator;

pub struct Snowflake {
    generator: SnowflakeGenerator,
}

impl Snowflake {
    pub fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let epoch_num = env::var("SNOWFLAKE_EPOCH")?
            .parse::<i64>()?;

        let worker_id_num = env::var("SNOWFLAKE_WORKER_ID")?
            .parse::<u16>()?;
        
        let generator = SnowflakeGenerator::new(epoch_num, worker_id_num)?;
        
        Ok(Self {
            generator,
        })
    }

    pub async fn generate(&self) -> i64 {
        self.generator.generate_id().await
    }
}