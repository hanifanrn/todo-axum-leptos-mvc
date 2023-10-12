use super::hello_world_service::{hello_world, mirror_body_json, mirror_body_string};

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
