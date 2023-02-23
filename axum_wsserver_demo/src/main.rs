use axum::{routing::get, Router};

use axum_wsserver_demo::ws_handler;
use log::info;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let app = Router::new().route("/", get(ws_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("server run at {:?}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
