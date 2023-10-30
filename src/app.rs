use crate::features::hello_world::hello_world_controller;

use axum::Router;

pub fn routes_all() -> Router {
    return Router::new().merge(hello_world_controller::create_routes());
}

pub async fn run() {
    // build out application with a single route
    let app = routes_all();
    // run it with hyper on localhost::3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
