use std::env;

use deadpool_postgres::{Manager, Pool};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use tokio_postgres::{Config, NoTls};

pub static DB_POOL: Lazy<DbPool> =
    Lazy::new(|| DbPool::new().expect("Failed to create database pool"));

#[derive(Clone)]
pub struct DbPool {
    pool: Pool,
}

impl DbPool {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Try parsing the URL first
        match database_url.parse::<Config>() {
            Ok(config) => {
                println!("Successfully parsed database URL");
                let manager = Manager::new(config, NoTls);
                match Pool::builder(manager).max_size(16).build() {
                    Ok(pool) => {
                        println!("Successfully created pool");
                        Ok(DbPool { pool })
                    }
                    Err(e) => {
                        println!("Failed to create pool: {:?}", e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("Failed to parse database URL: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn get_connection(
        &self,
    ) -> Result<deadpool_postgres::Client, deadpool_postgres::PoolError> {
        self.pool.get().await
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_integration() {
        let pool_result = DB_POOL.get_connection().await;
        match pool_result {
            Ok(client) => {
                println!("Successfully connected to database");
                let row = client.query_one("SELECT 1 + 1 as sum", &[]).await.unwrap();
                let sum: i32 = row.get("sum");
                assert_eq!(sum, 2);
            }
            Err(e) => {
                println!("Failed to connect to database: {:?}", e);
                panic!("Database connection failed: {:?}", e);
            }
        }
    }
}
