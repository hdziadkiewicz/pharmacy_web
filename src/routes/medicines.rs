use crate::db;
use crate::models::medicine::Medicine;
use askama::Template;
use axum::{Router, extract::State, response::Html, routing::get};
use sqlx::SqlitePool;

pub fn router() -> Router<AppState> {
    Router::new().route("/medicines", get(show_medicines))
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[derive(Template)]
#[template(path = "medicines.html")]
struct MedicinesTemplate {
    medicines: Vec<Medicine>,
}

async fn show_medicines(State(state): State<AppState>) -> Html<String> {
    let medicines = db::get_medicines(&state.db).await.unwrap_or_default();

    let tpl = MedicinesTemplate { medicines };

    Html(tpl.render().unwrap())
}
