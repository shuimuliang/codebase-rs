use axum_nest_demo::{App, Mode};

#[tokio::main]
async fn main() {
    let app = App::new(Mode::Prod);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app.router).await.unwrap();
}
