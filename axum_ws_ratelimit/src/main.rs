use {
    axum::{
        extract::{
            connect_info::ConnectInfo,
            ws::{Message, WebSocket},
            State, WebSocketUpgrade,
        },
        response::IntoResponse,
        routing::{get, Router},
    },
    axum_ws_ratelimit::state::{AppState, ShareState},
    log::debug,
    std::{
        env::set_var,
        net::SocketAddr,
        sync::{Arc, RwLock},
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    let share_state = Arc::new(RwLock::new(AppState::new()));

    let app = Router::new()
        .route("/", get(ws_handler))
        .with_state(share_state);
    let addr = "127.0.0.1:8080".to_owned().parse::<SocketAddr>()?;

    debug!("{:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(_who): ConnectInfo<SocketAddr>,
    State(_state): State<ShareState>,
) -> impl IntoResponse {
    // global check
    ws.on_upgrade(|websocket| async move { handle_websocket(websocket).await })
}

// Define the function to handle WebSocket connections
async fn handle_websocket(mut websocket: WebSocket) {
    while let Some(result) = websocket.recv().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(_) => {
                    websocket
                        .send(Message::Text("hi".to_owned()))
                        .await
                        .unwrap();
                }
                Message::Close(_c) => {}
                _ => {}
            },
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        }
    }
    ()
}
