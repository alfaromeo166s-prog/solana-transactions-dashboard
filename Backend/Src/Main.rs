use axum::{Router, routing::get, Json};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct Transaction {
    signature: String,
    timestamp: u64,
    status: String,
    authority: String,
    instruction: String,
    n: u32,
}

async fn get_transactions() -> Json<Vec<Transaction>> {
    let txs = vec![
        Transaction {
            signature: "5gNf...".into(),
            timestamp: 1699999999,
            status: "confirmed".into(),
            authority: "Fv9g...".into(),
            instruction: "do_fizzbuzz".into(),
            n: 42,
        }
    ];
    Json(txs)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/transactions", get(get_transactions));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
