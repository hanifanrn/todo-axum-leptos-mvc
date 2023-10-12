mod app;
mod features;

use crate::app::routes_all;

pub async fn run() {
    // build out application with a single route
    let app = routes_all();
    // run it with hyper on localhost::3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
