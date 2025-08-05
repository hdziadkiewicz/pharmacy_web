use askama::Template;
use axum::{Router, extract::State, response::Html, routing::get};
use sqlx::SqlitePool;

use crate::db;
use crate::models::medicine::Medicine;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

// Szablon HTML wykorzystujący Askama
#[derive(Template)]
#[template(path = "medicines.html")] // musisz mieć templates/medicines.html
struct MedicinesTemplate {
    medicines: Vec<Medicine>,
}

// Router dla modułu "medicines"
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root)) // strona główna
        .route("/medicines", get(show_medicines)) // lista leków
}

// Strona główna
async fn root() -> Html<&'static str> {
    Html(
        r#"
        <h1>Strona główna</h1>
        <p><a href="/medicines">Zobacz listę leków</a></p>
    "#,
    )
}

// Widok listy leków
async fn show_medicines(State(state): State<AppState>) -> Html<String> {
    let medicines = db::get_medicines(&state.db).await.unwrap_or_default();
    let tpl = MedicinesTemplate { medicines };

    Html(
        tpl.render()
            .unwrap_or_else(|_| "<p>Błąd renderowania szablonu</p>".to_string()),
    )
}
