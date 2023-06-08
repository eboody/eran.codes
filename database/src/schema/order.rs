use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;
use std::{marker::PhantomData, sync::Arc};

/// Struct to represent an Order with id, customer_id, and created_at
#[allow(unused)]
#[derive(Debug)]
pub struct Order {
    id: i32,
    customer_id: i32,
    created_at: NaiveDateTime,
}

// States

/// Struct to represent CustomerId state
#[derive(Default, Clone)]
pub struct CustomerId(i32);

/// Struct to represent absence of CustomerId
#[derive(Default, Clone)]
pub struct NoCustomerId;

/// Struct to represent Sealed state
#[derive(Default, Clone)]
pub struct Sealed;

/// Struct to represent Not Sealed state
#[derive(Default, Clone)]
pub struct NotSealed;

// End of States

/// Builder pattern for Order
#[derive(Default, Clone)]
pub struct OrderBuilder<C, S> {
    customer_id: Option<C>,
    pg_pool: Option<Arc<PgPool>>,
    seal_marker: PhantomData<S>,
}

impl OrderBuilder<NoCustomerId, NotSealed> {
    /// Create new OrderBuilder
    pub fn new(pg_pool: Arc<PgPool>) -> OrderBuilder<NoCustomerId, NotSealed> {
        OrderBuilder {
            pg_pool: Some(pg_pool),
            ..OrderBuilder::default()
        }
    }
}

impl<C> OrderBuilder<C, NotSealed> {
    /// Finalize the state of OrderBuilder
    pub fn seal(self) -> OrderBuilder<C, Sealed> {
        OrderBuilder {
            customer_id: self.customer_id,
            pg_pool: self.pg_pool,
            seal_marker: PhantomData,
        }
    }
}

impl OrderBuilder<CustomerId, Sealed> {
    /// Build the Order and save to database
    pub async fn build(self) -> Result<Order, sqlx::Error> {
        let now = Utc::now().naive_utc();

        let customer_id = self.customer_id.unwrap().0;

        sqlx::query_as!(
            Order,
            r#"
                INSERT INTO orders (customer_id, created_at)
                VALUES ($1, $2)
                RETURNING id, customer_id, created_at;
            "#,
            customer_id,
            now
        )
        .fetch_one(self.pg_pool.unwrap().as_ref())
        .await
    }
}

impl<C> OrderBuilder<C, NotSealed> {
    /// Set the customer_id of the Order
    pub fn customer_id(self, customer_id: i32) -> OrderBuilder<CustomerId, NotSealed> {
        OrderBuilder {
            customer_id: Some(CustomerId(customer_id)),
            pg_pool: self.pg_pool,
            seal_marker: self.seal_marker,
        }
    }
}
