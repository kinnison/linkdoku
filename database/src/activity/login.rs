//! Login activities
//!
//! When logging in, various things have to happen, this activity is meant to provide for that.

use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::models;

/// Log into linkdoku.  This is called at the end of a successful openid-connect cycle and provides
/// the caller with a database backed Identity etc.
/// The check/create is done in a transation, if any of that fails, you'll get an error back
#[tracing::instrument(skip_all)]
pub async fn login_upsert(
    conn: &mut AsyncPgConnection,
    oidc_handle: &str,
    gravatar_hash: &str,
    display_name: &str,
) -> QueryResult<(models::Identity, Vec<models::Role>, bool)> {
    conn.build_transaction()
        .run(|conn| {
            Box::pin(async move {
                let (new, mut identity) = match models::Identity::from_handle(conn, oidc_handle)
                    .await?
                {
                    Some(identity) => (false, identity),
                    None => (
                        true,
                        models::Identity::create(conn, oidc_handle, gravatar_hash, display_name)
                            .await?,
                    ),
                };
                if !new
                    && (identity.gravatar_hash != gravatar_hash
                        || identity.display_name != display_name)
                {
                    identity = identity.update(conn, gravatar_hash, display_name).await?;
                }
                let roles = if new {
                    vec![
                        models::Role::create(
                            conn,
                            &identity.default_role_uuid(),
                            &identity.uuid,
                            &format!("{display_name} (Role)"),
                            &format!("# Initial role for {display_name}"),
                            &identity.uuid,
                        )
                        .await?,
                    ]
                } else {
                    models::Role::by_owner(conn, &identity.uuid).await?
                };
                Ok((identity, roles, new))
            })
        })
        .await
}
