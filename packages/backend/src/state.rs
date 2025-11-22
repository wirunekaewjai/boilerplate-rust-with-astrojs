use std::sync::Arc;

use crate::{config::AppConfig, helpers::view_renderer::ViewRenderer};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub view: ViewRenderer,
}

impl AppState {
    pub async fn init(config: &AppConfig) -> SharedState {
        Arc::new(Self {
            view: ViewRenderer::new(config),
        })
    }
}
