CREATE TYPE category AS ENUM ('Language', 'Backend', 'Frontend', 'Database', 'Tool');

CREATE TABLE technologies (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    category category NOT NULL,
    UNIQUE(name, category)
);

CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10,2) NOT NULL
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    customer_id INT REFERENCES Customers(id) NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE order_products (
    order_id INT REFERENCES Orders(id) NOT NULL,
    product_id INT REFERENCES Products(id) NOT NULL,
    quantity INT NOT NULL,
    PRIMARY KEY(order_id, product_id)
);
