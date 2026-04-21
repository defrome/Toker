mod api_doc;
mod db;
mod errors;
mod handlers;
mod models;
mod services;

use std::{env, net::SocketAddr};

use anyhow::Context;
use axum::{routing::get_service, Router};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::db::client::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let database_url = env::var("DATABASE_URL").context(
        "DATABASE_URL is not set. Use local path like file:marketplace.db or libsql URL",
    )?;

    let db = Db::connect(&database_url).await?;
    db::migrations::run_schema(&db)
        .await
        .map_err(|e| anyhow::anyhow!("failed to run schema migration: {e:?}"))?;

    let state = AppState { db };

    let app = Router::new()
        .merge(handlers::health::routes())
        .merge(handlers::gifts::routes())
        .merge(handlers::users::routes())
        .merge(handlers::orders::routes())
        .route_service("/", get_service(ServeFile::new("static/index.html")))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", api_doc::ApiDoc::openapi()),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:3000".parse().expect("valid socket addr");
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("server started on http://{}", addr);
    info!("swagger docs at http://{}/swagger-ui", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "my_site=debug,tower_http=info".into()),
        )
        .init();
}
