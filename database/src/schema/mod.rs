#![allow(unused)]

use sqlx::{postgres::PgQueryResult, FromRow, PgPool, Row};
use std::error::Error;
use strum::Display;
use strum_macros::EnumString;

#[derive(FromRow)]
pub struct Technology {
    id: i32,
    name: String,
    category: Category,
}

pub struct TechnologyBuilder {
    name: String,
    category: Category,
}

impl TechnologyBuilder {
    fn new(name: &str, category: Category) -> Self {
        TechnologyBuilder {
            name: name.to_owned(),
            category,
        }
    }
    async fn build(&self, pool: &PgPool) -> Result<Technology, Box<dyn Error>> {
        let query = "INSERT INTO technologies (name, category) VALUES ($1, $2) RETURNING *";

        let technology: Technology = sqlx::query_as(query)
            .bind(&self.name)
            .bind(&self.category)
            .fetch_one(pool)
            .await?;

        Ok(technology)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "category", rename_all = "lowercase")]
pub enum Category {
    Language,
    Backend,
    Frontend,
    Database,
    Tool,
}
