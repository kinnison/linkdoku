//! Linkdoku main code
//!
//!

use std::convert::identity;

use axum::{routing::get, Router};
use clap::Parser;
use git_testament::git_testament;
use sentry::{integrations::tower::*, IntoDsn};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::state::BackendState;

mod api;
mod cli;
mod config;
mod login;
mod redirectors;
mod spa;
mod state;

git_testament!(VERSION);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    // Detect if we're running inside the scaleway cloud, if so, we want a simpler logging format
    let fmt_layer = if std::env::var("SCW_PUBLIC_KEY").is_ok() {
        tracing_subscriber::fmt::layer()
            .without_time()
            .with_ansi(false)
            .boxed()
    } else {
        tracing_subscriber::fmt::layer().boxed()
    };

    tracing_subscriber::Registry::default()
        .with(filter_layer)
        .with(fmt_layer)
        .with(sentry::integrations::tracing::layer())
        .init();

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

    let _guard = sentry::init(sentry::ClientOptions {
        dsn: config
            .sentry_dsn
            .as_deref()
            .map(IntoDsn::into_dsn)
            .and_then(Result::ok)
            .and_then(identity),
        release: Some(VERSION.commit.to_string().into()),
        traces_sample_rate: 1.0,
        environment: config.sentry_env.clone().map(|s| s.into()),
        ..Default::default()
    });

    // Request migrations
    #[cfg(feature = "migrations")]
    {
        info!("Applying pending database migrations...");
        database::apply_migrations_sync(config.database_url.as_str())?;
    }

    // Now prepare context/state we need to get going
    info!("Construct openid-connect providers");
    let providers = login::load_providers(&config).await?;

    info!("Establish database pool");
    let pool = database::create_pool(config.database_url.as_str()).await?;

    // Build the app router
    let app = Router::new()
        .nest("/api", api::router())
        .nest("/", redirectors::router())
        .route("/assets/:filename", get(spa::serve_file))
        .fallback(spa::spa_handler)
        .layer(CookieManagerLayer::new())
        .layer({
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Millis),
                        ),
                )
                .layer(CompressionLayer::new())
                .layer(NewSentryLayer::new_from_top())
                .layer(SentryHttpLayer::with_transaction())
        });

    // and provide all the state to it
    let port = config.port;
    let app = app.with_state(BackendState::new(cli, config, pool, providers));

    info!("Launching server on port {port}");
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
