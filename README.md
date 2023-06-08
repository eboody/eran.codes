# Eran's Online Store API
This is a RESTful API for an online store, including customers, orders, and products. It provides endpoints for managing these entities and their relationships.

## Structure
This project is divided into two main directories:

- database: This contains the SQL migrations and Rust models for the customer, order, and product entities in our database.
- http_server: This is our HTTP server implemented with the Axum framework.

## Database
Inside database/src/schema, there are modules for each of our entities. Each module provides a builder using the typestate pattern, ensuring that the entities are constructed in a valid state.

## HTTP Server
The HTTP server in http_server/src implements the following endpoints:

### Customers
- GET /customers: Retrieve a list of all customers.
- GET /customers/{id}: Retrieve a specific customer by their ID.
- POST /customers: Create a new customer.
- PUT /customers/{id}: Update a specific customer by their ID.
- DELETE /customers/{id}: Delete a specific customer by their ID.
### Orders
- GET /orders: Retrieve a list of all orders.
- GET /orders/{id}: Retrieve a specific order by its ID.
- POST /orders: Create a new order.
- PUT /orders/{id}: Update a specific order by its ID.
- DELETE /orders/{id}: Delete a specific order by its ID.
- GET /customers/{id}/orders: Retrieve a list of all orders for a specific customer.
- POST /customers/{id}/orders: Create a new order for a specific customer.
### Products
- GET /products: Retrieve a list of all products.
- GET /products/{id}: Retrieve a specific product by its ID.
- POST /products: Create a new product.
- PUT /products/{id}: Update a specific product by its ID.
- DELETE /products/{id}: Delete a specific product by its ID.
- GET /orders/{id}/products: Retrieve a list of all products in a specific order.
- POST /orders/{id}/products: Add a product to a specific order.
- DELETE /orders/{id}/products/{id}: Remove a product from a specific order.
- Running the Project
- To run the project, you need to have Rust and Docker installed. You can start the project using Docker Compose:

```bash
docker-compose up
```

This will start the database and the HTTP server.

## Testing
To run tests, use:

```bash
cargo test
```

Please note that the tests assume that a PostgreSQL server is running and that the DATABASE_URL environment variable is set to point to it.

## Contributing
Contributions are welcome! Please feel free to open an issue or pull request.
