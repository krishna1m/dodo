#![warn(clippy::all)]

use handle_errors::return_error;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    let store =
        store::Store::new("postgres://localhost:5432/dodo_payments").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot migrate DB");

    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::GET,
            Method::POST,
        ]);

    let get_transactions = warp::get()
        .and(warp::path("transactions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::user::get_transactions);


    let create_debit = warp::post()
        .and(warp::path("debit"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::user::debit);

    let create_credit = warp::post()
        .and(warp::path("credit"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::user::credit);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = get_transactions
        .or(create_debit)
        .or(create_credit)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}