use sqlx::PgPool;
use std::{marker::PhantomData, sync::Arc};

/// Struct to represent a Customer with id, name and email
#[allow(unused)]
#[derive(Debug)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// States

/// Struct to represent Name state
#[derive(Default, Clone)]
pub struct Name(String);

/// Struct to represent absence of Name
#[derive(Default, Clone)]
pub struct NoName;

/// Struct to represent Email state
#[derive(Default, Clone)]
pub struct Email(String);

/// Struct to represent absence of Email
#[derive(Default, Clone)]
pub struct NoEmail;

/// Struct to represent Sealed state
#[derive(Default, Clone)]
pub struct Sealed;

/// Struct to represent Not Sealed state
#[derive(Default, Clone)]
pub struct NotSealed;

// End of States

/// Builder pattern for Customer
#[derive(Default, Clone)]
pub struct CustomerBuilder<N, E, S> {
    name: Option<N>,
    email: Option<E>,
    pg_pool: Option<Arc<PgPool>>,
    seal_marker: PhantomData<S>,
}

impl CustomerBuilder<NoName, NoEmail, NotSealed> {
    /// Create new CustomerBuilder
    pub fn new(pg_pool: Arc<PgPool>) -> CustomerBuilder<NoEmail, NoEmail, NotSealed> {
        CustomerBuilder {
            pg_pool: Some(pg_pool),
            ..CustomerBuilder::default()
        }
    }
}

impl<N, E> CustomerBuilder<N, E, NotSealed> {
    /// Finalize the state of CustomerBuilder
    pub fn seal(self) -> CustomerBuilder<N, E, Sealed> {
        CustomerBuilder {
            name: self.name,
            email: self.email,
            pg_pool: self.pg_pool,
            seal_marker: PhantomData,
        }
    }
}

impl CustomerBuilder<Name, Email, Sealed> {
    /// Build the Customer and save to database
    pub async fn build(self) -> Result<Customer, sqlx::Error> {
        sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers (name, email) 
            VALUES ($1, $2)
            RETURNING id, name, email;
        "#,
            self.name.unwrap().0,
            self.email.unwrap().0
        )
        .fetch_one(self.pg_pool.unwrap().as_ref())
        .await
    }
}

impl<N, E> CustomerBuilder<N, E, NotSealed> {
    /// Set the name of the Customer
    pub fn name(self, name: &str) -> CustomerBuilder<Name, E, NotSealed> {
        CustomerBuilder {
            name: Some(Name(name.to_owned())),
            email: self.email,
            pg_pool: self.pg_pool,
            seal_marker: self.seal_marker,
        }
    }

    /// Set the email of the Customer
    pub fn email(self, email: &str) -> CustomerBuilder<N, Email, NotSealed> {
        CustomerBuilder {
            email: Some(Email(email.to_owned())),
            name: self.name,
            pg_pool: self.pg_pool,
            seal_marker: self.seal_marker,
        }
    }
}
