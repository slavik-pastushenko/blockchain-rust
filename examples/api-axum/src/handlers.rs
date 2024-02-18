use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use blockchain::Chain;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The application state.
#[derive(Clone)]
pub struct AppState {
    /// The blockchain.
    pub chain: Arc<Mutex<Chain>>,
}

/// Create a new wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletInput {
    /// The wallet email.
    pub email: String,
}

/// Add a new transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddTransactionInput {
    /// The sender address.
    pub from: String,

    /// The receiver address.
    pub to: String,

    /// The transaction amount.
    pub amount: f64,
}

/// Get the balance of a wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletBalanceInput {
    /// The wallet address.
    pub address: String,
}

/// Get a list of transactions of a wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWalletTransactionInput {
    /// The wallet address.
    pub address: String,

    /// The page number.
    pub page: usize,

    /// The page size.
    pub size: usize,
}

/// Get a list of transactions of a wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsInput {
    /// The page number.
    pub page: usize,

    /// The page size.
    pub size: usize,
}

/// Create a new wallet.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `body` - The request body.
///
/// # Returns
///
/// A new wallet address.
pub async fn create_wallet(
    State(state): State<AppState>,
    Json(body): Json<CreateWalletInput>,
) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();
    let address = chain.create_wallet(body.email);

    (StatusCode::OK, Json(json!({ "data": address })))
}

/// Get the balance of a wallet.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `params` - The request query parameters.
///
/// # Returns
///
/// The balance of the wallet.
pub async fn get_wallet_balance(
    State(state): State<AppState>,
    Query(params): Query<GetWalletBalanceInput>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let balance = chain.get_wallet_balance(params.address);

    match balance {
        Some(balance) => (StatusCode::OK, Json(json!({ "data": balance }))),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Wallet is not found" })),
        ),
    }
}

/// Get a list of transactions of a wallet.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `params` - The request query parameters.
///
/// # Returns
///
/// The list of transactions of the wallet.
pub async fn get_wallet_transactions(
    State(state): State<AppState>,
    Query(params): Query<GetWalletTransactionInput>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let transaction = chain.get_wallet_transactions(params.address, params.page, params.size);

    match transaction {
        Some(transaction) => (StatusCode::OK, Json(json!({ "data": transaction }))),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Wallet is not found" })),
        ),
    }
}

/// Get all transactions.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `params` - The request query parameters.
///
/// # Returns
///
/// All transactions.
pub async fn get_transactions(
    State(state): State<AppState>,
    Query(params): Query<GetTransactionsInput>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let transactions = chain.get_transactions(params.page, params.size);

    (StatusCode::OK, Json(json!({ "data": transactions })))
}

/// Get a transaction.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `hash` - The transaction hash.
///
/// # Returns
///
/// The transaction.
pub async fn get_transaction(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    let chain = state.chain.lock().unwrap();
    let transaction = chain.get_transaction(hash);

    match transaction {
        Some(transaction) => (StatusCode::OK, Json(json!({ "data": transaction }))),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Transaction is not found" })),
        ),
    }
}

/// Add a new transaction.
///
/// # Arguments
///
/// - `state` - The application state.
/// - `body` - The request body.
///
/// # Returns
///
/// The new transaction.
pub async fn add_transaction(
    State(state): State<AppState>,
    Json(body): Json<AddTransactionInput>,
) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();

    let result = chain.add_transaction(body.from, body.to, body.amount);

    (StatusCode::OK, Json(json!({ "data": result })))
}
