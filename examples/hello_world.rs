use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

mod handlers;
mod views;

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(handlers::get_form))
        .route("/submit", post(handlers::handle_submit));

    // Set up the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
