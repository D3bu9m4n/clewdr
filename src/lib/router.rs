use axum::{
    Router,
    http::HeaderMap,
    routing::{get, options, post},
};
use rquest::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};
use tower_http::services::ServeDir;

use crate::{
    api::{api_completion, api_messages, api_submit, api_version},
    state::ClientState,
};

/// RouterBuilder for the application
pub struct RouterBuilder {
    inner: Router,
}

impl RouterBuilder {
    /// Create a new RouterBuilder instance
    pub fn new(state: ClientState) -> Self {
        // Serve static files from "static" directory
        let static_service = ServeDir::new("static");

        let r = Router::new()
            .route("/v1", options(api_options))
            .route("/v1/messages", post(api_messages))
            .route("/api/submit", post(api_submit))
            .route("/api/version", get(api_version));
        let r = if state.config.enable_oai {
            r.route("/v1/chat/completions", post(api_completion))
        } else {
            r
        };
        let r = r.fallback_service(static_service).with_state(state);
        Self { inner: r }
    }

    /// return the inner router
    pub fn build(self) -> Router {
        self.inner
    }
}

/// Handle the CORS preflight request
async fn api_options() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        "Authorization, Content-Type".parse().unwrap(),
    );
    headers.insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        "POST, GET, OPTIONS".parse().unwrap(),
    );
    headers
}
