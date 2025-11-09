use std::error::Error;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct Bcrypt;

impl Bcrypt {
    pub fn hash(plain: &str) -> Result<String, Box<dyn Error>> {
        if plain.is_empty() {
            return Err("plain cannot be empty".into());
        }
        
        if plain.len() < 8 {
            return Err("plain must be at least 8 characters".into());
        }
        
        let hashed = hash(plain, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify(plain: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
        if plain.is_empty() || hash.is_empty() {
            return Err("plain and hash cannot be empty".into());
        }
        
        let is_valid = verify(plain, hash)?;
        Ok(is_valid)
    }
}