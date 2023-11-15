use crate::handlers::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use blockchain::Chain;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod handlers;

#[tokio::main]
async fn main() {
    let chain = Chain::new(2.0, 100.0, 0.01);

    let state = AppState {
        chain: Arc::new(Mutex::new(chain)),
    };

    let app = Router::new()
        .route("/transactions/:hash", get(handlers::get_transaction))
        .route("/transactions", get(handlers::get_transactions))
        .route("/transactions", post(handlers::add_transaction))
        .route("/wallet/balance", get(handlers::get_wallet_balance))
        .route("/wallet/create", post(handlers::create_wallet))
        .with_state(state);

    let address = SocketAddr::from(([0, 0, 0, 0], 7878));

    println!("Server is running on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
