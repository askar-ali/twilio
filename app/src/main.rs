// Main

use anyhow::Result;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::post,
    Extension, Router,
};
use db::AppState;
use tokio::signal;
use tower_http::{cors::Any, services::ServeDir};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod db;
mod handler;

// type alias
type ExtState = Extension<AppState>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    // Db
    let pool = db::conn().await?;

    let app_host = std::env::var("APP_HOST").unwrap_or("127.0.0.1".to_string());
    let app_port = std::env::var("APP_PORT").unwrap_or("8000".to_string());

    let app_state = AppState { db: pool.into() };

    let app = Router::new()
        .route("/status", post(handler::handle_twilio_webhook_status))
        .route("/receive", post(handler::handle_twilio_webhook_payload))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
        .nest_service("/assets", ServeDir::new("assets"));

    let server_url = format!("{}:{}", app_host, app_port);
    println!("App starting in {}!", server_url);

    //server listening address
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl + C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
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
}
