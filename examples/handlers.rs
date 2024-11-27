use axum::{
    extract::Form,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use crate::views::{render_form, render_response};

// Define a form data structure using Serde
#[derive(Deserialize)]
pub struct NameForm {
    pub name: String,
}

// Handle GET request to render the form
pub async fn get_form() -> impl IntoResponse {
    Html(render_form().into_string())
}

// Handle POST request when form is submitted
pub async fn handle_submit(Form(form): Form<NameForm>) -> impl IntoResponse {
    Html(render_response(&form.name).into_string())
}
