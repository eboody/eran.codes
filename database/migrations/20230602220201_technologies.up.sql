CREATE TYPE category AS ENUM ('Language', 'Backend', 'Frontend', 'Database', 'Tool');

CREATE TABLE technologies (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    category category NOT NULL,
    UNIQUE(name, category)
);

