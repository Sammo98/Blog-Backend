// -- MODULES -----------------------------------------------------------------------------------
pub mod db;
pub mod blog;
pub mod app_error;
pub mod handlers;
pub mod utils;

use handlers::*;

// -- IMPORTS  ---------------------------------------------------------------------------------
use axum::{
    routing::{get, post, delete},
    Router
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

// -- MAIN -------------------------------------------------------------------------------------

#[tokio::main]
async fn main() {

    // Initiate Tracing
    utils::tracing_init();

    // Initiate Surrealdb file-based instance 
    let database = db::SDB::init().await;

    // Create router instance with stated routes, database state
    let app = Router::new()
        .route("/", get(health_check))

        .route("/blog", post(create_blogs))

        .route("/blog/:id", get(get_blog))

        .route("/blog", delete(delete_blogs))

        .route("/blog/metadata", get(get_all_metadata))

        .with_state(database)

        .layer(TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        );

    // Run server on port 5000
    axum::Server::bind(&"[::]:8080".parse().expect("Failed to parse IP Address"))
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

