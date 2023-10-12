use crate::features::hello_world::hello_world_router;

use axum::Router;

pub fn routes_all() -> Router {
    return Router::new().merge(hello_world_router::create_routes());
}
