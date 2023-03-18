use {
    axum::{
        extract::{
            connect_info::ConnectInfo,
            ws::{CloseFrame, Message, WebSocket},
            State, WebSocketUpgrade,
        },
        response::IntoResponse,
        routing::{get, Router},
    },
    axum_ws_ratelimit::state::{AppState, ShareState},
    log::debug,
    std::{borrow::Cow, env::set_var, net::SocketAddr, sync::Arc},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    let share_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(ws_handler))
        .with_state(share_state);
    let addr = "127.0.0.1:8080".to_owned().parse::<SocketAddr>()?;

    debug!("listen to: {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(who): ConnectInfo<SocketAddr>,
    State(state): State<ShareState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |websocket| handle_websocket(websocket, who, state))
}

// Define the function to handle WebSocket connections
async fn handle_websocket(mut websocket: WebSocket, who: SocketAddr, state: ShareState) {
    // check client connection count by IpAddr
    if state.conn_exceed_limit(&who.ip()) {
        debug!("{} is_exceed.", who);
        if let Err(e) = websocket
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            debug!("Could not send Close due to {}, probably it is ok?", e);
            return;
        }
    }

    state.add_conn(who.ip());

    while let Some(result) = websocket.recv().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(_) => {
                    if let Err(e) = websocket.send(Message::Text("hi".to_owned())).await {
                        debug!("Could not send Close due to {}, probably it is ok?", e);
                        return;
                    }
                }
                Message::Close(_c) => {
                    // decrease connection count
                    state.remove_conn(who.ip());
                    debug!("client close connection {}", who);
                }
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