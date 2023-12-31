use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    return Router::new()
        .route("/", get(hello_world))
        .route("/mirror-body-string", get(mirror_body_string))
        .route("/mirror-body-json", post(mirror_body_json));
}

pub async fn hello_world() -> String {
    return "hello world from router".to_owned();
}

pub async fn mirror_body_string(body: String) -> String {
    return body;
}

pub async fn mirror_body_json() -> () {}
