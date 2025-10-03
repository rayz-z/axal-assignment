use axum::{Router, routing::get, extract::State, Json, };
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt},
    net::{TcpListener},
};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    tvl: Arc<Mutex<Vec<TVLResponse>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TVLResponse {
    tvl: Value,
    warning: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        tvl: Arc::new(Mutex::new(Vec::new())),
    };

    // Axum server
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>()?)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(get_tvl))
        .with_state(state.clone())
        .layer(cors);

    let http_listener = TcpListener::bind("0.0.0.0:3000").await?;

    // Socket listener
    let socket_state = state.clone();
    tokio::spawn(async move {
        let socket_listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

        loop {
            let (mut stream, _) = socket_listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            if stream.read_to_end(&mut buffer).await.is_ok() {
                if let Ok(data) = String::from_utf8(buffer) {
                    if let Ok(msg) = serde_json::from_str::<TVLResponse>(&data) {
                        println!("Received TVL message: {:#?}", msg);
                        let mut lock = socket_state.tvl.lock().unwrap();
                        lock.push(msg);
                    }
                }
            }
        }
    });

    axum::serve(http_listener, app).await?;

    Ok(())
}

async fn get_tvl(State(state): State<AppState>) -> Json<Vec<TVLResponse>> {
    let lock = state.tvl.lock().unwrap();
    let tvl_arr_len = lock.len();

    // send tvls from last 10 minutes at most
    if tvl_arr_len > 600 {
        return Json(lock[tvl_arr_len - 600..].to_vec())
    }
    Json(lock.clone())
}
