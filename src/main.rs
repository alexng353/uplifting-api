use std::{net::Ipv4Addr, sync::Arc, time::Duration};
use hmac::{Hmac, Mac};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

// tags
use v1::{auth::AUTH_TAG, exercises::EXERCISES_TAG, friends::FRIENDS_TAG, muscles::MUSCLES_TAG, sets::SETS_TAG, sync::SYNC_TAG, users::USERS_TAG, workouts::WORKOUTS_TAG};

mod v1;
mod structs;
mod state;
mod db;
mod error;
mod util;
mod extractors;

pub(crate) use anyhow::Context;
pub(crate) use axum::extract::{Json, State};
pub(crate) use error::{AppError, Errors};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use state::AppState;
pub(crate) use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = AUTH_TAG, description = "Authentication API endpoints"),
        (name = EXERCISES_TAG, description = "Exercise API endpoints"),
        (name = FRIENDS_TAG, description = "Friends API endpoints"),
        (name = MUSCLES_TAG, description = "Muscle API endpoints"),
        (name = SETS_TAG, description = "Set API endpoints"),
        (name = SYNC_TAG, description = "Sync API endpoints"),
        (name = USERS_TAG, description = "Users API endpoints"),
        (name = WORKOUTS_TAG, description = "Workout API endpoints"),
    ),
)]
struct ApiDoc;

/// Get health of the API.
#[utoipa::path(
    get,
    path = "/.well-known/health-check",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health_check() -> &'static str {
    "ok"
}

#[utoipa::path(
    get, 
    path = "/",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn index() -> &'static str {
    "ok 200"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse()?;
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let db = db::db().await?;
    let state = state::AppState { db: Arc::new(db),
    jwt_key: Hmac::new_from_slice(jwt_secret.as_bytes()).context("Failed to create HMAC")? 
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_headers(Any);

    let trace = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_response(|response: &axum::http::Response<_>, latency: Duration, _span: &tracing::Span| {
            tracing::info!(
                status = %response.status(),
                latency = ?latency,
                "response"
            );
        });

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health_check))
        .routes(routes!(index))
        .with_state(state.clone())
        .nest("/api/v1", v1::router(state.clone()))
        .layer(cors)
        .layer(trace)
        .split_for_parts();

    tokio::fs::write("openapi.json", api.to_pretty_json()?).await?;

    let router = router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", api));

    tracing::info!("Listening on http://0.0.0.0:{port}");
    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
