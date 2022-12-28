//! Linkdoku main code
//!
//!

use axum::Router;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

use crate::state::BackendState;

mod api;
mod config;
mod login;
mod spa;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let config = config::load_configuration().expect("Unable to load configuration");
    tracing_subscriber::fmt::init();
    info!("{:#?}", &*config);

    // Request migrations
    info!("Applying pending database migrations...");
    database::apply_migrations_sync(config.database_url.as_str())?;

    // Now prepare context/state we need to get going
    info!("Construct openid-connect providers");
    let providers = login::load_providers(&config).await?;

    info!("Establish database pool");
    let pool = database::create_pool(config.database_url.as_str()).await?;

    // Build the app router
    let app = Router::new()
        .nest("/api", api::router())
        .fallback(spa::spa_handler)
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        );

    // and provide all the state to it
    let port = config.port;
    let app = app.with_state(BackendState::new(config, pool, providers));

    info!("Launching server on port {port}");
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
