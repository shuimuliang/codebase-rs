use axum::{extract::Path, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/admin", admin_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}
fn admin_routes() -> Router {
    async fn keys() -> impl IntoResponse {
        "keys"
    }

    async fn key(Path(key): Path<String>) -> impl IntoResponse {
        key
    }

    Router::new()
        .route("/keys", get(keys))
        .route("/key/:key", get(key))
}
