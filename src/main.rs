// Load config
//
// Create DB pool
//
// Create repositories
//
// Create services
//
// Create app state
//
// Call http::router(app_state)
//
// Bind to address
//
// Serve forever
mod app_state;
mod config;
mod error;

#[tokio::main]
async fn main() {
    // app_config instantiates the entire configuration for the application's components
    let app_config = config::AppConfig::load().expect("Failed to load config");

    // app_state holds the initialized state of the application like repositories and
    // services
    let app_state = app_state::AppState::init(app_config.clone())
        .await
        .expect("Failed to initialize app state");

    // // Create repositories
    // let user_repo = UserRepository::new(db_pool.clone());
    // let product_repo = ProductRepository::new(db_pool.clone());
    //
    // // Create services
    // let user_service = UserService::new(user_repo);
    // let product_service = ProductService::new(product_repo);
    //
    // // Create app state
    // let app_state = AppState {
    //     user_service,
    //     product_service,
    //     config,
    // };
    //
    // // Call http::router(app_state)
    // let router = http::router(app_state);
    //
    // // Bind to address
    // let addr = format!("{}:{}", app_state.config.host, app_state.config.port);
    // let listener = tokio::net::TcpListener::bind(&addr)
    //     .await
    //     .expect("Failed to bind to address");
    //
    // println!("Server running on {}", addr);
    //
    // // Serve forever
    // axum::serve(listener, router)
    //     .await
    //     .expect("Failed to serve");
}
