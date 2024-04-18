CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    image_name VARCHAR(255) NOT NULL,
    image_data BYTEA NOT NULL
);