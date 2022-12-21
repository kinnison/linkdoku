/// Models for linkdoku databases
/// equivalent schema available in [crate::schema]
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable)]
pub struct Identity {
    pub id: i32,
    pub oidc_handle: String,
    pub gravatar_hash: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::identity)]
pub struct NewIdentity<'a> {
    pub oidc_handle: &'a str,
    pub gravatar_hash: &'a str,
    pub display_name: &'a str,
}

impl Identity {
    /// Retrieve an identity from the database with the given OIDC handle.
    ///
    /// If there was no identity with the given handle, we return Ok(None)
    /// to signal that nothing went wrong, but there was no such identity.
    pub async fn from_handle(
        conn: &mut AsyncPgConnection,
        handle: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::identity::dsl::*;
        identity
            .filter(oidc_handle.eq(handle))
            .first(conn)
            .await
            .optional()
    }

    /// Create an identity, inserting it into the database, will error if the
    /// given OIDC handle already exists
    pub async fn create(
        conn: &mut AsyncPgConnection,
        oidc_handle: &str,
        gravatar_hash: &str,
        display_name: &str,
    ) -> QueryResult<Self> {
        use crate::schema::identity;
        let new = NewIdentity {
            oidc_handle,
            gravatar_hash,
            display_name,
        };
        diesel::insert_into(identity::table)
            .values(&new)
            .get_result(conn)
            .await
    }
}
