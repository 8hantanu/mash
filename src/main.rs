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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
