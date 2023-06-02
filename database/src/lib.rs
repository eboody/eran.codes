use dotenvy::dotenv;
use sqlx::{PgPool, Result};

#[allow(unused)]
struct Database {
    pool: PgPool,
}

impl Database {
    #[allow(unused)]
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let pool = PgPool::connect(&connection_string).await?;

        Ok(Database { pool })
    }

    #[allow(unused)]
    pub async fn test(&self, some_string: &str) -> Result<String, sqlx::error::Error> {
        // Make a simple query to return the given parameter
        let row: (String,) = sqlx::query_as("SELECT $1")
            .bind(some_string)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}

#[cfg(test)]
mod test {
    use super::Database;

    #[tokio::test]
    async fn db_working() {
        let database = Database::new().await.unwrap();

        let some_string = "OK";

        let response = database.test(some_string).await.unwrap();
        println!(
            "->>DEBUG<<-{}:{} -> response = {:?}",
            file!().to_owned(),
            line!(),
            response
        );

        assert_eq!(response, some_string)
    }
}
