use crate::features::hello_world::hello_world_controller;

use axum::Router;
use tokio::net::TcpListener;

pub fn routes_all() -> Router {
    return Router::new().merge(hello_world_controller::create_routes());
}

pub async fn run() {
    // build out application with a single route
    let app = routes_all();
    // run it with hyper on localhost::3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
