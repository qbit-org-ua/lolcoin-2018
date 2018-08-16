use warp::Filter;

pub fn serve() {
    use crate::common::{Coins, UserId};
    use crate::database::{TransfersCollection, UsersCollection, DB};
    use uuid::Uuid;

    let users = UsersCollection::load_from_file("./users.json").unwrap();
    let transfers = TransfersCollection::load_from_file("./transfers.json.log").unwrap();
    let db = std::sync::Arc::new(DB::new(users, transfers).unwrap());
    let db = warp::any().map(move || db.clone());

    let text = warp::path("plaintext").map(|| "Hello, World!");
    let users = warp::path("users")
        .and(db.clone())
        .map(|db: std::sync::Arc<DB>| warp::reply::json(&db.get_users().in_memory_storage));
    let transfers_index = warp::path("transfers").and(warp::path::index());
    let transfers = warp::get2()
        .and(transfers_index)
        .and(db.clone())
        .map(|db: std::sync::Arc<DB>| warp::reply::json(&db.get_transfers()));
    let send_transfer =
        warp::post2()
            .and(transfers_index)
            .and(db.clone())
            .map(|db: std::sync::Arc<DB>| {
                let res = db.transfer(
                    UserId::new(Uuid::parse_str("384fc6ad-0556-4301-ba3b-757c9ad29423").unwrap()),
                    UserId::new(Uuid::parse_str("384fc6ad-0556-4301-ba3b-757c9ad29424").unwrap()),
                    Coins::new(1),
                );
                println!("RES: {:#?}", res);
                warp::reply::json(&["ok"])
            });
    let routes = text.or(users).or(transfers).or(send_transfer);

    warp::serve(routes)
        .unstable_pipeline()
        .run(([127, 0, 0, 1], 3030));
}
