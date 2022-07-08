-- Create tables
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR,
    directions VARCHAR
);

CREATE TABLE measurements (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    recipe_id INT REFERENCES recipes(id) ON DELETE CASCADE,
--    TODO: Decide whether or not you should be able to delete products if there is an ingredient that relies on it.
--    product INT REFERENCES products(id) ON DELETE SET NULL,
    product_id INT REFERENCES products(id),
    amount FLOAT,
    measurement_id INT REFERENCES measurements(id)
);

-- Seed tables

-- NOTE: when seeding with serial ID, make sure not to assign the serial value, that will mess up the records.

-- products
INSERT INTO products (name) VALUES ('Banana');
INSERT INTO products (name) VALUES ('Apple');
INSERT INTO products (name) VALUES ('Bread');
INSERT INTO products (name) VALUES ('Oats');
INSERT INTO products (name) VALUES ('Spaghetti');

---- recipes
INSERT INTO recipes (name, description, directions) VALUES ('Banana Bread', 'Bread made out of bananas, bro.', 'not sure how to do this yet');

-- measurements
--INSERT INTO measurements VALUES (1, 'cups');
--
---- ingredients
--INSERT INTO ingredients VALUES (
--    -- id
--    1,
--    -- recipe id
--    1,
--    -- product id
--    1,
--    -- amount
--    1.0,
--    -- measurement id
--    1);
