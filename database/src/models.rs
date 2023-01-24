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

    /// Update this identity with new data
    pub async fn update(
        &self,
        conn: &mut AsyncPgConnection,
        gravatar_hash: &str,
        display_name: &str,
    ) -> QueryResult<Self> {
        use crate::schema::identity::dsl;
        diesel::update(dsl::identity.find(&self.uuid))
            .set((
                dsl::gravatar_hash.eq(gravatar_hash),
                dsl::display_name.eq(display_name),
            ))
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
    pub short_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::role)]
pub struct NewRole<'a> {
    pub uuid: &'a str,
    pub owner: &'a str,
    pub display_name: &'a str,
    pub description: &'a str,
    pub short_name: &'a str,
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

    /// Retrieve a role by short name
    pub async fn by_short_name(
        conn: &mut AsyncPgConnection,
        role_name: &str,
    ) -> QueryResult<Option<Role>> {
        use crate::schema::role::dsl::*;
        role.filter(short_name.eq(role_name))
            .first(conn)
            .await
            .optional()
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
        short_name: &str,
    ) -> QueryResult<Role> {
        use crate::schema::role;
        let new = NewRole {
            uuid,
            owner,
            display_name,
            description,
            short_name,
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
                role::short_name.eq(&self.short_name),
                role::display_name.eq(&self.display_name),
                role::description.eq(&self.description),
            ))
            .execute(conn)
            .await
            .map(|_| ())
    }

    /// Retrieve the puzzles which are published to this role
    pub async fn visible_puzzles(
        &self,
        conn: &mut AsyncPgConnection,
        user: Option<&str>,
    ) -> QueryResult<Vec<Puzzle>> {
        let can_see_unpublished = if let Some(actor) = user {
            self.can_modify(conn, actor).await?
        } else {
            false
        };
        use crate::schema::puzzle::dsl::*;
        if can_see_unpublished {
            // We're logged in as someone who can edit this role, so all puzzles
            puzzle
                .filter(owner.eq(&self.uuid))
                .order_by(created_at.desc())
                .load(conn)
                .await
        } else {
            // Only published puzzles because the user is not logged in
            puzzle
                .filter(owner.eq(&self.uuid))
                .filter(visibility.eq(Visibility::Published))
                .order_by(created_at.desc())
                .load(conn)
                .await
        }
    }

    /// This role's short name is available if either no other role has it,
    /// or the name is unchanged.
    pub async fn short_name_available(&self, conn: &mut AsyncPgConnection) -> QueryResult<bool> {
        use crate::schema::role::dsl::*;
        let count: i64 = role
            .filter(short_name.eq(&self.short_name))
            .filter(uuid.ne(&self.uuid))
            .count()
            .get_result(conn)
            .await?;
        Ok(count == 0)
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

    pub async fn by_short_name(
        conn: &mut AsyncPgConnection,
        owning_role: &str,
        puzzle_short_name: &str,
    ) -> QueryResult<Option<Self>> {
        use crate::schema::puzzle::dsl::*;
        puzzle
            .filter(owner.eq(owning_role).and(short_name.eq(puzzle_short_name)))
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
        conn: &mut AsyncPgConnection,
        user: Option<&str>,
    ) -> QueryResult<bool> {
        match self.visibility {
            Visibility::Restricted => {
                if let Some(user) = user {
                    let roles = Role::by_owner(conn, user).await?;
                    Ok(roles.into_iter().any(|role| role.uuid == self.owner))
                } else {
                    Ok(false)
                }
            }
            _ => Ok(true),
        }
    }

    pub async fn update_metadata(
        &self,
        conn: &mut AsyncPgConnection,
        short_name: &str,
        display_name: &str,
    ) -> QueryResult<Self> {
        use crate::schema::puzzle::dsl;
        diesel::update(dsl::puzzle.find(&self.uuid))
            .set((
                dsl::short_name.eq(short_name),
                dsl::display_name.eq(display_name),
            ))
            .get_result(conn)
            .await
    }

    pub async fn can_edit(&self, conn: &mut AsyncPgConnection, user: &str) -> QueryResult<bool> {
        // We're permitted to edit this puzzle *iff* the given user has access to the owning role
        let user = match Identity::from_uuid(conn, user).await? {
            Some(id) => id,
            None => return Ok(false),
        };

        if user
            .roles(conn)
            .await?
            .iter()
            .any(|role| role.uuid == self.owner)
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn set_visibility(
        &self,
        conn: &mut AsyncPgConnection,
        visibility: Visibility,
    ) -> QueryResult<Self> {
        use crate::schema::puzzle::dsl;
        diesel::update(dsl::puzzle.find(&self.uuid))
            .set((
                dsl::visibility.eq(visibility),
                dsl::updated_at.eq(OffsetDateTime::now_utc()),
            ))
            .get_result(conn)
            .await
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
    pub uuid: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::puzzle_state)]
pub struct NewPuzzleState<'a> {
    pub puzzle: &'a str,
    pub description: &'a str,
    pub visibility: Visibility,
    pub updated_at: OffsetDateTime,
    pub data: &'a str,
    pub uuid: &'a str,
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
        data: &str,
    ) -> QueryResult<PuzzleState> {
        use crate::schema::puzzle_state;
        let state_uuid = utils::random_uuid_within("puzzle_state", &self.uuid);
        let new = NewPuzzleState {
            puzzle: &self.uuid,
            description,
            visibility,
            data,
            updated_at: OffsetDateTime::now_utc(),
            uuid: &state_uuid,
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
        conn: &mut AsyncPgConnection,
        puzzle: &Puzzle,
        user: Option<&str>,
    ) -> QueryResult<bool> {
        assert_eq!(
            self.puzzle, puzzle.uuid,
            "Puzzle does not match puzzle state?"
        );
        match self.visibility {
            Visibility::Restricted => {
                if let Some(user) = user {
                    let roles = Role::by_owner(conn, user).await?;
                    Ok(roles.into_iter().any(|role| role.uuid == puzzle.owner))
                } else {
                    Ok(false)
                }
            }
            _ => Ok(true),
        }
    }

    pub async fn by_uuid(conn: &mut AsyncPgConnection, uuid: &str) -> QueryResult<Option<Self>> {
        use crate::schema::puzzle_state::dsl;

        dsl::puzzle_state
            .filter(dsl::uuid.eq(uuid))
            .first(conn)
            .await
            .optional()
    }

    pub async fn update(
        &self,
        conn: &mut AsyncPgConnection,
        description: &str,
        data: &str,
    ) -> QueryResult<()> {
        use crate::schema::puzzle_state::dsl;

        diesel::update(dsl::puzzle_state)
            .filter(dsl::id.eq(self.id))
            .set((
                dsl::description.eq(description),
                dsl::data.eq(data),
                dsl::updated_at.eq(OffsetDateTime::now_utc()),
            ))
            .execute(conn)
            .await
            .map(|_| ())
    }

    pub async fn set_visibility(
        &self,
        conn: &mut AsyncPgConnection,
        visibility: Visibility,
    ) -> QueryResult<()> {
        use crate::schema::puzzle_state::dsl;

        diesel::update(dsl::puzzle_state)
            .filter(dsl::id.eq(self.id))
            .set((
                dsl::visibility.eq(visibility),
                dsl::updated_at.eq(OffsetDateTime::now_utc()),
            ))
            .execute(conn)
            .await
            .map(|_| ())
    }
}

#[derive(Queryable)]
pub struct Tag {
    pub uuid: String,
    pub name: String,
    pub colour: String,
    pub black_text: bool,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::tag)]
pub struct NewTag<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub colour: &'a str,
    pub black_text: bool,
}

impl Tag {
    pub async fn by_uuid(conn: &mut AsyncPgConnection, uuid: &str) -> QueryResult<Option<Self>> {
        use crate::schema::tag::dsl;

        dsl::tag.find(uuid).first(conn).await.optional()
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        name: &str,
        colour: &str,
        black_text: bool,
    ) -> QueryResult<Self> {
        use crate::schema::tag::dsl;
        let new_uuid = utils::random_uuid("tag");

        let newtag = NewTag {
            uuid: &new_uuid,
            name,
            colour,
            black_text,
        };

        diesel::insert_into(dsl::tag)
            .values(newtag)
            .get_result(conn)
            .await
    }

    pub async fn get_all(
        conn: &mut AsyncPgConnection,
        prefix: Option<&str>,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema::tag::dsl;

        if let Some(prefix) = prefix {
            dsl::tag
                .filter(dsl::name.like(format!("{prefix}%")))
                .get_results(conn)
                .await
        } else {
            dsl::tag.get_results(conn).await
        }
    }
}
