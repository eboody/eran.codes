use dotenvy::dotenv;
use sqlx::postgres::PgPool;

use database::schema::{Category, TechnologyBuilder};

const NUMBER_OF_ROWS: i32 = 10;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let connection_string =
        std::env::var("DATABASE_URL").expect("database connection string not present as env var");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let ids: Vec<i32> = (1..=NUMBER_OF_ROWS).collect();

    for id in ids {
        let tech_name = format!("Technology{}", id);
        let tech_category = match id % 5 {
            0 => Category::Language,
            1 => Category::Backend,
            2 => Category::Frontend,
            3 => Category::Database,
            _ => Category::Tool,
        };

        let tech_builder = TechnologyBuilder::new(&tech_name, tech_category);
        let tech = tech_builder.build(&pool).await;

        println!("{tech:#?}");
    }

    Ok(())
}
