use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tera::{Context, Tera};

// Define a form data structure using Serde
#[derive(Deserialize)]
struct NameForm {
    name: String,
}

// Initialize Tera for template rendering
async fn render_form(tera: Tera) -> impl IntoResponse {
    let ctx = Context::new();
    let rendered = tera.render("form.html", &ctx).unwrap();
    Html(rendered)
}

// Handle form submission using HTMX
async fn handle_submit(Form(form): Form<NameForm>) -> impl IntoResponse {
    let response = format!("<p>Hello, {}!</p>", form.name);
    Html(response)
}

#[tokio::main]
async fn main() {
    // Initialize Tera
    let tera = Tera::new("templates/**/*").unwrap();

    // Define routes
    let app = Router::new()
        .route("/", get(move || render_form(tera.clone())))
        .route("/submit", post(handle_submit));

    // Set up the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

