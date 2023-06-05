use {
    crate::version::VERSION as VERSION_INFO,
    axum::debug_handler,
    hyper::StatusCode,
    prometheus::{IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry, TextEncoder},
    std::sync::Once,
    tracing::error,
};

lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    static ref PROXY_VERSION: IntCounterVec = IntCounterVec::new(
        Opts::new("proxy_version", "Proxy version info"),
        &["buildts", "git", "rustc", "solana", "version"]
    ).unwrap();

    pub static ref OPEN_CONNECTIONS: IntGauge = IntGauge::new(
        "ws_open_connection", "WebSocket open connections").unwrap();

    pub static ref ROOM_PEOPLES: IntGaugeVec = IntGaugeVec::new(
        Opts::new("room_peoples", "number of people in a room"),
        &["room"]
    ).unwrap();

    pub static ref GRPC_CONNECTED: IntGauge = IntGauge::new(
        "grpc_connected", "gRPC connected status").unwrap();

}

pub fn init() {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        macro_rules! register {
            ($collector:ident) => {
                REGISTRY
                    .register(Box::new($collector.clone()))
                    .expect("collector can't be registered");
            };
        }
        register!(OPEN_CONNECTIONS);
        register!(ROOM_PEOPLES);
        register!(GRPC_CONNECTED);
        register!(PROXY_VERSION);

        PROXY_VERSION
            .with_label_values(&[
                VERSION_INFO.buildts,
                VERSION_INFO.git,
                VERSION_INFO.rustc,
                VERSION_INFO.solana,
                VERSION_INFO.version,
            ])
            .inc()
    });
}

#[debug_handler]
pub async fn metrics_as_http_response() -> Result<String, StatusCode> {
    let metrics = TextEncoder::new()
        .encode_to_string(&REGISTRY.gather())
        .map_err(|error| {
            error!("could not encode custom metrics: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(metrics)
}
