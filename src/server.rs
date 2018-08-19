use serde_json::json;
use uuid::Uuid;
use warp::Filter;

use crate::common::{UserId, UserSecret};
use crate::database::{TransfersCollection, User, UsersCollection, DB};
use crate::forms::TransferData;

pub fn serve() {
    let users = UsersCollection::load_from_file("./users.json").unwrap();
    let transfers = TransfersCollection::load_from_file("./transfers.json.log").unwrap();
    let db = std::sync::Arc::new(DB::new(users, transfers).unwrap());

    let db = warp::any().map(move || db.clone());
    let auth = warp::header("authorization").and(db.clone()).and_then(
        |authorization_token: String, db: std::sync::Arc<DB>| {
            db.get_user_by_secret(&UserSecret::new(authorization_token))
                .ok_or_else(|| warp::reject())
        },
    );
    let cors = warp::options()
        .map(|| "")
        .with(warp::reply::with::header("Access-Control-Allow-Headers", "authorization, content-type"));

    let users = warp::path("users")
        .and(db.clone())
        .map(|db: std::sync::Arc<DB>| warp::reply::json(&db.get_users().in_memory_storage));
    let user = warp::path("user")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(|query: std::collections::HashMap<String, String>| {
            query.get("user_id")
                .ok_or_else(|| warp::reject())
                .and_then(|user_id| {
                    Uuid::parse_str(user_id)
                        .or_else(|_| Err(warp::reject()))
                        .map(|user_id| UserId::new(user_id))
                })
        })
        .and(db.clone())
        .map(|user_id: UserId, db: std::sync::Arc<DB>| warp::reply::json(&db.get_user(user_id)));

    let transfers_index = warp::path("transfers")
        .and(warp::path::index())
        .and(db.clone());
    let transfers = warp::get2()
        .and(transfers_index.clone())
        .map(|db: std::sync::Arc<DB>| {
            let mut transfers = db.get_transfers();
            transfers.reverse();
            warp::reply::json(&transfers)
        });
    let send_transfer = warp::post2()
        .and(transfers_index.clone())
        .and(auth.clone())
        .and(warp::body::json())
        .map(
            |db: std::sync::Arc<DB>, user: User, transfer: TransferData| {
                let status = match db.transfer(user.get_user_id(), transfer.to, transfer.amount) {
                    Ok(()) => "ok",
                    Err(error) => {
                        eprintln!("Transfer failed due to {:?}", error);
                        "error"
                    }
                };
                warp::reply::json(&json!({ "status": status }))
            },
        );

    let routes =
        cors.with(warp::reply::with::default_header("Access-Control-Allow-Origin", "*"))
        .or(users.with(warp::reply::with::default_header("Access-Control-Allow-Origin", "*")))
        .or(user.with(warp::reply::with::default_header("Access-Control-Allow-Origin", "*")))
        .or(transfers.with(warp::reply::with::default_header("Access-Control-Allow-Origin", "*")))
        .or(send_transfer.with(warp::reply::with::default_header("Access-Control-Allow-Origin", "*")));

    warp::serve(routes)
        .unstable_pipeline()
        .run(([0, 0, 0, 0], 3030));
}
