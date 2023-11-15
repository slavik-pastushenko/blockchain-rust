use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use blockchain::Chain;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub chain: Arc<Mutex<Chain>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWallet {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTransaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalance {
    pub address: String,
}

pub async fn create_wallet(
    State(state): State<AppState>,
    Json(body): Json<CreateWallet>,
) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();
    let address = chain.create_wallet(body.email);

    (StatusCode::OK, Json(json!({ "data": address })))
}

pub async fn get_wallet_balance(
    State(state): State<AppState>,
    Query(params): Query<GetWalletBalance>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let balance = chain.get_wallet_balance(params.address);

    if balance.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Wallet is not found" })),
        );
    }

    (StatusCode::OK, Json(json!({ "data": balance })))
}

pub async fn get_transactions(State(state): State<AppState>) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();
    let transactions = chain.get_transactions();

    (StatusCode::OK, Json(json!({ "data": transactions })))
}

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let transaction = chain.get_transaction(hash);

    if transaction.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Transaction is not found" })),
        );
    }

    (StatusCode::OK, Json(json!({ "data": transaction })))
}

pub async fn add_transaction(
    State(state): State<AppState>,
    Json(body): Json<AddTransaction>,
) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();

    let result = chain.add_transaction(body.from, body.to, body.amount);

    (StatusCode::OK, Json(json!({ "data": result })))
}
