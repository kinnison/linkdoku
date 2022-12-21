//! Linkdoku Database
//!
//! This crate represents the linkdoku database interface.
//! In general everything here assumes we're using diesel, postgresql,
//! and async connections via diesel_async.
//!
//! However, for migrations, we *MUST* run sync currently since we do
//! not get an async implementation of migration running :(

use diesel::{backend::Backend, migration::Result};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn apply_migrations_sync<DB>(conn: &mut impl MigrationHarness<DB>) -> Result<()>
where
    DB: Backend,
{
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub mod models;
pub mod schema;
