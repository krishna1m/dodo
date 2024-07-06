# Dodo
Simple backend service for managing transactions and user accounts.

## Prerequisites
1. Make sure you have `cargo` installed.
2. Make sure you have PostgreSQL installed and it is up and running. Try using PostgreSQL 15 or later.
3. You can optionally use `psql` to connect to the postgresql database.

## How to run
1. Create a database with the name `dodo_payments`.
```bash
$ psql
user=# create database dodo_payments;
```
2. Start the application. It starts at port 3030.
```bash
$ cargo run
```
3. Use the APIs listed in the next section via `curl` or a tool of your choice.

## APIs and Features
The application provides the following features.
1. User Management
### Registration
POST `/register`
```json
{
    "email": "test@email.com",
    "password": "testpass"
}
```
Response
```text
User Added
```
### Login/Authentication
POST `/login`
```json
{
    "email": "test@email.com",
    "password": "testpass"
}
```
Response
```text
"<authorization token>"
```
2. Transaction Management
### Credit amount
POST `/credit`
```
Content-Type: application/json
Authorization: <token>
```
Request
```json
{
    "amount": 4.32
}
```
Response
```json
{
    "ttype": "Credit",
    "amt": 4.32
}
```
### Debit amount
POST `/debit`
```
Content-Type: application/json
Authorization: <token>
```
Request
```json
{
    "amount": 1.50
}
```
Response
```json
{
    "ttype": "Debit",
    "amt": 1.50
}
```
### List transactions
3. Account Balances
GET `/transactions`
```
Authorization: <token>
```
Response
```json
[
    {
        "id": 2,
        "ttype": "Debit",
        "amt": 1.50
    },
    {
        "id": 1,
        "ttype": "Credit",
        "amt": 4.32
    }
]
```
### Get balance
GET `/balance`
```
Authorization: <token>
```
Response
```json
{
    "amount": 2.82
}
```