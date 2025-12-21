use domain::user;

#[allow(unused)]
pub struct SqlxUserRepository {
    pg: sqlx::PgPool,
}

#[allow(unused)]
impl user::Repository for SqlxUserRepository {
    // Implement user repository methods here
    fn get_by_id(&self, id: &user::Id) -> domain::Result<Option<user::Model>> {
        todo!()
    }

    fn save(&self, user: &user::Model) -> domain::Result<()> {
        todo!()
    }

    fn get_by_email(&self, email: &user::Email) -> domain::Result<Option<user::Model>> {
        todo!()
    }
}
