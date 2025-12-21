pub trait Repository {
    fn get_by_id(&self, id: &UserId) -> Result<Option<User>, RepoError>;
    fn get_by_email(&self, email: &Email) -> Result<Option<User>, RepoError>;
    fn save(&self, user: &User) -> Result<(), RepoError>;
}
