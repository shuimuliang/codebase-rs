use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use axum::{
    routing::{get, post},
    Router,
    response::Json,
    extract::State,
    body::Bytes,
    http::StatusCode,
};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use axum_macros::debug_handler;

#[derive(Default, Clone)]
struct AppState {
    db: HashMap<String, Bytes>,
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let share_state = SharedState::default();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/root", get(root))
        .route("/json", get(json))
        .route("/create_todo", get(create_todo))
        .route("/login", post(login))
        .route("/get", post(get_value_handler))
        .route("/set", post(set_value_handler))
    .with_state(share_state);

    // run it with hyper on localhost:3000
    let addr :SocketAddr = "0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() {}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn create_todo() -> StatusCode {
    StatusCode::CREATED
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    id: i32,
}

async fn login(Json(req): Json<LoginRequest>) -> Json<LoginResponse> {
    Json(
        LoginResponse {
            id: req.id + 1
        }
    )
}

#[derive(Serialize, Deserialize)]
struct RedisGet {
    key: String,
}

#[derive(Serialize, Deserialize)]
struct RedisSet {
    key: String,
    value: String,
}

#[debug_handler]
async fn get_value_handler(
    State(state): State<SharedState>,
    Json(req): Json<RedisGet>
) -> Result<Bytes, StatusCode> {
    let db = &state.read().unwrap().db;
    if let Some(value) = db.get(&req.key) {
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[debug_handler]
async fn set_value_handler(
    State(state): State<SharedState>,
    Json(req): Json<RedisSet>,
) {
    let db = &mut state.write().unwrap().db;
    db.insert(req.key, Bytes::copy_from_slice(req.value.as_bytes()));
}
