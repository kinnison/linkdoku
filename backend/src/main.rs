//! Linkdoku main code
//!
//!

use axum::Router;
use clap::Parser;
use git_testament::git_testament;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

use crate::state::BackendState;

mod api;
mod cli;
mod config;
mod login;
mod spa;
mod state;

git_testament!(VERSION);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Detect if we're running inside the scaleway cloud, if so, we want a simpler logging format
    if std::env::var("SCW_PUBLIC_KEY").is_ok() {
        tracing_subscriber::fmt()
            .without_time()
            .with_ansi(false)
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    info!("Starting up Linkdoku {VERSION}");

    let cli = cli::Cli::parse();
    cli.show();

    if let Ok(port) = std::env::var("PORT") {
        info!("Overriding port from environment with {port}");
        std::env::set_var("LINKDOKU__PORT", port);
    }

    if let Some(port) = cli.port.as_ref() {
        info!("Overriding port from CLI with {port}");
        std::env::set_var("LINKDOKU__PORT", format!("{port}"));
    }

    let config = config::load_configuration(&cli).expect("Unable to load configuration");
    config.show();

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
    let app = app.with_state(BackendState::new(cli, config, pool, providers));

    info!("Launching server on port {port}");
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
