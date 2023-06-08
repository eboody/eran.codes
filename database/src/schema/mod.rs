use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
#[allow(unused)]
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
    pub fn new(name: &str, category: Category) -> Self {
        TechnologyBuilder {
            name: name.to_owned(),
            category,
        }
    }
    pub async fn build(&self, pool: &PgPool) -> Result<Technology, sqlx::Error> {
        let technology: Technology = sqlx::query_as(
            r#"INSERT INTO technologies (name, category) VALUES ($1, $2) RETURNING *"#,
        )
        .bind(&self.name)
        .bind(&self.category)
        .fetch_one(pool)
        .await?;

        Ok(technology)
    }
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "category")]
pub enum Category {
    Language,
    Backend,
    Frontend,
    Database,
    Tool,
}
