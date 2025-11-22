mod config;
mod handlers;
mod helpers;
mod state;

use std::{io, path::PathBuf};

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

use crate::{
    config::AppConfig,
    handlers::{render_home::render_home, serve_asset::serve_asset},
    state::AppState,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load();
    let router = create_router(&config).await;

    serve(&config, router).await
}

async fn create_router(config: &AppConfig) -> Router {
    let state = AppState::init(&config).await;

    #[rustfmt::skip]
    let router = Router::new()
        .route("/", get(render_home))
        .with_state(state);

    if config.dev_mode {
        use axum_reverse_proxy::ReverseProxy;
        use tower_livereload::LiveReloadLayer;

        let proxy = ReverseProxy::new("/", &config.frontend_url);

        return Router::new()
            .merge(router)
            .fallback_service(proxy)
            .layer(LiveReloadLayer::new())
            .layer(TraceLayer::new_for_http());
    }

    let assets_dir = PathBuf::from(&config.assets_dir);
    let asset_router = Router::new()
        .route("/{filename}", get(serve_asset))
        .with_state(assets_dir);

    Router::new()
        .nest(&config.assets_prefix, asset_router)
        .merge(router)
        .fallback_service(ServeDir::new(&config.public_dir))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn serve(config: &AppConfig, router: Router) -> io::Result<()> {
    let shutdown_signal = async || {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            use tokio::signal::unix::{SignalKind, signal};

            signal(SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        tracing::info!("application shutting down...");
    };

    let listen_addr = format!("{}:{}", &config.server_hostname, config.server_port);
    let listener = TcpListener::bind(&listen_addr).await?;

    tracing::info!("application started (http://{listen_addr})");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
}
