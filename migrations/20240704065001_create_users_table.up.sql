-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id serial NOT NULL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    pwd VARCHAR(255) NOT NULL,
    balance float(4) NOT NULL
);