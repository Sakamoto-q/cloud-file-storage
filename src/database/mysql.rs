use std::env;
use std::error::Error;
use sqlx::{query_as, MySql, Pool};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub icon_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct File {
    pub id: String,
    pub filename: String,
    pub owner_id: String,
    #[sqlx(json)]
    pub accessible_user_ids: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub session_key: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
}

pub struct MySQLClient {
    pool: Pool<MySql>,
}

impl MySQLClient {
    pub async fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("MY_SQL_DATABASE_URL")?;
        let pool = sqlx::MySqlPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init_database(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let schema = std::fs::read_to_string(filename)?;
        
        for statement in schema.split(';').filter(|s| !s.trim().is_empty()) {
            match sqlx::raw_sql(statement).execute(&self.pool).await {
                Ok(_) => println!("✓ Executed: {}", statement.trim().lines().next().unwrap_or("")),
                Err(e) => {
                    if !e.to_string().contains("database exists") && !e.to_string().contains("already exists") {
                        return Err(Box::new(e));
                    }
                }
            }
        }
        
        Ok(())
    }

    pub async fn create_user(
        &self,
        id: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, Box<dyn Error>> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES (?, ?, ?, ?)"
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let user = query_as::<_, User>(
            "SELECT id, email, password_hash, icon_url, created_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        println!("✓ User created: {} ({})", id, email);
        Ok(user)
    }

    pub async fn get_user(&self, id: &str) -> Result<Option<User>, Box<dyn Error>> {
        let user = query_as::<_, User>(
            "SELECT id, email, password_hash, icon_url, created_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let user = query_as::<_, User>(
            "SELECT id, email, password_hash, icon_url, created_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user_icon(&self, id: &str, icon_url: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("UPDATE users SET icon_url = ? WHERE id = ?")
            .bind(icon_url)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_file(
        &self,
        id: &str,
        filename: &str,
        owner_id: &str,
        accessible_user_ids: &str,
    ) -> Result<File, Box<dyn Error>> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO files (id, filename, owner_id, accessible_user_ids, created_at) VALUES (?, ?, ?, CAST(? AS JSON), ?)"
        )
        .bind(id)
        .bind(filename)
        .bind(owner_id)
        .bind(accessible_user_ids)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let file = query_as::<_, File>(
            "SELECT id, filename, owner_id, accessible_user_ids, created_at FROM files WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        println!("✓ File created: {} ({}) by {}", id, filename, owner_id);
        Ok(file)
    }

    pub async fn get_file(&self, id: &str) -> Result<Option<File>, Box<dyn Error>> {
        let file = query_as::<_, File>(
            "SELECT id, filename, owner_id, accessible_user_ids, created_at FROM files WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn update_file_access(
        &self,
        id: &str,
        accessible_user_ids: &str,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query("UPDATE files SET accessible_user_ids = CAST(? AS JSON) WHERE id = ?")
            .bind(accessible_user_ids)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_file(&self, id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn list_user_files(&self, user_id: &str) -> Result<Vec<File>, Box<dyn Error>> {
        let files = query_as::<_, File>(
            "SELECT id, filename, owner_id, accessible_user_ids, created_at FROM files"
        )
        .fetch_all(&self.pool)
        .await?;

        let user_files: Vec<File> = files
            .into_iter()
            .filter(|f| {
                f.owner_id == user_id || 
                if let Ok(user_ids) = serde_json::from_value::<Vec<String>>(f.accessible_user_ids.clone()) {
                    user_ids.contains(&user_id.to_string())
                } else {
                    false
                }
            })
            .collect();

        println!("✓ Listed {} files for user {}", user_files.len(), user_id);
        Ok(user_files)
    }

    pub async fn add_user_to_file(&self, file_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
        let file = self.get_file(file_id).await?;
        if file.is_none() {
            return Err("File not found".into());
        }
        let file = file.unwrap();
        
        let mut user_ids: Vec<String> = serde_json::from_value(file.accessible_user_ids)
            .unwrap_or_default();
        
        if !user_ids.contains(&user_id.to_string()) {
            user_ids.push(user_id.to_string());
        }
        
        let updated_ids = serde_json::to_string(&user_ids)?;
        self.update_file_access(file_id, &updated_ids).await?;
        
        Ok(())
    }

    pub async fn remove_user_from_file(&self, file_id: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
        let file = self.get_file(file_id).await?;
        if file.is_none() {
            return Err("File not found".into());
        }
        let file = file.unwrap();
        
        let mut user_ids: Vec<String> = serde_json::from_value(file.accessible_user_ids)
            .unwrap_or_default();
        
        user_ids.retain(|id| id != user_id);
        
        let updated_ids = serde_json::to_string(&user_ids)?;
        self.update_file_access(file_id, &updated_ids).await?;
        
        Ok(())
    }

    pub async fn check_user_can_access(&self, file_id: &str, user_id: &str) -> Result<bool, Box<dyn Error>> {
        let file = self.get_file(file_id).await?;
        if file.is_none() {
            return Err("File not found".into());
        }
        let file = file.unwrap();
        
        if file.owner_id == user_id {
            return Ok(true);
        }
        
        let user_ids: Vec<String> = serde_json::from_value(file.accessible_user_ids)
            .unwrap_or_default();
        
        Ok(user_ids.contains(&user_id.to_string()))
    }

    pub async fn check_user_is_owner(&self, file_id: &str, user_id: &str) -> Result<bool, Box<dyn Error>> {
        let file = self.get_file(file_id).await?;
        if file.is_none() {
            return Err("File not found".into());
        }
        let file = file.unwrap();
        
        Ok(file.owner_id == user_id)
    }

    pub async fn create_session(
        &self,
        id: &str,
        user_id: &str,
        session_key: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Session, Box<dyn Error>> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO sessions (id, user_id, session_key, ip_address, user_agent, created_at, last_accessed_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(user_id)
        .bind(session_key)
        .bind(ip_address)
        .bind(user_agent)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let session = query_as::<_, Session>(
            "SELECT id, user_id, session_key, ip_address, user_agent, created_at, last_accessed_at FROM sessions WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        println!("✓ Session created for user: {}", user_id);
        Ok(session)
    }

    pub async fn get_session(&self, session_key: &str) -> Result<Option<Session>, Box<dyn Error>> {
        let session = query_as::<_, Session>(
            "SELECT id, user_id, session_key, ip_address, user_agent, created_at, last_accessed_at FROM sessions WHERE session_key = ?"
        )
        .bind(session_key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn verify_session(&self, session_key: &str, ip_address: &str) -> Result<Option<Session>, Box<dyn Error>> {
        if let Some(session) = self.get_session(session_key).await? {
            if let Some(stored_ip) = &session.ip_address {
                if stored_ip == ip_address {
                    return Ok(Some(session.clone()));
                }
            }
        }
        
        Ok(None)
    }

    pub async fn update_session_access_time(&self, session_key: &str) -> Result<(), Box<dyn Error>> {
        let now = chrono::Utc::now();
        
        sqlx::query("UPDATE sessions SET last_accessed_at = ? WHERE session_key = ?")
            .bind(now)
            .bind(session_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<Session>, Box<dyn Error>> {
        let sessions = query_as::<_, Session>(
            "SELECT id, user_id, session_key, ip_address, user_agent, created_at, last_accessed_at FROM sessions WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    pub async fn delete_session(&self, session_key: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM sessions WHERE session_key = ?")
            .bind(session_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_user_sessions(&self, user_id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM sessions WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}