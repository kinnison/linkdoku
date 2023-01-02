/// Models for linkdoku databases
/// equivalent schema available in [crate::schema]
///
pub mod sql_types;

pub use self::sql_types::Visibility;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Identity {
    pub uuid: String,
    pub oidc_handle: String,
    pub display_name: String,
    pub gravatar_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::identity)]
pub struct NewIdentity<'a> {
    pub uuid: &'a str,
    pub oidc_handle: &'a str,
    pub display_name: &'a str,
    pub gravatar_hash: &'a str,
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

    /// Retrieve the roles for this identity
    pub async fn roles(&self, conn: &mut AsyncPgConnection) -> QueryResult<Vec<Role>> {
        Role::by_owner(conn, &self.uuid).await
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

    /// Are we permitted to edit this role?
    ///
    pub async fn can_modify(
        &self,
        _conn: &mut AsyncPgConnection,
        actor: &str,
    ) -> QueryResult<bool> {
        // TODO: When we add role permissions check them here, for now owner can edit
        Ok(self.owner == actor)
    }

    /// Save this role
    pub async fn save(&self, conn: &mut AsyncPgConnection) -> QueryResult<()> {
        use crate::schema::role;
        diesel::update(role::table)
            .filter(role::uuid.eq(&self.uuid))
            .set((
                role::owner.eq(&self.owner),
                role::display_name.eq(&self.display_name),
                role::description.eq(&self.description),
            ))
            .execute(conn)
            .await
            .map(|_| ())
    }

    /// Retrieve the puzzles which are published to this role
    pub async fn published_puzzles(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<Puzzle>> {
        use crate::schema::puzzle::dsl::*;
        puzzle
            .filter(owner.eq(&self.uuid))
            .filter(visibility.eq(Visibility::Published))
            .order_by(created_at.desc())
            .load(conn)
            .await
    }
}

#[derive(Debug, Queryable)]
pub struct Puzzle {
    pub uuid: String,
    pub owner: String,
    pub display_name: String,
    pub short_name: String,
    pub visibility: Visibility,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::puzzle)]
pub struct NewPuzzle<'a> {
    pub uuid: &'a str,
    pub owner: &'a str,
    pub display_name: &'a str,
    pub short_name: &'a str,
    pub visibility: Visibility,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Puzzle {
    pub async fn by_uuid(
        conn: &mut AsyncPgConnection,
        puzzle_uuid: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::puzzle::dsl::*;
        puzzle
            .filter(uuid.eq(puzzle_uuid))
            .first(conn)
            .await
            .optional()
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        uuid: &str,
        owner: &str,
        display_name: &str,
        short_name: &str,
        visibility: Visibility,
    ) -> QueryResult<Self> {
        use crate::schema::puzzle;
        let new = NewPuzzle {
            uuid,
            owner,
            display_name,
            short_name,
            visibility,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };
        diesel::insert_into(puzzle::table)
            .values(&new)
            .get_result(conn)
            .await
    }

    pub async fn can_be_seen(
        &self,
        _conn: &mut AsyncPgConnection,
        user: Option<&str>,
    ) -> QueryResult<bool> {
        match self.visibility {
            Visibility::Restricted => Ok(user.map(|u| u == self.owner).unwrap_or(false)),
            _ => Ok(true),
        }
    }
}

#[derive(Queryable)]
pub struct PuzzleState {
    pub id: i32,
    pub puzzle: String,
    pub description: String,
    pub visibility: Visibility,
    pub updated_at: OffsetDateTime,
    pub data: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::puzzle_state)]
pub struct NewPuzzleState<'a> {
    pub puzzle: &'a str,
    pub description: &'a str,
    pub visibility: Visibility,
    pub updated_at: OffsetDateTime,
    pub data: String,
}

impl Puzzle {
    pub async fn all_states(&self, conn: &mut AsyncPgConnection) -> QueryResult<Vec<PuzzleState>> {
        use crate::schema::puzzle_state::dsl::*;
        puzzle_state
            .filter(puzzle.eq(&self.uuid))
            .order_by(id.asc())
            .load(conn)
            .await
    }

    pub async fn add_state(
        &self,
        conn: &mut AsyncPgConnection,
        description: &str,
        visibility: Visibility,
        data: String,
    ) -> QueryResult<PuzzleState> {
        use crate::schema::puzzle_state;
        let new = NewPuzzleState {
            puzzle: &self.uuid,
            description,
            visibility,
            data,
            updated_at: OffsetDateTime::now_utc(),
        };
        diesel::insert_into(puzzle_state::table)
            .values(&new)
            .get_result(conn)
            .await
    }
}

impl PuzzleState {
    pub async fn can_be_seen(
        &self,
        _conn: &mut AsyncPgConnection,
        puzzle: &Puzzle,
        user: Option<&str>,
    ) -> QueryResult<bool> {
        match self.visibility {
            Visibility::Restricted => Ok(user.map(|u| u == puzzle.owner).unwrap_or(false)),
            _ => Ok(true),
        }
    }
}
