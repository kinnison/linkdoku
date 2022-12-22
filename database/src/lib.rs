//! Linkdoku Database
//!
//! This crate represents the linkdoku database interface.
//! In general everything here assumes we're using diesel, postgresql,
//! and async connections via diesel_async.
//!
//! However, for migrations, we *MUST* run sync currently since we do
//! not get an async implementation of migration running :(

use diesel_async::{
    pooled_connection::{bb8, AsyncDieselConnectionManager, PoolError},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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

pub async fn create_pool(db_url: &str) -> Result<Pool, PoolError> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    bb8::Pool::builder().build(config).await
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

pub use axum_link::Connection;
