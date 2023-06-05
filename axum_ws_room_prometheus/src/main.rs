use {
    axum::{routing::get, Router, Server},
    axum_ws_room_prometheus::{ws_handler, ChatState},
    std::env::set_var,
    std::net::SocketAddr,
};

#[tokio::main]
async fn main() {
    // Initialize logging
    {
        set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let share_state = ChatState::default();
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(share_state);
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
