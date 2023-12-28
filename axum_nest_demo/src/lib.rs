use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub enum Mode {
    Test,
    Prod,
}
pub struct App {
    pub router: Router,
}

impl App {
    pub fn new(mode: Mode) -> Self {
        let router = Router::new().route("/ping", get(ping));

        let router = match mode {
            Mode::Test => router.nest("/admin", admin_routes()),
            _ => router,
        };

        Self { router }
    }
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

#[cfg(test)]
mod tests {
    use super::{App, Mode};
    use axum_test::TestServer;

    fn test_app_new() -> App {
        App::new(Mode::Test)
    }

    fn prod_app_new() -> App {
        App::new(Mode::Prod)
    }

    #[tokio::test]
    async fn test_ping() {
        let test_app = test_app_new();
        let server = TestServer::new(test_app.router).unwrap();
        let response = server.get("/ping").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_admin_dev() {
        let test_app = test_app_new();
        let server = TestServer::new(test_app.router).unwrap();
        let response = server.get("/admin/keys").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn test_admin_prod() {
        let test_app = prod_app_new();
        let server = TestServer::new(test_app.router).unwrap();
        let response = server.get("/admin/keys").await;
        response.assert_status_not_found();
    }
}
