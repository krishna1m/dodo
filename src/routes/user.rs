use handle_errors::APILayerError;
use tracing::instrument;

use crate::store::Store;
use crate::types::amount::Amount;
use crate::types::user::Session;

#[instrument]
pub async fn get_transactions(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = session.user_id;
    match store
        .get_transactions(user_id)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_balance(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = session.user_id;
    match store
        .get_user_balance(&user_id)
        .await
        .map(|balance| Amount {
            amount: balance.0,
        })
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn debit(
    session: Session,
    store: Store,
    amt: Amount,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = session.user_id;
    let old_balance = store.get_user_balance(&user_id).await?;
    if old_balance.0 < amt.amount {
        Err(warp::reject::custom(handle_errors::Error::ClientError(
            APILayerError {
                status: 422,
                message: String::from("Your balance is too low to process the transaction"),
            }
        )))
    } else {
        let new_balance = old_balance.0 - amt.amount;
        let _ = store.update_user_balance(&user_id, new_balance).await;
        let action = store.add_debit(&user_id, amt.amount).await;
            
        match action {
            Ok(transaction) => Ok(warp::reply::json(&transaction)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    }
}

pub async fn credit(
    session: Session,
    store: Store,
    amt: Amount,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = session.user_id;
    let old_balance = store.get_user_balance(&user_id).await?;
    let new_balance = old_balance.0 + amt.amount;
    let _ = store.update_user_balance(&user_id, new_balance).await;
    let action = store.add_credit(&user_id, amt.amount).await;
        
    match action {
        Ok(transaction) => Ok(warp::reply::json(&transaction)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}