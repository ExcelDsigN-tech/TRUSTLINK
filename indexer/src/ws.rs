use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use futures::StreamExt;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

pub type Tx = broadcast::Sender<String>;

pub fn create_channel() -> Tx {
    let (tx, _) = broadcast::channel(256);
    tx
}

pub fn ws_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(_pool): State<Arc<PgPool>>) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut _tx, mut rx) = socket.split();

    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }

    info!("WebSocket connection closed");
}

pub async fn broadcast_event(tx: &Tx, event_type: &str, payload: &str) {
    let message = format!(r#"{{"type":"{event_type}","data":{payload}}}"#);
    let _ = tx.send(message);
}
