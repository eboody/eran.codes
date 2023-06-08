use sqlx::{types::BigDecimal, PgPool};
use std::{marker::PhantomData, sync::Arc};

/// Struct to represent a Product with id, name and price
#[allow(unused)]
#[derive(Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: BigDecimal,
}

// States

/// Struct to represent Name state
#[derive(Default, Clone)]
pub struct ProductName(String);

/// Struct to represent absence of Name
#[derive(Default, Clone)]
pub struct NoProductName;

/// Struct to represent Price state
#[derive(Default, Clone)]
pub struct Price(BigDecimal);

/// Struct to represent absence of Price
#[derive(Default, Clone)]
pub struct NoPrice;

/// Struct to represent Sealed state
#[derive(Default, Clone)]
pub struct Sealed;

/// Struct to represent Not Sealed state
#[derive(Default, Clone)]
pub struct NotSealed;

// End of States

/// Builder pattern for Product
#[derive(Default, Clone)]
pub struct ProductBuilder<N, P, S> {
    name: Option<N>,
    price: Option<P>,
    pg_pool: Option<Arc<PgPool>>,
    seal_marker: PhantomData<S>,
}

impl ProductBuilder<NoProductName, NoPrice, NotSealed> {
    /// Create new ProductBuilder
    pub fn new(pg_pool: Arc<PgPool>) -> ProductBuilder<NoProductName, NoPrice, NotSealed> {
        ProductBuilder {
            pg_pool: Some(pg_pool),
            ..ProductBuilder::default()
        }
    }
}

impl<N, P> ProductBuilder<N, P, NotSealed> {
    /// Finalize the state of ProductBuilder
    pub fn seal(self) -> ProductBuilder<N, P, Sealed> {
        ProductBuilder {
            name: self.name,
            price: self.price,
            pg_pool: self.pg_pool,
            seal_marker: PhantomData,
        }
    }
}

impl ProductBuilder<ProductName, Price, Sealed> {
    /// Build the Product and save to database
    pub async fn build(self) -> Result<Product, sqlx::Error> {
        sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (name, price) 
            VALUES ($1, $2)
            RETURNING id, name, price;
        "#,
            self.name.unwrap().0,
            self.price.unwrap().0
        )
        .fetch_one(self.pg_pool.unwrap().as_ref())
        .await
    }
}

impl<N, P> ProductBuilder<N, P, NotSealed> {
    /// Set the name of the Product
    pub fn name(self, name: &str) -> ProductBuilder<ProductName, P, NotSealed> {
        ProductBuilder {
            name: Some(ProductName(name.to_owned())),
            price: self.price,
            pg_pool: self.pg_pool,
            seal_marker: self.seal_marker,
        }
    }

    /// Set the price of the Product
    pub fn price(self, price: BigDecimal) -> ProductBuilder<N, Price, NotSealed> {
        ProductBuilder {
            price: Some(Price(price)),
            name: self.name,
            pg_pool: self.pg_pool,
            seal_marker: self.seal_marker,
        }
    }
}
