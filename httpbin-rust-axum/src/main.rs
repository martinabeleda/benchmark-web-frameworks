use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let app_state = AppState::default();
    let app = Router::new()
        .route("/get/:key", get(get_handler))
        .route("/post", post(post_handler))
        .layer(Extension(Arc::new(app_state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Default, Clone)]
struct AppState {
    // You can add shared application state here
}

#[derive(Serialize)]
struct GetResponse {
    key: String,
    value: String,
}

async fn get_handler(
    Path(key): Path<String>,
    Extension(_app_state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    // For simplicity, we are returning a static value for any key
    let response = GetResponse {
        key,
        value: "Hello, World!".to_string(),
    };
    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
struct PostRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct PostResponse {
    result: String,
}

async fn post_handler(
    Extension(_app_state): Extension<Arc<AppState>>,
    Json(body): Json<PostRequest>,
) -> impl IntoResponse {
    // For simplicity, we are returning a success message without storing the key-value pair
    let response = PostResponse {
        result: format!("Received key: {}, value: {}", body.key, body.value),
    };
    (StatusCode::OK, Json(response))
}
