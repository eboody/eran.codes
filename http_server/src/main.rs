use axum::{routing::get, Router, Server};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

use database::{
    schema::{customer::CustomerBuilder, order::OrderBuilder, TechnologyBuilder},
    Database,
};

use dotenvy::dotenv;

struct AppState {
    database: Database,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = Database::new().await.expect("didnt get database client"); //TODO: handle more gracefully

    let app_state = Arc::new(AppState { database });

    let customer_builder = CustomerBuilder::new(app_state.database.get_pool());

    let customer = customer_builder
        .name("Eran")
        .email("eran@test.com")
        .seal()
        .build()
        .await;

    let customer_id = customer.unwrap().id;

    let order_builder = OrderBuilder::new(app_state.database.get_pool());

    let order = order_builder.customer_id(customer_id).seal().build().await;
    println!(
        "->>DEBUG<<-{}:{} -> order = {:#?}",
        file!().to_owned(),
        line!(),
        order
    );

    let socket = SocketAddr::from(([127, 0, 0, 1], 5001));

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .with_state(Arc::clone(&app_state));

    Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
