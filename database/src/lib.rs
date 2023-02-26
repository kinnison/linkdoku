//! Linkdoku Database
//!
//! This crate represents the linkdoku database interface.
//! In general everything here assumes we're using diesel, postgresql,
//! and async connections via diesel_async.
//!
//! However, for migrations, we *MUST* run sync currently since we do
//! not get an async implementation of migration running :(

use std::{fmt::Display, time::Duration};

use ::bb8::ErrorSink;
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager, PoolError},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub use axum_link::Connection;
use futures::{future::BoxFuture, FutureExt};
use lazy_static::lazy_static;
use rustls::RootCertStore;
use tokio_postgres_rustls::MakeRustlsConnect;
use tracing::error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tracing::instrument(skip(db_url))]
pub fn apply_migrations_sync(db_url: &str) -> diesel::migration::Result<()> {
    use diesel::{Connection, PgConnection};
    let mut conn = PgConnection::establish(db_url)?;
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub(crate) mod utils;

pub mod activity;
pub mod models;
pub mod schema;

// Helper functions

pub type Pool = bb8::Pool<AsyncPgConnection>;

lazy_static! {
    static ref MAKE_TLS_CONNECT: MakeRustlsConnect = {
        let mut store = RootCertStore::empty();
        store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        MakeRustlsConnect::new(
            rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(store)
                .with_no_client_auth(),
        )
    };
}

fn establish_connection(url: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    (async {
        let (client, connection) = tokio_postgres::connect(url, MAKE_TLS_CONNECT.clone())
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    })
    .boxed()
}

#[derive(Debug)]
struct MyErrorSink;

impl<E> ErrorSink<E> for MyErrorSink
where
    E: Display,
{
    fn sink(&self, err: E) {
        error!("BB8 pool error: {err}");
    }

    fn boxed_clone(&self) -> Box<dyn ErrorSink<E>> {
        Box::new(MyErrorSink)
    }
}

pub async fn create_pool(db_url: &str) -> Result<Pool, PoolError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(
        db_url,
        establish_connection,
    );
    bb8::Pool::builder()
        .idle_timeout(Duration::from_secs(30).into())
        .connection_timeout(Duration::from_secs(10))
        .error_sink(Box::new(MyErrorSink))
        .build(config)
        .await
}

pub async fn create_connection(db_url: &str) -> ConnectionResult<AsyncPgConnection> {
    establish_connection(db_url).await
}

pub mod axum_link {
    use std::ops::{Deref, DerefMut};

    use async_trait::async_trait;
    use axum::{
        extract::{FromRef, FromRequestParts},
        http::{request::Parts, StatusCode},
    };
    use diesel_async::{pooled_connection::bb8, AsyncPgConnection};

    use super::Pool;

    pub struct Connection(bb8::PooledConnection<'static, AsyncPgConnection>);

    #[async_trait]
    impl<S> FromRequestParts<S> for Connection
    where
        Pool: FromRef<S>,
        S: Send + Sync,
    {
        type Rejection = (StatusCode, String);

        #[tracing::instrument(name = "acquire_connection", skip_all)]
        async fn from_request_parts(
            _parts: &mut Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            let pool = Pool::from_ref(state);
            let conn = pool
                .get_owned()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(Self(conn))
        }
    }

    impl Deref for Connection {
        type Target = AsyncPgConnection;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Connection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
