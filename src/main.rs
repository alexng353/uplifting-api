use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

mod v1;

#[derive(OpenApi)]
#[openapi(
    tags(
        // (name = CUSTOMER_TAG, description = "Customer API endpoints"),
        // (name = ORDER_TAG, description = "Order API endpoints")
    )
)]
struct ApiDoc;

/// Get health of the API.
#[utoipa::path(
    method(get, head),
    path = "/.well-known/health-check",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health_check() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(health_check))
        // .nest("/api/customer", customer::router())
        // .nest("/api/order", order::router())
        // .routes(routes!(
        //     inner::secret_handlers::get_secret,
        //     inner::secret_handlers::post_secret
        // ))
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    println!("Listening on http://localhost:8080");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
