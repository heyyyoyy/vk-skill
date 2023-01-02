use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Router, Json,
    extract::State,
};

use std::{net::{SocketAddr, IpAddr}, sync::Arc};
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
use tokio::signal;

use crate::types::request::VKRequest;
use crate::types::response::VKResponse;

#[derive(Debug, Default)]
pub struct VKSkill;

pub struct VKListener {
    vk_skill: Arc<VKSkill>
}


impl VKListener {
    pub fn new() -> Self {
        Self {vk_skill: Arc::new(VKSkill::default())}
    }

    async fn webhook(State(_vk_skill): State<Arc<VKSkill>>, Json(payload): Json<VKRequest>) -> impl IntoResponse {
        tracing::debug!("Request: {payload:?}");

        // Build main response
        let mut resp = VKResponse::from_request(payload).await;

        // Mock user handler  
        resp.response.text = "12345".to_string();

        (StatusCode::OK, Json(resp))
    }

    async fn shutdown_signal(&self) {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };
    
        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };
    
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
    
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    
        tracing::debug!("Signal received, starting graceful shutdown");
    }

    pub async fn run_server<I>(&self, addr: (I, u16))
    where I: Into<IpAddr>
    {
        let app = Router::new()
        .route("/webhook/", post(Self::webhook))
        .with_state(Arc::clone(&self.vk_skill))
        .layer(
            CorsLayer::new()
            .allow_methods([Method::POST, Method::GET, Method::OPTIONS])
            .allow_origin(Any)
            .allow_headers(Any)
        )
        .layer(TraceLayer::new_for_http());

        let addr = SocketAddr::from(addr);
        tracing::debug!("Listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(self.shutdown_signal())
            .await
            .unwrap();
    }
}
