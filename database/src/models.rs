/// Models for linkdoku databases
/// equivalent schema available in [crate::schema]
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Identity {
    pub uuid: String,
    pub oidc_handle: String,
    pub gravatar_hash: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::identity)]
pub struct NewIdentity<'a> {
    pub uuid: &'a str,
    pub oidc_handle: &'a str,
    pub gravatar_hash: &'a str,
    pub display_name: &'a str,
}

impl Identity {
    /// Compute the UUID for an identity, given its oidc_handle
    pub fn uuid(oidc_handle: &str) -> String {
        utils::uuid("identity", oidc_handle, "fromhandle")
    }

    // Compute the UUID for a given identity's default role
    pub fn default_role_uuid(&self) -> String {
        utils::uuid("role", &self.uuid, "defaultrole")
    }

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

    /// Retrieve an identity from the database with the given UUID
    ///
    /// If there was no identity with the given handle, you get Ok(None)
    pub async fn from_uuid(conn: &mut AsyncPgConnection, uuid: &str) -> QueryResult<Option<Self>> {
        use crate::schema::identity::dsl;
        dsl::identity
            .filter(dsl::uuid.eq(uuid))
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
        let uuid = Self::uuid(oidc_handle);
        let new = NewIdentity {
            uuid: &uuid,
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
    pub uuid: String,
    pub owner: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::role)]
pub struct NewRole<'a> {
    pub uuid: &'a str,
    pub owner: &'a str,
    pub display_name: &'a str,
    pub description: &'a str,
}

impl Role {
    /// Retrieve a role by uuid
    pub async fn by_uuid(
        conn: &mut AsyncPgConnection,
        role_uuid: &str,
    ) -> QueryResult<Option<Role>> {
        use crate::schema::role::dsl::*;
        role.filter(uuid.eq(role_uuid)).first(conn).await.optional()
    }

    /// Retrieve roles owned by a given identity
    pub async fn by_owner(
        conn: &mut AsyncPgConnection,
        owner_uuid: &str,
    ) -> QueryResult<Vec<Role>> {
        use crate::schema::role::dsl::*;
        role.filter(owner.eq(owner_uuid)).get_results(conn).await
    }

    /// Create a new role
    pub async fn create(
        conn: &mut AsyncPgConnection,
        uuid: &str,
        owner: &str,
        display_name: &str,
        description: &str,
    ) -> QueryResult<Role> {
        use crate::schema::role;
        let new = NewRole {
            uuid,
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
