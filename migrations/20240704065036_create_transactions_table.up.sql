-- Add up migration script here
CREATE TYPE transaction_type AS ENUM('debit', 'credit');
CREATE TABLE IF NOT EXISTS transactions (
    id serial NOT NULL PRIMARY KEY,
    ttype transaction_type NOT NULL,
    amt float(4),
    user_id integer REFERENCES users
);