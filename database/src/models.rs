/// Models for linkdoku databases
/// equivalent schema available in [crate::schema]
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Queryable)]
pub struct Role {
    pub id: i32,
    pub owner: i32,
    pub display_name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::role)]
pub struct NewRole<'a> {
    pub owner: i32,
    pub display_name: &'a str,
    pub description: &'a str,
}

impl Role {
    /// Retrieve a role by id
    pub async fn by_id(conn: &mut AsyncPgConnection, role_id: i32) -> QueryResult<Option<Role>> {
        use crate::schema::role::dsl::*;
        role.filter(id.eq(role_id)).first(conn).await.optional()
    }

    /// Retrieve roles owned by a given identity
    pub async fn by_owner(conn: &mut AsyncPgConnection, owner_id: i32) -> QueryResult<Vec<Role>> {
        use crate::schema::role::dsl::*;
        role.filter(owner.eq(owner_id)).get_results(conn).await
    }

    /// Create a new role
    pub async fn create(
        conn: &mut AsyncPgConnection,
        owner: i32,
        display_name: &str,
        description: &str,
    ) -> QueryResult<Role> {
        use crate::schema::role;
        let new = NewRole {
            owner,
            display_name,
            description,
        };
        diesel::insert_into(role::table)
            .values(&new)
            .get_result(conn)
            .await
    }
}
