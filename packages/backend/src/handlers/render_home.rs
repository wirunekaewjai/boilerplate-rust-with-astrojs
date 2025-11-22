use axum::{extract::State, response::Html};
use minijinja::context;
use serde::Serialize;

use crate::state::SharedState;

pub async fn render_home(State(state): State<SharedState>) -> Html<String> {
    let products = [
        Product {
            name: "Product 1".to_string(),
            price: 10.99,
        },
        Product {
            name: "Product 2".to_string(),
            price: 19.99,
        },
    ];

    let ctx = context! {
        products,
    };

    let html = state.view.render("home.html", ctx).await;

    Html(html)
}

#[derive(Serialize)]
pub struct Product {
    pub name: String,
    pub price: f64,
}
